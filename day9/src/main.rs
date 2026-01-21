use std::error::Error;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::{BTreeSet, HashMap};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
}

fn solve_max_rectangle(polygon: &[Point]) -> i64 {
    let mut xs = BTreeSet::new();
    let mut ys = BTreeSet::new();
    for p in polygon {
        xs.insert(p.x);
        xs.insert(p.x + 1); // expand the point 
        ys.insert(p.y);
        ys.insert(p.y + 1);
    }
    let xs: Vec<i64> = xs.into_iter().collect();
    let ys: Vec<i64> = ys.into_iter().collect();

    let x_map: HashMap<i64, usize> = xs.iter().enumerate().map(|(i, &v)| (v, i)).collect();
    let y_map: HashMap<i64, usize> = ys.iter().enumerate().map(|(i, &v)| (v, i)).collect();

    let grid_width = xs.len() - 1;
    let grid_height = ys.len() - 1;
    let mut grid = vec![vec![false; grid_width]; grid_height];
    
    // ledger to mark unique vertical segments
    let mut is_vert_wall = vec![vec![false; grid_width]; grid_height];

    for i in 0..polygon.len() {
        let p1 = polygon[i];
        let p2 = polygon[(i + 1) % polygon.len()];

        // map physical coords to grid indices
        let c1 = x_map[&p1.x];
        let r1 = y_map[&p1.y];
        let c2 = x_map[&p2.x];
        let r2 = y_map[&p2.y];

        if p1.x == p2.x {
            let c = c1; // column index for the interval [x, x+1)
            let r_start = r1.min(r2);
            let r_end = r1.max(r2);
            
            for r in r_start..r_end {
                grid[r][c] = true; 
                is_vert_wall[r][c] = true; // mark as a wall for the scanline
            }
        }
        else if p1.y == p2.y {
            let r = r1; // row index for the interval [y, y+1)
            let c_start = c1.min(c2);
            let c_end = c1.max(c2);
            
            for c in c_start..c_end {
                grid[r][c] = true;
            }
        }
    }

    // iterate through every row. toggle "inside" status when we pass a vertical wall.
    for r in 0..grid_height {
        let mut inside = false;
        
        for c in 0..grid_width {
            if is_vert_wall[r][c] {
                // if we are currently on a vertical wall, we are definitely "inside"
                // but we need to know if we are ENTERING or EXITING the shape
                // in a compressed grid with, a wall is a solid block
                // we toggle the state only when we finish passing the wall
                
                // lookahead: if the next column is not a wall, we toggle
                // for edge cases where walls are next to each other 
                let next_is_wall = if c + 1 < grid_width { is_vert_wall[r][c+1] } else { false };
                
                if !next_is_wall {
                    inside = !inside;
                }
            } else {
                // we are in empty space. fill it if we are currently "inside"
                if inside {
                    grid[r][c] = true;
                }
            }
        }
    }


    let mut sat = vec![vec![0i64; grid_width + 1]; grid_height + 1];

    for r in 0..grid_height {
        let row_phys_height = ys[r + 1] - ys[r];
        for c in 0..grid_width {
            let col_phys_width = xs[c + 1] - xs[c];
            
            let cell_area = if grid[r][c] {
                row_phys_height * col_phys_width
            } else {
                0
            };

            sat[r + 1][c + 1] = cell_area 
                + sat[r][c + 1] 
                + sat[r + 1][c] 
                - sat[r][c];
        }
    }


    let mut max_area = 0;
    
    for i in 0..polygon.len() {
        let p1 = polygon[i];
        let r1 = y_map[&p1.y];
        let c1 = x_map[&p1.x];

        for j in (i + 1)..polygon.len() {
            let p2 = polygon[j];
            let r2 = y_map[&p2.y];
            let c2 = x_map[&p2.x];

            // normalize coordinates
            let r_min = r1.min(r2);
            let r_max = r1.max(r2); 
            let c_min = c1.min(c2);
            let c_max = c1.max(c2);

            // calculate expected area using the inclusive outer boundary
            let width = xs[c_max + 1] - xs[c_min];
            let height = ys[r_max + 1] - ys[r_min];
            let expected_area = width * height;

            let actual_area = sat[r_max + 1][c_max + 1]
                - sat[r_min][c_max + 1]
                - sat[r_max + 1][c_min]
                + sat[r_min][c_min];

            if actual_area == expected_area {
                if expected_area > max_area {
                    max_area = expected_area;
                }
            }
        }
    }

    max_area
}

fn get_sorted_unique_coords<I>(iter: I) -> Vec<i64>
where
    I: Iterator<Item = i64>,
{
    let set: BTreeSet<i64> = iter.collect();
    set.into_iter().collect()
}

fn find_area(point_a: (u64, u64), point_b: (u64, u64)) -> u64 {
    (point_a.0.abs_diff(point_b.0) + 1) * (point_a.1.abs_diff(point_b.1) + 1)
}

fn solution1(data: Vec<(u64, u64)>) -> u64 {
    let mut largest: u64 = 0;
    for (i, tile) in data.iter().enumerate() {
        for other_tile in data.iter().skip(i + 1) {
            let current = find_area(*tile, *other_tile);
            if current > largest { largest = current; }
        }
    }
    largest
}

fn solution2(data: Vec<String>) -> i64 {
    let points: Vec<Point> = data.iter()
        .map(|point| { 
            point.split_once(',').and_then(|(str_a, str_b) | {
                let a = str_a.parse::<i64>().ok()?; 
                let b = str_b.parse::<i64>().ok()?;
                Some(Point {
                    x: a,
                    y: b,
                })
            }).unwrap()}
        ).collect();

    solve_max_rectangle(&points)
}

fn main() -> Result<(), Box< dyn Error>> {
    let reader = BufReader::new(File::open("./input.txt")?);

    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

    let parsed_data: Vec<(u64, u64)> = lines.clone().iter().map(|a|a.split_once(',').and_then(|(str_a, str_b)| {
            let a = str_a.parse::<u64>().ok()?;
            let b = str_b.parse::<u64>().ok()?;
            Some((a, b))
        })
        .unwrap()
    ).collect();

    // println!("Parsed Data: {:?}", parsed_data);
    // println!("how many thingies? {}", parsed_data.len());
    // println!("testing find area: {}", find_area((0 as u64, 0 as u64), (2 as u64, 2 as u64)));
    
    println!("Solution 1 | Largest Area Any 2: {}", solution1(parsed_data));

    println!("Solution 2 | Largest bounded area: {}", solution2(lines));

    Ok(())
}
