use std::{fs, iter::zip};


fn get_file() -> String{
    let data = fs::read_to_string("inputs/day01_ex1.txt")
        .expect("Unable to read file");
    
    data
}

fn main() {

    let content = get_file();

    let mut v1: Vec<i32> = Vec::new();
    let mut v2: Vec<i32> = Vec::new();

    let iter = content.split_whitespace();

    for (i, s_val) in iter.enumerate() {
        let curr: i32 = s_val.parse().unwrap();

        if i % 2 == 0 {v1.push(curr);} else {v2.push(curr);}      
    }

    // sort the arrays
    v1.sort();
    v2.sort();


    let mut total: i32 = 0;
    for (i, j) in zip(v1, v2) {
        total += (i - j).abs();
    }

    println!("{}", total);

    return;
}
