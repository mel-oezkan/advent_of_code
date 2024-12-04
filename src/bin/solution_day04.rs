use std::ops::Index;
use std::{array, fs, vec};
use std::path::Path;


fn check_xmas(char_mat: &Vec<&str>, col: usize, row: usize, next: usize, reverse: bool) -> bool {
    
    let char_order: Vec<char>  = vec!['X','M', 'A', 'S'];

    // check if the indices are in valid range
    let max_row = char_mat.len();
    let max_col = char_mat[0].len();

    let current_val = char_mat[row]
        .chars().nth(col).unwrap(); 
    
    

    return true;
}

fn main() {

    // let path = Path("inputs/day04_ex1.txt");
    let path = Path::new("test_inputs/day04_test.txt");

    let content: String = fs::read_to_string(path)
        .expect("Error reading file");

    let char_vec: Vec<&str> = content.lines()
        .map(|x| x).collect(); 
    
    println!("{}", char_vec.len());
    
    for (row, line) in content.lines().enumerate() {
        for (col, character) in line.chars().enumerate() {
            print!("{}", character);

            // if current char is a starter
            if character == 'S' {
                check_xmas(&char_vec, col, row, 1, true);

            } else if  character == 'X' {
                check_xmas(&char_vec, col, row, 1, false);
            }
            
        }
        
        println!();
    }


    return;
}