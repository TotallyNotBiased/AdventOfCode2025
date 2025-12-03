// First working solution to part 2

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
    let mut hold = 0;

    for (command, amount) in parsed_data {

        hold = register;

        match command {
            'R' => register += amount,
            'L' => register -= amount,
            _ => unreachable!("Input guaranteed to be R or L"),
        }

        // before the euclid function, we can have a neg number
        // or a number greater than 99, so we can see if we have crossed 0
        // we need to do additional checks for any special cases
        // if, before rem_euclid equalization, we are at 0 or something that will be zero after,
        // we have to handle it

        if register > 99 && register.rem_euclid(100) == 0 { // case: exact multiple of 100
            answer += (register.abs() / 100) - 1; 
        } else if register > 99 {
            answer += register.abs() / 100;
        } else if register < 0 && (register.rem_euclid(100) == 0 || hold == 0){ // case: exact multiple of -100
            answer += register.abs() / 100;
        } else if register < 0 {
            answer += register.abs() / 100 + 1;
        }

        register = register.rem_euclid(100); // the greatest mod function

        if register == 0 {
            answer += 1;
        }
    }

    println!("Final Register: {}", register);
    println!("Answer (Times zero hit including during rotation): {}", answer);

    Ok(())
}

