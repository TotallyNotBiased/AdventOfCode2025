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

fn find_all_pairs(input: &Vec<[u64; 3]>) -> Vec<(([u64; 3],[u64; 3]), u64)> {
    let mut pairs: Vec<(([u64; 3],[u64; 3]), u64)> = Vec::new();
    for (i, current_point) in input.iter().enumerate() {
        for other_point in input.iter().skip(i + 1) {
            let dist = distance_squared(*current_point, *other_point);
            pairs.push(((*current_point, *other_point), dist));
        }
    }

    pairs.sort_by_key(|&(_, distance)| distance);
    // debugging with print
    // println!("sorted pairs: {:?}:", pairs);
    pairs
}

fn solution1(input: Vec<[u64; 3]>) -> u64 {
    let pairs = find_all_pairs(&input);
    let mut circuits: Vec<Vec<[u64; 3]>> = Vec::new();

    // loop through pairs, put both into a new circuit unless it matches one of the
    // other circuits?
    // i need to make a new data structure callled circuits, which is made out of
    // vectors of vectors of points.
    // while looping over the pairs, i need to loop over the vectors as well
    // and check each vector for the existence of one of the points. if true
    // then push the OTHER point onto that vector. if false
    // make a new vector and then push both points onto that vector.
    // HOWEVER there is another case. What if two vectors exist and then they are
    // bridged by another pair?
    // Answer: if BOTH return true, then we merge the two vectors
    
    _ = pairs.into_iter().take(1000).map(|((a, b), _)| { 
        let a_exists = circuits.iter().position(|circuit| circuit.contains(&a));
        let b_exists = circuits.iter().position(|circuit| circuit.contains(&b));

        match (a_exists, b_exists) {
            (Some(a), Some(b)) => { if a != b {
                let (first_idx, second_idx) = if a > b {
                    (a, b)
                } else {
                    (b, a)
                };
                let mut source_vec = circuits.remove(first_idx);
                circuits[second_idx].append(&mut source_vec);
            }}
            (Some(a), None) => {
                circuits[a].push(b);
            }
            (None, Some(b)) => {
                circuits[b].push(a);
            }
            (None, None) => {
                circuits.push(vec![a, b]);
            }
        }}).collect::<Vec<_>>();

    circuits.sort_by_key(|v| v.len());
    circuits.reverse();

    (circuits[0].len() * circuits[1].len() * circuits[2].len()) as u64   
}

// here is a data structure. it contains a list of pairs. every value
// is a key that can link to another value. so when we find pairs which
// have one value that is shared with another pair, we can link them

fn solution2(input: Vec<[u64; 3]>) -> u64 {
    let pairs = find_all_pairs(&input);
    let mut circuits: Vec<Vec<[u64; 3]>> = Vec::new();
    
    // ditch the iterator for a for loop
    for ((n, m), _) in pairs { 
        let a_exists = circuits.iter().position(|circuit| circuit.contains(&n));
        let b_exists = circuits.iter().position(|circuit| circuit.contains(&m));

        match (a_exists, b_exists) {
            (Some(a), Some(b)) => { if a != b {
                let (first_idx, second_idx) = if a > b {
                    (a, b)
                } else {
                    (b, a)
                };
                let mut source_vec = circuits.remove(first_idx);
                circuits[second_idx].append(&mut source_vec);
                if circuits.len() <= 1 {
                    return n[0] * m[0];
                }

            }},
            (Some(a), None) => {
                circuits[a].push(m);
            },
            (None, Some(b)) => {
                circuits[b].push(n);
            },
            (None, None) => {
                circuits.push(vec![n, m]);
            },
        }
    }
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
    println!("Part 2 | Mult of x values of 2 last: {}", solution2(input));


    Ok(())
}
