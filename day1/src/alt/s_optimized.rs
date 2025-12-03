// Serena's part 2 solution

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

        match command {
            'R' => register += amount,
            'L' => { if register == 0 { answer -= 1 }; register -= amount;},
            _ => unreachable!("Input guaranteed to be R or L"),
        }

        // before the euclid function, we can have a neg number
        // or a number greater than 99, so we can see if we cross 0

        // 

        if register > 99 && register.rem_euclid(100) == 0 {
            answer += (register.abs() / 100) - 1;
        } else if register > 99 {
            answer += register.abs() / 100;
        } else if register < 0 && (register.rem_euclid(100) == 0) {
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

