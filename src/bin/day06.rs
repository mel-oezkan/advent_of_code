use core::panic;
use std::collections::btree_set::Difference;
use std::fs;
use std::str::Chars;
use std::{collections::HashMap};
use std::path::Path;

use std::fmt::{Debug, Display};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ParseError {
    #[error("buffer pool ran out of free frames!")]
    ProcessError,
    #[error("buffer pool ry an out of free frames!")]
    Unknown,
}

const LEFT : char = '<';
const RIGHT : char = '>';
const UP : char = '^';
const DOWN : char = 'v';
const BLOCK : char = '#';

const DIRECTIONS : [char; 4] = [LEFT, RIGHT, UP, DOWN];

type Position = (i32, i32);

#[derive(Debug)]
struct Player {
    position: Position,
    direction: char,
}

#[derive(Debug)]
struct Solver {
    player: Player,
    board: Vec<Vec<char>>,
    counter: i32,
}

impl Solver {
    pub fn new (path: String) ->  Self {
        let content = fs::read_to_string(path)
            .expect("Error reading file");

        let mut board: Vec<Vec<char>> = content
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect();

        // find the starting position
        for row_index in 0..board.len() {
            let current_row = &board[row_index];
            for (col_index, current_value) in current_row.iter().enumerate() {
                for direction in DIRECTIONS.iter() {
                    if current_value == direction {
                        return Solver {
                            player: Player {
                                position: (col_index as i32, row_index as i32),
                                direction: *direction,
                            },
                            board,
                            counter: 0,
                        }
                    }
                }
            }
        }

        panic!("No player found");
    }


    fn move_player(&mut self) -> bool{
        let (x, y) = self.player.position;
        let dir = self.player.direction;

        let new_x = match dir {
            LEFT => x -1,
            RIGHT => x +1,
            _ => x,
        };
        let new_y = match dir {
            UP => y -1,
            DOWN => y +1,
            _ => y,
        };

        let new_pos = (new_x, new_y);
        if !self.check_bounds(new_pos) {
            self.counter += 1;

            return false;
        };


        self.board[y as usize][x as usize] = 'X';
        
        if self.check_block(new_pos) {
            self.player.direction = match dir {
                LEFT => UP,
                RIGHT => DOWN,
                UP => RIGHT,
                DOWN => LEFT,
                _ => panic!("Invalid direction"),
            };
            
            return true;
        }

        if self.board[new_y as usize][new_x as usize] == '.' {
            self.counter += 1;
        }

        self.player.position = new_pos;
        return true;
    }

    fn check_bounds(&self, pos: Position) -> bool {
        let (x, y) = pos;
        if x < 0 || y < 0 {
            return false;
        }
        
        let len_y = self.board.len();
        let len_x = self.board[0].len();

        if x >= len_x as i32 || y >= len_y as i32 {
            return false;
        }
        
        true
    }

    fn check_block(&self, pos: Position) -> bool {
        let (x, y) = pos;
        self.board[y as usize][x as usize] == BLOCK
    }

    fn pretty_print(&self) {
        // print the current position in red
        let (x, y) = self.player.position;
        for (i, row) in self.board.iter().enumerate() {
            for (j, value) in row.iter().enumerate() {
                if i == y as usize && j == x as usize {
                    print!("{}", self.player.direction);
                    continue;
                }
                print!("{}", value);
            }
            println!();
        }

    }

    pub fn solve(&mut self){
        let mut player = &mut self.player;
        let mut board = &mut self.board;

        let count = 0;
        loop {
            if count >= 1000 {
                break;
            }

            let valid_move = self.move_player();
            self.pretty_print();
            println!();

            if !valid_move {
                break;
            }
        }

        println!("Counter: {}", self.counter);

    }

}

fn main() {

    let path = String::from("inputs/day06.txt");    
    // let path = Path::new("test_inputs/day06.txt");

    let mut solution = Solver::new(path);
    // dbg!(solution.player);

    solution.solve();


    return;
}