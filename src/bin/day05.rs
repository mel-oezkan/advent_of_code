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


struct ContentParser {
    path: String,
    rules: Vec<(i32, i32)>,
    hash_rules: HashMap<i32, Vec<i32>>,
    sequences: Vec<Vec<i32>>
}   

impl ContentParser {
    pub fn new(path: String) -> Self {
        ContentParser {
            path,
            rules: Vec::new(),
            hash_rules: HashMap::new(),
            sequences: Vec::new()
        }
    }

    pub fn solve_1(&mut self) -> i32 {
        // method to solve the first part of the problem
        let content_path = Path::new(&self.path);
        let content = fs::read_to_string(content_path)
            .expect("Error reading file");

        self.parse_conent(&content);

        self.process_rules();
        dbg!(&self.hash_rules);

        self.process_sequences();

        return 0;
    }

    fn parse_conent(&mut self, content: &String) {
        
        let mut parse_rules = true;
        for line in content.lines() {
            
            if line == "" {
                parse_rules = false;
                continue;
            }

            if parse_rules {
                self.rules.push(self.parse_rule(line));
            } else {
                self.sequences.push(self.parse_sequence(line));
            }
        }
    }


    pub fn process_rules(&mut self) {

        for rule in &self.rules {
            if ! self.hash_rules.get(&rule.0).is_some() {
                self.hash_rules.insert(rule.0, Vec::new());
            }

            let hash_value = self.hash_rules.get_mut(&rule.0).unwrap();
            hash_value.push(rule.1);
        }
    }


    pub fn process_sequences(&self) -> Result<i32, ParseError> {
        let mut middle_sum = 0;
        for seq in &self.sequences {
            let mut valid = true;

            for i in 1..seq.len() {
                let curr = seq[i];
                let current_rule = self.hash_rules.get(&curr);
                
                // check if rules exist for the given value
                match current_rule {
                    // iterate ove the prev vals and check for rule breakage
                    Some(rule) => {
                        for j in 0..i {
                            // stop search if deviation is found
                            if rule.contains(&seq[j]) {
                                valid = false;
                                break;
                            }
                        }
                    },

                    // no rule found just continue
                    None => {
                        continue;
                    }
                }
                // check if the prev are contained in the current hash ruleset        
            }    

            if valid {
                middle_sum += seq[seq.len() / 2];
                let dbg_msg = format!("{} is valid", seq.iter().map(|x| x.to_string()+",").collect::<String>());
                dbg!(dbg_msg);
            }
        }

        dbg!(middle_sum);
        Ok(middle_sum)
    }

    

    fn parse_rule(&self, line: &str) -> (i32, i32) {
        // method to parse the rules and create respective entry
        dbg!(line);
        let rule: Vec<&str> = line.split("|").collect();
        let rule_l: i32 = rule[0].parse::<i32>().unwrap();
        let rule_r: i32 = rule[1].parse::<i32>().unwrap();

        return (rule_l, rule_r);
    }

    fn parse_sequence(&self, line: &str) -> Vec<i32> {
        // method to parse the sequence and create respective entry
        let sequence: String = line
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>();

        let token: Vec<&str> = sequence
            .split(",")
            .collect();

        return token.iter().map(|x| x.parse::<i32>().unwrap()).collect();
    }
}

fn main() {

    // let path = Path::new("inputs/day05.txt");    
    // let path = Path::new("test_inputs/day05.txt");

    // let mut cp = ContentParser::new(String::from("test_inputs/day05.txt"));
    let mut cp = ContentParser::new(String::from("inputs/day05.txt"));

    cp.solve_1();


    return;
}