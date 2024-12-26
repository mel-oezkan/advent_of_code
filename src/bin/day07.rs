use std::fs;

/*
oooo
....
xxoo
xoxo
xoox
xxxo
xxox
xoxx
xxxx
....
oxxo
oxox
oxxx
....
ooxx

following function creates the possible combinations of the elements
*/

fn generate_patterns(pattern_len: usize) -> Vec<String> {
    let mut patterns = Vec::new();
    
    fn generate_recursive(current: String, patterns: &mut Vec<String>, pattern_len: usize) {
        if current.len() == pattern_len {
            patterns.push(current);
            return;
        }
        
        generate_recursive(current.clone() + "*", patterns, pattern_len);
        generate_recursive(current.clone() + "+", patterns, pattern_len);
    }
    
    generate_recursive(String::new(), &mut patterns, pattern_len);
    patterns
}


fn main() {
    // let path = String::from("test_inputs/day07.txt");
    let path = String::from("inputs/day07.txt");

    let content = fs::read_to_string(path)
        .expect("Error reading file");


    let mut result: i64 = 0;

    for line in content.lines() {
        // println!("Line: {}", line);
        let values = 
            line
                .split_whitespace()
                .map(|x| 
                        String::from(x.trim_end_matches(":"))
                            .parse::<i64>()
                            .expect("Inavalid value"))
                .collect::<Vec<i64>>();

        // use plus and product to chekc if the elements after 0 can be
        // constructed from the elements after 0
        
        // brute force solution and start with * then replace +
        let pattern_len = values.len() - 1;
        let patterns = generate_patterns(pattern_len);

        for pattern in patterns {
            let mut current = values.clone();
            let target = current.remove(0);

            // update the next value from the pattern
            // replace the next value by the running value
            for (i, c) in pattern.chars().enumerate() {
                if i == current.len()-1 {
                    break;
                }

                if c == '*' {
                    current[i+1] = current[i] * current[i+1];
                } else {
                    current[i+1] = current[i] + current[i+1];
                }

                if current[i+1] > target {
                    // println!("Invalid Pattern: {}", pattern);
                    break;
                }
            }

            if current.last().unwrap() == &target {
                println!("Pattern Matched: {}", pattern);
                result += target;
                break;
            } else {
                // println!("Solution too small: {} = {}", pattern, current.last().unwrap());  
            }
            
        }
        // println!();
    }

    println!("Result: {}", result);


}