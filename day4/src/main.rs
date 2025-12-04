use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

use ndarray::{Array2, s, Zip};
use std::cmp::min;
use std::mem;


fn is_adjacent_threshold(grid: &Array2<char>, center: (usize, usize), count: u8) -> bool {
    
    let radius = 1;

    let (rows, cols) = grid.dim();
    let (x, y) = center;

    // calc slice positions and dimensions
    let r_start = x.saturating_sub(radius);
    let c_start = y.saturating_sub(radius);

    let r_end = min(x + radius + 1, rows);
    let c_end = min(y + radius + 1, cols);


    // serena's optimisation: slice, then count adjacent. 
    let window = grid.slice(s![r_start..r_end, c_start..c_end]);

    // if we reach more than 5 blanks it's good. If we reach more than 4 rolls we know it's not good.
    // NOTE: it is 4 because we are also arbitrarily accessing the center of the window with try_fold().
    let result = window.iter().try_fold((0, 0), |(a, b), &c| {
        let blanks = if c == '.' { a + 1 } else { a };
        let rolls = if c == '@' { b + 1 } else { b };

        // abuse the shit out of try_fold to pass errors as values
        if blanks > (8 - count) {
            return Err(true);
        }
        if rolls > (count) {
            return Err(false);
        }

        // 3. Continue with new state
        Ok((blanks, rolls))
    });

    // Handle the output
    match result {
        Err(val) => val, // pass the error as value
        Ok(_) => true,         // we are on an edge. we didn't hit more than 4 rolls so we're good.
    }

}

fn solution1(grid: &Array2<char>) -> u32 {
    // iterate over the grid.
    // for each @ call is_adjacent_threshold() and if true then increment

    let mut total = 0;

    for ((r, c), value) in grid.indexed_iter(){
        if value == &'@' {
            if is_adjacent_threshold(grid, (r, c), 4) { total += 1; }
        }
    }

    total
}

// conway's game of life solution

fn solution2(grid: &Array2<char>) -> u32 {

    let mut total = 0;
    
    let mut current = grid.clone();
    let mut next = grid.clone();

    loop {

        let mut subtotal = 0;

        Zip::indexed(&mut next)
        .and(&current)
        .for_each(|(row, col), next_cell, &current_cell| {

            *next_cell = if is_adjacent_threshold(&current, (row, col), 4) {
                '.'
            } else {
                current_cell
            };

            if *next_cell != current_cell {
                subtotal += 1;
            }
        });

        if subtotal == 0 { break; }
        total += subtotal;

        mem::swap(&mut current, &mut next)
    }

    total
}

fn main() -> Result<(), Box<dyn Error>> {

    let reader = BufReader::new(File::open("./input.txt")?);

    let mut width = 0;

    let parsed_data: Vec<char> = reader
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            let line = line.expect("Failed to read newline");

            let row: Vec<char> = line.chars().collect();
            if i == 0 { width = row.len() }

            row
        })
        .collect();

    let height = parsed_data.len() / width;
    let grid = Array2::from_shape_vec((height, width), parsed_data)?;

    println!("Loaded grid: {:?}", grid.dim());

    println!("Solution 1 | Number of accessible rolls: {}", solution1(&grid));
    println!("Solution 2 | Number of removable rolls: {}", solution2(&grid));

    Ok(())
}
