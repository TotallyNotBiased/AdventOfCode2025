use std::error::Error;
use std::io::{BufRead, BufReader};
use std::fs::File;

fn distance_squared(a: &[u32; 3], b: &[u32; 3]) -> u32 {
    a.iter()
        .zip(b.iter())
        .map(|a, b| { a.abs_diff(*b).pow_2())
        .sum()
}


}
fn main() -> Result<(), Box<dyn Error>>{

    let reader = BufReader::new(File::open("./input.txt")?);

    let input: Vec<[u32; 3]> = reader.lines()
        .map(|l| { 
            let line = l.expect("not a line");
            let v: Vec<u32> = line.split(',')
                .map(|n| n.parse::<u32>().expect("not a number"))
                .collect();
            v.try_into().expect("not 3 numbers")
            }).collect();

        


    Ok(())
}
