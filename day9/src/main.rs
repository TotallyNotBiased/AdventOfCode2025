use std::error::Error;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::time::Duration;
use minifb::{Key, Window, WindowOptions};

struct Grid<T> {
    cells: Vec<Vec<Option<T>>>,
    max_x: usize,
    max_y: usize,
}

impl<T> Grid<T> {
    fn new() -> Self {
        Grid {
            cells: Vec::new(),
            max_x: 0,
            max_y: 0,
        }
    }

    fn place_point(&mut self, x: usize, y: usize, value: T) {
        while self.cells.len() <= y {
            self.cells.push(Vec::new());
            self.max_y = self.cells.len();
        }

        while self.cells[y].len() <= x {
            self.cells[y].push(None);
        }

        self.cells[y][x] = Some(value);

        self.max_x = self.max_x.max(x+1);
    }

    fn trim(&mut self) {
        let first_data_row = self.cells.iter().position(|row| row.iter()
            .any(|c| c.is_some()));
        let min_y = match first_data_row {
            Some(y) => y,
            None => { self.cells.clear(); self.max_x = 0; self.max_y = 0; return; }
        };

        if min_y > 0 { self.cells.drain(0..min_y); }

        let min_x = self.cells.iter()
            .filter_map(|row| row.iter().position(|c| c.is_some()))
            .min().unwrap_or(0);

        if min_x > 0 {
            for row in &mut self.cells {
                if row.len() > min_x { row.drain(0..min_x); } else { row.clear(); }
            }
        }
        self.max_y = self.cells.len();
        self.max_x = self.cells.iter().map(|r| r.len()).max().unwrap_or(0);
    }
    
    fn visualize<F>(&self, title: &str, scale: usize, color_mapper: F) 
    where 
        F: Fn(&T) -> u32 
    {
        let width = self.max_x;
        let height = self.max_y;
        
        if width == 0 || height == 0 {
            println!("Grid is empty, nothing to visualize.");
            return;
        }

        let bg_color = 0x00101010; 
        let mut buffer: Vec<u32> = vec![bg_color; width * height];

        for (y, row) in self.cells.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if let Some(val) = cell {
                    // Calculate index in 1D buffer
                    let idx = y * width + x;
                    buffer[idx] = color_mapper(val);
                }
            }
        }

        let mut window = Window::new(
            title,
            width,
            height,
            WindowOptions {
                scale: match scale {
                    1 => minifb::Scale::X1,
                    2 => minifb::Scale::X2,
                    4 => minifb::Scale::X4,
                    8 => minifb::Scale::X8,
                    _ => minifb::Scale::X1,
                },
                ..WindowOptions::default()
            },
        ).unwrap_or_else(|e| {
            panic!("{}", e);
        });

        while window.is_open() && !window.is_key_down(Key::Escape) {
            window.update_with_buffer(&buffer, width, height).unwrap();
            std::thread::sleep(Duration::from_millis(16)); // ~60 FPS cap
        }
    }
}

// given a red tile's coordinates, we KNOW that if there are no red tiles with the same x or y
// coordinates in a given direction, we don't need to check in that direction
//
// We know that every red tile has at least one tile in the same row OR column, because of the
// definition.
//
// We can set an outer bound for the grid 


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

fn solution2(data: Vec<(u64, u64)>) -> u64 {
    let mut grid: Grid<char> = Grid::new();

    for point in data {
        grid.place_point(point.0 as usize, point.1 as usize, '#');
    }

    grid.visualize("Advent of Code Day 9", 2, |val| { 
        let mut r = 0;
        let mut g = 0;
        let mut b = 0;
        if *val == '#' {
            r = 255;
        }
        
        (r << 16) | (g << 8) | b
    });
    0
}

fn main() -> Result<(), Box< dyn Error>> {
    let reader = BufReader::new(File::open("./input.txt")?);

    let parsed_data: Vec<(u64, u64)> = reader.lines()
        .map(|a|a.unwrap().split_once(',').and_then(|(str_a, str_b)| {
            let a = str_a.parse::<u64>().ok()?;
            let b = str_b.parse::<u64>().ok()?;
            Some((a, b))
        }).unwrap()).collect();

    // println!("Parsed Data: {:?}", parsed_data);
    // println!("how many thingies? {}", parsed_data.len());
    // println!("testing find area: {}", find_area((0 as u64, 0 as u64), (2 as u64, 2 as u64)));
    
    println!("Solution 1 | Largest Area Any 2: {}", solution1(parsed_data.clone()));
    println!("Solution 2 | Visualize + largest bounded: {}", solution2(parsed_data));

    Ok(())
}
