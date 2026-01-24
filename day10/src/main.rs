use std::io::{BufRead, BufReader};
use std::fs::File;
use std::error::Error;
use regex::Regex;

fn linear_solve_min(button_masks: &[u64], target_state: u64) -> Option<u64> {
    let num_buttons = button_masks.len();
    let num_lights = 10; 

    let mut matrix: Vec<u128> = Vec::with_capacity(num_lights);

    for light in 0..num_lights {
        let mut row_value: u128 = 0;
        for (button_index, button_mask) in button_masks.iter().enumerate() {
            if (button_mask >> light) & 1 == 1 {
                row_value |= 1 << button_index;
            }
        }
        if (target_state >> light) & 1 == 1 {
            row_value |= 1 << num_buttons; 
        }
        matrix.push(row_value);
    }

    let mut pivot_row = 0;
    let mut col = 0;
    
    let mut pivot_cols = vec![false; num_buttons]; 

    while pivot_row < num_lights && col < num_buttons {
        let mut swap_row = None;
        for r in pivot_row..num_lights {
            if (matrix[r] >> col) & 1 == 1 {
                swap_row = Some(r);
                break;
            }
        }

        if let Some(r_idx) = swap_row {
            matrix.swap(pivot_row, r_idx);
            pivot_cols[col] = true;

            for r in 0..num_lights {
                if r != pivot_row && (matrix[r] >> col) & 1 == 1 {
                    let pivot_val = matrix[pivot_row];
                    matrix[r] ^= pivot_val;
                }
            }
            pivot_row += 1;
        }
        col += 1;
    }

    for r in 0..num_lights {
        let row_mask = (1 << num_buttons) - 1;
        let target_bit = (matrix[r] >> num_buttons) & 1;
        if (matrix[r] & row_mask) == 0 && target_bit == 1 {
            return None; // means unsolvable, but also this should be unreachable
        }
    }

    let mut particular_solution: u64 = 0;
    for r in 0..num_lights {
        let row_val = matrix[r];
        if row_val == 0 { continue; }
        
        let pivot_col = row_val.trailing_zeros() as usize;
        if pivot_col < num_buttons {
            let target_val = (row_val >> num_buttons) & 1;
            if target_val == 1 {
                particular_solution |= 1 << pivot_col;
            }
        }
    }

    let mut null_basis: Vec<u64> = Vec::new();

    for c in 0..num_buttons {
        if !pivot_cols[c] {
            let mut basis_vector = 1 << c;
            
            for r in 0..num_lights {
                if (matrix[r] >> c) & 1 == 1 {
                    let pivot_c = matrix[r].trailing_zeros() as usize;
                    if pivot_c < num_buttons {
                         basis_vector |= 1 << pivot_c;
                    }
                }
            }
            null_basis.push(basis_vector);
        }
    }

    let mut best_solution = particular_solution;
    let mut min_popcount = best_solution.count_ones();

    let num_free_vars = null_basis.len();
    let combinations = 1 << num_free_vars;

    for i in 1..combinations {
        let mut current_solution = particular_solution;
        
        for (b_idx, basis) in null_basis.iter().enumerate() {
            if (i >> b_idx) & 1 == 1 {
                current_solution ^= basis;
            }
        }

        let pop = current_solution.count_ones();
        if pop < min_popcount {
            min_popcount = pop;
            best_solution = current_solution;
        }
    }

    Some(best_solution)
}
fn parse_lines(input: &str) -> (u64, Vec<u64>, Vec<u64>) {
    let re_header = Regex::new(r"\[([.#]+)\]").unwrap();
    let header_caps = re_header.captures(input).expect("Could not find [...] pattern");
    let header_str = header_caps.get(1).unwrap().as_str();

    let mut header_val: u64 = 0;
    for (i, c) in header_str.chars().enumerate() {
        if c == '#' {
            header_val |= 1 << i;
        }
    }

    let re_masks = Regex::new(r"\(([\d,\s]+)\)").unwrap();
    let mut masks = Vec::new(); // No fixed size
    
    for cap in re_masks.captures_iter(input) {
        let inner = cap.get(1).unwrap().as_str();
        let mut mask_val: u64 = 0;
        
        for num_str in inner.split(',') {
            if let Ok(shift) = num_str.trim().parse::<u64>() {
                mask_val |= 1 << shift;
            }
        }
        masks.push(mask_val);
    } 

    let re_array = Regex::new(r"\{([\d,\s]+)\}").unwrap();
    let array_caps = re_array.captures(input).expect("Could not find {...} pattern");
    let array_str = array_caps.get(1).unwrap().as_str();

    let mut values = Vec::new();
    for num_str in array_str.split(',') {
        if let Ok(num) = num_str.trim().parse::<u64>() {
            values.push(num);
        }
    }

    (header_val, masks, values)
}

fn solution1(states: Vec<u64>, buttons: Vec<Vec<u64>>) -> u64 {
    let mut minimum_presses: u64 = 0;

    for (state, button_masks) in states.iter().zip(buttons) {
        if let Some(mask) = linear_solve_min(&button_masks, *state) {
             minimum_presses += mask.count_ones() as u64; 
        } else {
             println!("Still no solution for state {:b}", state);
        }
    }

    minimum_presses
    
}


fn main() -> Result<(), Box<dyn Error>> {

    let reader = BufReader::new(File::open("./input.txt")?);

    let data: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

    let mut states: Vec<u64> = Vec::new();
    let mut buttons: Vec<Vec<u64>> = Vec::new();
    let mut joltages: Vec<Vec<u64>> = Vec::new();
    
    for line in data.iter() {
        let (s, b, j) = parse_lines(line);
        states.push(s);
        buttons.push(b);
        joltages.push(j);
    }

    println!("{:?}", buttons);

    println!("Solution 1 | Minimum Button Presses: {}", solution1(states, buttons));

    Ok(())
}
