use std::{fs};

fn main() {

    let input_file = fs::read_to_string("inputs/day02_ex1.txt")
        .expect("Readign the file caused an error");


    let mut n_safe = 0;
    for line in input_file.lines() {

        let mut prev_val: i32 = 0;

        let mut valid: bool = true;
        let mut total: i32 = 0;
        let mut abs_total: i32 = 0;

        for (i, val) in line.split_whitespace().enumerate() {
            let curr_val: i32 = val.parse().unwrap();

            if i > 0 { 
                let diff: i32 = prev_val - curr_val;

                // check the increase rule 
                if diff.abs() > 3 || diff == 0{
                    valid = false;
                    // println!("i:{}  j:{}\t  {}", prev_val, curr_val, line);
                    break;
                } 

                total += diff;
                abs_total += diff.abs();
            }

            // update the prev to current value for next check
            prev_val = curr_val;
        }

        // if still valid and the absolute sum matches the 
        // increments then the line is either increasing or decreasing
        if valid {
            
            println!("{}, t: {}, a: {} \t\t {}", abs_total == total.abs(), total, abs_total, line);
        }

        if abs_total == total.abs() && valid{
            // println!("safe: \t\t  {} ", line);
            n_safe += 1;
        }
    }


    println!("Number of safe lines: {}", n_safe);

}