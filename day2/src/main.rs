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
        .filter(|&n| has_repeating_pattern(n))
        .sum();

    println!("Total of all funny numbers: {}", total);

    Ok(())
}


fn has_repeating_pattern(n: u64) -> bool {
    let log = n.checked_ilog10().unwrap_or(0);
    let total_len = log + 1;

    // We only need to check pattern lengths up to half the total digits.
    // If the number is 6 digits long, the longest possible repeating pattern 
    // is 3 digits (repeated twice).
    for pattern_len in 1..=(total_len / 2) {
        
        // Optimization: A pattern must repeat perfectly, so total digits
        // must be divisible by pattern length.
        if total_len % pattern_len != 0 {
            continue;
        }

        // 1. Shift Right: Remove the last 'pattern_len' digits
        // divisor = 10^L
        let shift_divisor = 10_u64.pow(pattern_len);
        let prefix = n / shift_divisor;

        // 2. Mask Left: Keep the last 'D - L' digits
        // effectively removing the first 'L' digits
        // mask = 10^(D-L)
        let mask_divisor = 10_u64.pow(total_len - pattern_len);
        let suffix = n % mask_divisor;

        if prefix == suffix {
            return true;
        }
    }

    false
}