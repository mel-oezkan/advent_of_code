use std::ops::Index;
use std::{array, fs, vec};
use std::path::Path;

#[derive(Debug, Clone, Copy)]
struct Point {x: i32, y: i32}

impl Point {
    fn get_direction(direction: Direction) -> Point {
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

fn check_xmas(char_mat: &Vec<&str>, pos: Point, dircetion: Direction) -> bool {
    
    let char_order: Vec<char>  = vec!['X','M', 'A', 'S'];

    // check if the indices are in valid range
    let max_row = char_mat.len();
    let max_col = char_mat[0].len();

    let current_val = char_mat[pos.y as usize]
        .chars().nth(pos.x as usize).unwrap(); 

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

            let curr_point = Point {x: col as usize, y: row as usize};

            // if current char is a starter
            if character == 'X' {

                for direction in DIRECTIONS {
                    check_xmas(&char_vec, curr_point, direction);
                }

            }
            
        }
        
        println!();
    }


    return;
}