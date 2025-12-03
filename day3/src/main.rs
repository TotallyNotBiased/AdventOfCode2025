use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use ndarray::{Array2, s, Axis};
use std::time::Instant;

// Okay. We need to iterate over each line, doing a left to right search. We need to find the largest
// single number in the line. However, we need to find the largest single number that also still has
// the second largest number in the line after it. How do we search for that?

// If we find the largest digit near the left, we are guaranteed to find the largest total number
// by looking right. The ONLY case where this doesn't work is if the largest number is the actual
// last number in the row.

// Serena's optimization: stop as SOON as you read a 9 in either case. You don't need to compare 
// higher than that.

// So we need to write an algorithm that iterates through the [len - 1] indexes of the row, then
// returns the index of the leftmost highest number. Then, we need to seach from that index to 
// the end of the row, returning the index of the highest number. Then we concatenate the digits
// to form a number and add it to a running total.

fn solution1(grid: &Array2<u32>) -> u64 {
    let mut total: u64 = 0;
    let width = grid.ncols();

    // iterate over the ndarray's vertical axis
    for row in grid.axis_iter(Axis(0)) {

        let mut val1 = 0; 
        let mut split_index = 0; // where we stop our first search

        // window of the row minus last member, enumerate over its digits.
        for (i, &val) in row.slice(s![0..width-1]).iter().enumerate() {
            if val > val1 {
                val1 = val;
                split_index = i;
            }
            if val == 9 {
                break;
            }
        }

        let mut val2 = 0;

        // window of the row from index to last member, iterate over its values.
        for &val in row.slice(s![split_index + 1..width]) {
            if val > val2 {
                val2 = val;
            }
            if val == 9 {
                break;
            }
        }

        let concatvalue = (val1 as u64 * 10) + val2 as u64;
        total += concatvalue;

    }
    total
}

fn solution2(grid: &Array2<u32>) -> u64 {
    let mut total: u64 = 0;
    let width = grid.ncols();
    let sequence_len = 12;

    // keep this part
    for row in grid.axis_iter(Axis(0)) {

        let mut currentval = 0; 
        let mut split_index = 0; // where we stop/start our searches

        // run this 12 times. digits remaining decrease
        for digits_remaining in (1..=sequence_len).rev() {
            // we're looking for at least 12 digits so we can't look further than this. 
            // the window moves back each time
            let search_end_index = width - (digits_remaining - 1);
            
            // the split index keeps marching forward
            let window = row.slice(s![split_index..search_end_index]);

            let mut best_digit = 0;
            let mut best_relative_index = 0;

            // run the comparison from split index to the end
            for (i, &val) in window.iter().enumerate() {
                if val > best_digit {
                    best_digit = val;
                    best_relative_index = i;
                }
                if val == 9 {
                    break;
                }
            }

            currentval = (currentval as u64 * 10) + best_digit as u64;

            split_index += best_relative_index + 1
        }
        

            
        total += currentval;

    }
    total
}

fn main() -> Result<(), Box<dyn Error>> {

    let file_path = Path::new("./input.txt");
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut width = 0;

    let parsed_data: Vec<u32> = reader
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            let line = line.expect("Failed to read line");

            let row: Vec<u32> = line.chars()
                .filter_map(|c| c.to_digit(10))
                .collect();

            if i == 0 { width = row.len() }

            row
        })
        .collect();

    let height = parsed_data.len() / width;
    let grid = Array2::from_shape_vec((height, width), parsed_data)?;

    println!("Loaded grid: {:?}", grid.dim());

    let result = solution1(&grid);
    println!("Total: {}", result);

    let start_time = Instant::now();
    let result = solution2(&grid);
    let elapsed_time = start_time.elapsed();

    println!("Total: {}\nExecution Time: {:?}", result, elapsed_time);

    Ok(())
}
