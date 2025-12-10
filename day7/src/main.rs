use std::error::Error;
use std::io::{BufRead, BufReader};
use std::fs::File;

// the splitters give an instruction each. left 1, right 1. that's all they do
// the other piece of info we have is their position.
// so, if we make a linear array of all the beams in a line like [50 53]
// and then we overlay the position of all beams on the carets if they are [50, 53]
// we can update the beams and count split events [49, 51, 52, 54] +2
// until the end of the file

fn solution2(data: Vec<String>) -> u64 {

    let size = data[0].len();
    let mut current_vec: Vec<u64> =  vec![0; size];
    

    let _result = data.iter().map(|s| {
        let mut next_vec: Vec<u64> = vec![0; size];
        s.chars().enumerate().map(|(idx, c)| {
            let path_count = current_vec[idx];
            match c {
                'S' => { next_vec[idx] += 1; },
                '^' => { if current_vec[idx] > 0 { 
                            next_vec[idx] = 0;
                            next_vec[idx - 1] += path_count;
                            next_vec[idx + 1] += path_count;
                         }},
                 _  => {next_vec[idx] += path_count},
            }
        })
        .collect::<Vec<_>>();
        current_vec = next_vec;
    }).collect::<Vec<_>>();

    current_vec.iter().sum()
}

// so what we have here is a pre-topologically sorted directed acyclic graph
// and if we... actually fuck it Serena says this is obviously
// Pascal's triangle xd

fn solution1(data: Vec<String>) -> u32 {

    let size = data[0].len();
    let mut working_vec: Vec<bool> =  vec![false; size];
    let mut total = 0; 

    let _result = data.iter().map(|s| {
        s.chars().enumerate().map(|(idx, c)| {
            match c {
                'S' => { working_vec[idx] = true; },
                '^' => { if working_vec[idx] == true { 
                            total += 1;
                            working_vec[idx] = false;
                            working_vec[idx - 1] = true;
                            working_vec[idx + 1] = true;
                         }},
                 _  => {},
            }

        })
        .collect::<Vec<_>>();
    }).collect::<Vec<_>>();

    total
}

fn main() -> Result<(), Box<dyn Error>> {

    let reader = BufReader::new(File::open("./input.txt")?);

    let parsed_data: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

    println!("Part 1 Solution | Beam Split Times: {}", solution1(parsed_data.clone()));
    println!("Part 2 Solution | Timelines Generated: {}", solution2(parsed_data));

    

    Ok(())
}
