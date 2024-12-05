use std::ops::Index;
use std::{array, fs, vec};
use std::path::Path;

#[derive(Debug, Clone, Copy)]
struct Point {x: i32, y: i32}

impl Point {
    fn get_direction_delta(direction: &Direction) -> Point {
        match direction {
            Direction::Right => Point { x: 1, y: 0 },
            Direction::Left => Point { x: -1, y: 0 },
            Direction::Up => Point { x: 0, y: 1 },
            Direction::Down => Point { x: 0, y: -1 },
            Direction::RightUp => Point { x: 1, y: 1 },
            Direction::RightDown => Point { x: 1, y: -1 },
            Direction::LeftUp => Point { x: -1, y: 1 },
            Direction::LeftDown => Point { x: -1, y: -1 },
        }
    }


    fn print_direction(direction: &Direction) -> &str {
        match direction {
            Direction::Right => "right",
            Direction::Left => "left",
            Direction::Up => "up",
            Direction::Down => "down",
            Direction::RightUp => "right-up",
            Direction::RightDown => "right-down",
            Direction::LeftUp => "left-up",
            Direction::LeftDown => "left-down"
        }
    }
}

enum Direction {
    Right,
    Left,
    Up,
    Down,
    RightUp,
    RightDown,
    LeftUp,
    LeftDown,
}

const DIRECTIONS: [Direction; 8] = [
    Direction::Right,
    Direction::Left,
    Direction::Up,
    Direction::Down,
    Direction::RightUp,
    Direction::RightDown,
    Direction::LeftUp,
    Direction::LeftDown,
];

struct ProblemSolver {
    grid: Vec<char> 
}


impl ProblemSolver {

    fn solution1 (self) {

    }
}

fn check_xmas(char_mat: &Vec<&str>, pos: Point, direction: &Direction, char_idx: usize) -> usize {
    
    let char_order: Vec<char>  = vec!['X','M', 'A', 'S'];

    // check if the indices are in valid range
    let max_y = char_mat.len() as i32;
    let max_x = char_mat[0].len() as i32;

    // check if the next char pos is contained in the grid
    let dir_delta: Point = Point::get_direction_delta(direction);
    let new_pos = Point { 
        x: pos.x + dir_delta.x, 
        y: pos.y + dir_delta.y};

    if new_pos.x < 0 || new_pos.x >= max_x 
    || new_pos.y < 0 || new_pos.y >= max_y {

        return 0;
    }

    // get the next char and check if the chars mathch
    let next_char = char_order[char_idx];
    let grid_char = char_mat[new_pos.y as usize]
        .chars().nth(new_pos.x as usize).unwrap();

    // check if char matches next char
    if next_char == grid_char {
        // check if terminal char "S"
        if char_idx == 3 {
            return char_idx+1;
        }

        // continue the search with next char
        return check_xmas(char_mat, new_pos, direction, char_idx+1);
    }

    char_idx
}

fn main() {

    let path = Path::new("inputs/day04_ex1.txt");
    // let path = Path::new("test_inputs/day04_test.txt");

    let content: String = fs::read_to_string(path)
        .expect("Error reading file");

    let char_vec: Vec<&str> = content.lines().map(|x| x).collect();
    
    println!("{}", char_vec.len());

    let mut n_matches = 0;
    
    for (row, line) in content.lines().enumerate() {
        for (col, character) in line.chars().enumerate() {

            let curr_pos = Point {x: col as i32, y: row as i32};

            // if current char is a starter
            if character == 'X' {

                for direction in DIRECTIONS {

                    let found_xmas: usize = check_xmas(&char_vec, curr_pos, &direction, 1);

                    // for each direction check if the next char for xmas is contained
                    if found_xmas == 4 {
                        n_matches += 1;
                    };
                }

            }
            
        }
        
    }

    println!("matches: {}", n_matches);
}