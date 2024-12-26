use std::collections::HashMap;



type Position = (i32, i32);

struct Solution {
    board: Vec<Vec<char>>,
    antenna_table: HashMap<char, Vec<Position>>,
}

impl Solution {
    pub fn new(path: String) -> Self {

        let board: Vec<Vec<char>> = std::fs::read_to_string(path)
            .expect("Failed to read file")
            .lines()
            .map(|line| line.chars().collect())
            .collect();

        let mut antenna_table: HashMap<char, Vec<Position>> = HashMap::new();

        Solution {
            board,
            antenna_table
        }
    }
}

fn main() {
    // let path = String::from("inputs/day08.txt");
    let path = String::from("test_inputs/day08.txt");

    let mut solution = Solution::new(path);
}

