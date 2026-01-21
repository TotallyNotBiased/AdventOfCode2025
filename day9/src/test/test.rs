use std::error::Error;
use std::io::{BufRead, BufReader};
use std::fs::File;
use minifb::{Key, MouseMode, Window, WindowOptions};
use std::collections::HashMap;
use std::time::Duration;
use std::cmp::{min, max};

struct SparseGrid<T> {
    cells: HashMap<(isize, isize), T>, 
}

impl<T> SparseGrid<T> {
    fn new() -> Self {
        SparseGrid {
            cells: HashMap::new(),
        }
    }

    fn place_point(&mut self, x: isize, y: isize, value: T) {
        self.cells.insert((x, y), value);
    }

    fn visualize<F>(&self, title: &str, window_width: usize, window_height: usize, color_mapper: F)
    where
        F: Fn(&T) -> u32,
    {
        let mut camera_x: isize = 0;
        let mut camera_y: isize = 0;
        let mut zoom: f32 = 1.0; 

        let mut window = Window::new(
            title,
            window_width,
            window_height,
            WindowOptions {
                resize: true,
                ..WindowOptions::default()
            },
        ).unwrap();

        let mut buffer: Vec<u32> = vec![0; window_width * window_height];

        while window.is_open() && !window.is_key_down(Key::Escape) {
            if window.is_key_down(Key::Left)  { camera_x -= (10.0 / zoom) as isize; }
            if window.is_key_down(Key::Right) { camera_x += (10.0 / zoom) as isize; }
            if window.is_key_down(Key::Up)    { camera_y -= (10.0 / zoom) as isize; }
            if window.is_key_down(Key::Down)  { camera_y += (10.0 / zoom) as isize; }
            
            if window.is_key_down(Key::Z) { zoom *= 1.05; }
            if window.is_key_down(Key::X) { zoom *= 0.95; }

            for p in buffer.iter_mut() { *p = 0x00101010; }

            for ((world_x, world_y), value) in &self.cells {
                
                // transform world -> screen
                let screen_x = ((*world_x - camera_x) as f32 * zoom) as isize + (window_width as isize / 2);
                let screen_y = ((*world_y - camera_y) as f32 * zoom) as isize + (window_height as isize / 2);

                // check bounds
                if screen_x >= 0 && screen_x < window_width as isize && 
                   screen_y >= 0 && screen_y < window_height as isize 
                {
                    let size = (1.0 * zoom).max(1.0) as isize; // At least 1 pixel
                    let color = color_mapper(value);

                    for dy in 0..size {
                        for dx in 0..size {
                            let px = screen_x + dx;
                            let py = screen_y + dy;
                            
                            if px < window_width as isize && py < window_height as isize {
                                let idx = (py as usize) * window_width + (px as usize);
                                buffer[idx] = color;
                            }
                        }
                    }
                }
            }

            window.update_with_buffer(&buffer, window_width, window_height).unwrap();
            std::thread::sleep(Duration::from_millis(16));
        }
    }
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

fn solution2(data: Vec<(u64, u64)>) -> u64 {

    let mut grid = SparseGrid::new();

    for (i, point) in data.iter().enumerate() {
        if i < (data.len() - 1) {
            let next_point = data[i+1];
            
            if point.0 == next_point.0 {
                let start_y = min(point.1, next_point.1);
                let end_y = max(point.1, next_point.1);

                for y in (start_y + 1)..end_y {
                    grid.place_point(point.0 as isize, y as isize, '@');
                }
            }
            else if point.1 == next_point.1 {
                let start_x = min(point.0, next_point.0);
                let end_x = max(point.0, next_point.0);

                for x in (start_x + 1)..end_x {
                    grid.place_point(x as isize, point.1 as isize, '@');
                }
            }
            grid.place_point(point.0 as isize, point.1 as isize, '#');
        }
    }

    println!("Grid populated. Opening window...");
    
    grid.visualize("AoC 2025 Day 9 Viewer", 800, 600, |val| {
        let mut r = 0;
        let mut g = 0;
        let mut b = 0;
        if *val == '#' {
            r = 255;
        }
        if *val == '@' {
            g = 255;
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
