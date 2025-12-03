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

    // we only need to check pattern lengths up to half the total digits.
    for pattern_len in 1..=(total_len / 2) {
        
        // pattern must repeat perfectly, so total digits
        // must be divisible by pattern length.
        if total_len % pattern_len != 0 {
            continue;
        }

        // Bitshift right, remove the last pattern_len digits
        let shift_divisor = 10_u64.pow(pattern_len);
        let prefix = n / shift_divisor;

        // Mask left, keep the rightmost length - pattern-length digits
        let mask_divisor = 10_u64.pow(total_len - pattern_len);
        let suffix = n % mask_divisor;

        if prefix == suffix {
            return true;
        }
    }

    false
}