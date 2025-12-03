// Jerrome's part 2 solution (not working)


use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = Path::new("./input.txt");

    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let parsed_data: Vec<(char, i32)> = reader
        .lines()
        .filter_map(|line_result| {
            let line = line_result.ok()?;
            let mut chars = line.chars();
            let character = chars.next()?;
            let number = chars.as_str().parse::<i32>().ok()?;
            Some((character, number))
        })
        .collect();

    let mut register: i32 = 50; 
    let mut answer = 0;

    for (command, amount) in parsed_data {

        let val = match command {
            'R' => amount,
            'L' => -amount,
            _ => unreachable!("Input guaranteed to be R or L"),
        };

        let target = register + val;

        let passes = target.div_euclid(100);

        answer += passes.abs();

        register = target.rem_euclid(100); // the greatest mod function
    }

    println!("Final Register: {}", register);
    println!("Answer (Times zero hit in total): {}", answer);

    Ok(())
}

