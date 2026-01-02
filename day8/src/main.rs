use std::error::Error;
use std::io::{BufRead, BufReader};
use std::fs::File;

fn distance_squared(a: [u64; 3], b: [u64; 3]) -> u64 {
    a.iter().zip(b.iter())
            .map(|(a, b)| a.abs_diff(*b).pow(2) as u64)
            .sum()
}

// for each point, iterate through all other points
// calcuate legs for each one and sort them in order
// at the end of every point loop, sort merge them all
//

fn find_all_pairs(input: Vec<[u64; 3]>) -> Vec<(([u64; 3],[u64; 3]), u64)> {
    let mut pairs: Vec<(([u64; 3],[u64; 3]), u64)> = Vec::new();
    for (i, current_point) in input.iter().enumerate() {
        for other_point in input.iter().skip(i + 1) {
            let dist = distance_squared(*current_point, *other_point);
            pairs.push(((*current_point, *other_point), dist));
        }
    }

    pairs.sort_by_key(|&(_, distance)| distance);
    println!("sorted pairs: {:?}:", pairs);
    pairs
}

fn solution1(input: Vec<[u64; 3]>) -> u64 {
    let mut pairs = find_all_pairs(input);
    0
}

// here is a data structure. it contains a list of pairs. every value
// is a key that can link to another value. so when we find pairs which
// have one value that is shared with another pair, we can link them

fn solution2(input: Vec<[u64; 3]>) -> u64 {
    0
}

fn main() -> Result<(), Box<dyn Error>>{

    let reader = BufReader::new(File::open("./input.txt")?);

    let input: Vec<[u64; 3]> = reader.lines()
        .map(|l| { 
            let line = l.expect("not a line");
            let v: Vec<u64> = line.split(',')
                .map(|n| n.parse::<u64>().expect("not a number"))
                .collect();
            v.try_into().expect("not 3 numbers")
            }).collect();

    //println!("Test: distance between 1, 1, 1 and 2, 2, 2: {}", distance_squared(&[1 as u64, 1 as u64, 1 as u64], &[2 as u64, 2 as u64, 2 as u64]));
    println!("Part 1 | Multiple of 3 largest: {}", solution1(input.clone()));
    println!("Part 2 | {}", solution2(input));


    Ok(())
}
