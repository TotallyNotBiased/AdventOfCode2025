use core::num;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn solution1(lines: &Vec<String>) -> u64 {
    // okay, start again
    let mut total = 0;
    
    // split the data 
    let (num_rows, op_row) = lines.split_at(lines.len() - 1);

    let grid: Vec<Vec<u64>> = num_rows
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<u64>().unwrap())
                .collect()
        }).collect();

    let operators: Vec<bool> = op_row[0]
        .split_whitespace()
        .map(|s| match s {
            "*" => true,
            "+" => false,
            _ => unreachable!(),
        }).collect();

    let num_columns = grid[0].len(); // prepare to transpose

    for col_idx in 0..num_columns { 
        
        // iterate over the rows and find the numbers at the current col_idx
        let column_values = grid.iter().map(|row| row[col_idx]);

        // find the operator for that index in our Vec<bool>
        let is_multiplication = operators[col_idx];

        let column_result = if is_multiplication {
            column_values.product::<u64>() 
        } else {
            column_values.sum::<u64>()
        };

        total += column_result;
    }

    total
}

fn solution2(lines: &Vec<String>) -> u64 {
    let mut total = 0;

    let grid: Vec<Vec<char>> = lines
        .into_iter()
        .map(|s| s.chars().collect())
        .collect();

    let num_columns = grid[0].len();

    let mut intermediate_sum = 0;

    for col_idx in (0..num_columns).rev() {
        let column_values = grid.iter().map(|row| row[col_idx]);

        let num = column_values
            .filter_map(|c| c.to_digit(10))
            .fold(0, |acc, digit| acc * 10 + digit);

        
                                                


    }

    total

}

fn main() -> Result<(), Box<dyn Error>> {

    let reader = BufReader::new(File::open("./input.txt")?);
    /* nah this is awful, gonna stop right here before it gets worse

    let numbers1: Vec<u64> = lines
        .next().unwrap()?
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let numbers2: Vec<u64> = lines
        .next().unwrap()?
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let operators: Vec<bool> = lines
        .next().unwrap()?
        .split_whitespace()
        .map(|s| {match s { "+" => false, "*" => true, _ => unreachable!() }})
        .collect();

    let zipped_iter = std::iter::zip(numbers1, numbers2);
    
    let _result = zipped_iter.enumerate()
        .map(|(i, (num1, num2))| {
            match operators[i] {
                false => { 
                    total += num1 + num2
                },
                true => {
                    total += num1 * num2
                }
            }
        })
        .collect::<Vec<_>>();

    */

    // smash everything into a string vec (5 lines)
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;


    println!("Solution 1 Result | Sum of all problems: {}", solution1(&lines));
    println!("Solution 2 Result | Sum of all Chinese problems {}", solution2(&lines));

    Ok(())
}
