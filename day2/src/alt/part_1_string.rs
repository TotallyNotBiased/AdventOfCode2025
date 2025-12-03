// Step 1: Throw out numbers that are easy to check (ranges with fully odd digits)
// Step 2: Convert to string, split string in two, compare the two halves using &str's PartialEq

use std::fs;
use std::error::Error;

fn decimal_digits(n: u64) -> u32 {
    n.checked_ilog10().unwrap_or(0) + 1
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = fs::read_to_string("./input.txt")?;

    let mut ranges: Vec<(u64, u64)> = data
        .trim()
        .split(',')
        .map(|s| {
            let (start_str, end_str) = s.split_once('-')
                .ok_or("Invalid format, missing hyphen")?;
            Ok((start_str.parse()?, end_str.parse()?))
            
        })
        .collect::<Result<_, Box<dyn Error>>>()?;

    ranges.sort_unstable_by_key(|r| r.0);

    let mut total_numbers_to_check: u64 = 0;
    let mut total: u64 = 0;

    for (start, end) in ranges {

        if !(decimal_digits(start) % 2 != 0 && (decimal_digits(start) == decimal_digits(end))) {

            println!("{} -> {}", start, end);

            for n in start..end {
                let numstring = n.to_string();
                let cmptuple = numstring.split_at(numstring.chars().count()/2);
                match cmptuple {
                    (x, y) if x == y => { 
                        total += (x.to_string() + y).parse::<u64>()?
                    },
                    _ => (), 
                }
            }
            total_numbers_to_check += end - start;
        }
        
        
    }

    println!("Total numbers to check: {}", total_numbers_to_check);
    println!("Total of all funny numbers: {}", total);



    Ok (())
}
