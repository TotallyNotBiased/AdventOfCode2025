use std::collections::{BTreeSet, HashMap};
use std::io::{BufRead, BufReader};
use std::error::Error;
use std::fs::File;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
}
fn solve_max_rectangle(polygon: &[Point]) -> i64 {
    // --- STEP 1: Coordinate Compression with "+1 Expansion" ---
    // We add both v and v+1. This ensures that every coordinate line
    // creates a dedicated 1-unit wide strip in our grid.
    let mut xs = BTreeSet::new();
    let mut ys = BTreeSet::new();
    for p in polygon {
        xs.insert(p.x);
        xs.insert(p.x + 1); // Expansion for thickness
        ys.insert(p.y);
        ys.insert(p.y + 1); // Expansion for thickness
    }
    let xs: Vec<i64> = xs.into_iter().collect();
    let ys: Vec<i64> = ys.into_iter().collect();

    let x_map: HashMap<i64, usize> = xs.iter().enumerate().map(|(i, &v)| (v, i)).collect();
    let y_map: HashMap<i64, usize> = ys.iter().enumerate().map(|(i, &v)| (v, i)).collect();

    let grid_width = xs.len() - 1;
    let grid_height = ys.len() - 1;
    let mut grid = vec![vec![false; grid_width]; grid_height];

    // --- STEP 2: Paint Borders & Fill Interior ---
    
    // A. Paint Vertical Borders
    // We mark which columns are "vertical walls". This helps the scanline later.
    let mut is_vert_wall = vec![vec![false; grid_width]; grid_height];

    for i in 0..polygon.len() {
        let p1 = polygon[i];
        let p2 = polygon[(i + 1) % polygon.len()];

        // Map physical coords to grid indices
        let c1 = x_map[&p1.x];
        let r1 = y_map[&p1.y];
        let c2 = x_map[&p2.x];
        let r2 = y_map[&p2.y];

        // 1. Paint Vertical Edges
        if p1.x == p2.x {
            let c = c1; // The column index for the interval [x, x+1)
            let r_start = r1.min(r2);
            let r_end = r1.max(r2);
            
            for r in r_start..r_end {
                grid[r][c] = true; 
                is_vert_wall[r][c] = true; // Mark as a wall for the scanline
            }
        }
        // 2. Paint Horizontal Edges
        else if p1.y == p2.y {
            let r = r1; // The row index for the interval [y, y+1)
            let c_start = c1.min(c2);
            let c_end = c1.max(c2);
            
            for c in c_start..c_end {
                grid[r][c] = true;
            }
        }
    }

    // B. Scanline Fill (Horizontal Pass)
    // Iterate through every row. Toggle "Inside" status when we pass a vertical wall.
    for r in 0..grid_height {
        let mut inside = false;
        
        for c in 0..grid_width {
            if is_vert_wall[r][c] {
                // If we are currently on a vertical wall, we are definitely "Inside".
                // But we need to know if we are ENTERING or EXITING the shape.
                // In a compressed grid with +1 expansion, a wall is a solid block.
                // We toggle the state only when we *finish* passing the wall.
                
                // Peek ahead: If the next column is NOT a wall, we toggle.
                // (This handles walls that might be >1 unit thick if coords were close)
                let next_is_wall = if c + 1 < grid_width { is_vert_wall[r][c+1] } else { false };
                
                if !next_is_wall {
                    inside = !inside;
                }
            } else {
                // We are in empty space. Fill it if we are currently "Inside".
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

            // Normalize coordinates
            let r_min = r1.min(r2);
            let r_max = r1.max(r2); 
            let c_min = c1.min(c2);
            let c_max = c1.max(c2);

            // FIX 1: Allow 1-unit thick rectangles (lines) 
            // If r_min == r_max, it's a horizontal line with height = 1 (due to thickness)
            // So we remove the 'continue' check that skips them.

            // FIX 2: Extend the range to include the thickness of the bottom/right edges.
            // Because we added 'v+1' to the coords, xs[c_max + 1] is exactly (p_max.x + 1).
            
            // Calculate Expected Area using the inclusive outer boundary
            let width = xs[c_max + 1] - xs[c_min];
            let height = ys[r_max + 1] - ys[r_min];
            let expected_area = width * height;

            // FIX 3: Adjust SAT query to include the r_max/c_max row/col
            // SAT indices:
            // Top-Left: (r_min, c_min)
            // Bottom-Right (exclusive in SAT logic): (r_max + 1, c_max + 1)
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
    let mut set: BTreeSet<i64> = iter.collect();
    set.into_iter().collect()
}

fn main() -> Result<(), Box<dyn Error>> {
    let reader = BufReader::new(File::open("./input.txt").unwrap());

    let points: Vec<Point> = reader.lines()
        .map(|point| { 
            point.unwrap().split_once(',').and_then(|(str_a, str_b) | {
                let a = str_a.parse::<i64>().ok()?; 
                let b = str_b.parse::<i64>().ok()?;
                Some(Point {
                    x: a,
                    y: b,
                })
            }
        ).unwrap()}).collect();

    let result = solve_max_rectangle(&points);
    println!("Max Area: {}", result);

    Ok(())
}
