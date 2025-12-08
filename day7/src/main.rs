use std::error::Error;
use std::io::{BufRead, BufReader};
use std::fs::File;

// the splitters give an instruction each. left 1, right 1. that's all they do
// the other piece of info we have is their position.
// so, if we make a linear array of all the beams in a line like [50 53]
// and then we overlay the position of all beams on the carets if they are [50, 53]
// we can update the beams and count split events [49, 51, 52, 54] +2
// until the end of the file

fn solution1(data: Vec<String>) -> u32 {

    data.iter().map(|s| {
        s.chars().iter().map(|c| {

        })
        .collect();s
    });

    total
}

fn main() -> Result<(), Box<dyn Error>> {

    let reader = BufReader::new(File::open("./input.txt")?);

    let parsed_data: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

    println!("Part 1 Solution | Beam Split Times: {}", solution1(parsed_data));
    // println!("Part 2 Solution | Beam Split Events: {}", solution2(df));

    

    Ok(())
}
