use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

// my cs instinct says to use an interval tree but i hate pointers so we will use binary search instead

fn binary_search(ranges: &Vec<(u64, u64)>, number: u64) -> bool {
    let idx = ranges.partition_point(|range| range.0 <= number);

    // if idx is 0, the number is smaller than the very first range start.
    if idx > 0 {
        // check the range immediately before the partition point
        let (_, end) = ranges[idx - 1];
        
        // if the number fits in this previous range, we found it.
        if number < end {
            true
        } else {
            false
        }
    } else {
        false
    }
}

fn solution1(ranges: &Vec<(u64, u64)>, values: Vec<u64>) -> u64 {
    let mut total = 0;

    for value in values {
        if binary_search(&ranges, value) { total += 1; }
    };

    total
}

fn solution2(ranges: Vec<(u64, u64)>) -> u64{
    let mut total = 0;

    for (start, end) in ranges {
        total += (end - start) + 1
    }
    total
}

fn main() -> Result<(), Box<dyn Error>>{

    let reader = BufReader::new(File::open("./input.txt")?);
    let mut lines = reader.lines();

    let mut ranges: Vec<(u64, u64)> = Vec::new();
    let mut values: Vec<u64> = Vec::new();

    while let Some(line) = lines.next() {
        let line = line?;
        if line.trim().is_empty() {
            break;
        }

        if let Some ((start, end)) = line.split_once('-') {
            ranges.push((start.parse()?, end.parse()?))
        }
    }

    for line in lines {
        let line = line?;
        if !line.trim().is_empty() {
            values.push(line.parse()?);
        }
    }

    ranges.sort_unstable_by_key(|r| r.0);

    // now we merge ranges so we can binary search

    let mut merged: Vec<(u64, u64)> = Vec::new();
    
    let (mut cur_start, mut cur_end) = ranges[0];

    for (next_start, next_end) in ranges.clone().into_iter().skip(1) {
        // check for overlap
        // if the next range starts before (or exactly when) the current one ends
        if next_start <= cur_end {
            // we merge them by extending the current end to the max of both
            cur_end = cur_end.max(next_end);
        } else {
            // if no overlap, just push
            merged.push((cur_start, cur_end));
            cur_start = next_start;
            cur_end = next_end;
        }
    }

    merged.push((cur_start, cur_end));

    println!("Solution 1 Result | # of fresh IDs: {}", solution1(&merged, values));

    println!("Solution 2 Result | # of IDs that could be fresh {}", solution2(merged));


    Ok(())
}
