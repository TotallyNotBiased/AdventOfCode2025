use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn Error>> {

    let reader = BufReader::new(File::open("./input.txt")?);
    let mut lines = reader.lines();

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

    Zip::indexed(numbers1)
        .and(numbers2)
        .for_each(|number, i| {

        })

    Ok(())
}
