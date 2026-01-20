use std::error::Error;
use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() -> Result<(), Box< dyn Error>> {
    let reader = BufReader::new(File::open("./input.txt")?);

    let parsed_data: Vec<(u64, u64)> = reader.lines()
        .map(|a|a.unwrap().split_once(',').and_then(|(str_a, str_b)| {
            let a = str_a.parse::<u64>().ok()?;
            let b = str_b.parse::<u64>().ok()?;
            Some((a, b))
        }).unwrap()).collect();

    println!("Parsed Data: {:?}", parsed_data);
    println!("how many thingies? {}", parsed_data.len());

    Ok(())
}
