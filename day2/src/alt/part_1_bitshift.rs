// Part 2 solution, using bitshifts and pattern matching.

use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let data = fs::read_to_string("./input.txt")?;

    // begin the long ass iterator chain
    let total: u64 = data
        .trim()
        .split(',')
        .map(|s| {
            let (start_str, end_str) = s.split_once('-')
                .ok_or("Invalid format, missing hyphen")?;
            Ok((start_str.parse::<u64>()?, end_str.parse::<u64>()?))
        })
        .collect::<Result<Vec<(u64, u64)>, Box<dyn Error>>>()?
        .into_iter() // Turn the Vec back into an iterator
        // Serena's optimisation (throw out completely odd-length ranges)
        .filter(|(start, end)| {
            let start_len = start.checked_ilog10().unwrap_or(0) + 1;
            let end_len = end.checked_ilog10().unwrap_or(0) + 1;
            // Keep if lengths are different (crosses a boundary like 99-101)
            // OR if the length is even.
            start_len != end_len || start_len % 2 == 0
        })
        // flatten the ranges into a single stream of numbers
        .flat_map(|(start, end)| start..=end) 
        .filter(|&n| is_recursive_pattern(n))
        .sum();

    println!("Total of all funny numbers: {}", total);

    Ok(())
}


fn is_recursive_pattern(n: u64) -> bool {
    // ilog10 returns floor(log10(n)). 
    // length of any number is ilog10 + 1.
    let len = n.checked_ilog10().unwrap_or(0) + 1;

    // throw it out if length is odd
    if len % 2 != 0 {
        return false;
    }

    let divisor = 10_u64.pow(len / 2); 
    // 10 to the power of half the length allows us to snip the number with these operations:

    let upper = n / divisor; // essentially a decimal bitwise operation (cuts off right side)
    let lower = n % divisor; // the opposite, only keeps right side

    upper == lower
}