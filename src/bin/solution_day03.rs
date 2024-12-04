use std::fs;
use std::path::Path;


fn main() {
    let path = Path::new("inputs/day03_ex1.txt");
    // let path = Path::new("test_inputs/day03_test.txt");

    let content = fs::read_to_string(path)
        .expect("Error reading file");

    let outer_chars = ['m', 'u', 'l', '(', ',', ')'];
    let mut outer_index = 0;

    let mut curr_val: Vec<String> = vec![String::new(), String::new()];
    let mut curr_val_index = 0;

    let mut total: i32 = 0;

    for (idx, i) in content.chars().enumerate() {

        // checks if the value is in the correct values
        if i == outer_chars[outer_index] 
        {
            // println!("{}: {}", outer_index, i);
            if outer_index == 4 // -> ,
            {
                curr_val_index += 1;
            } 
            else if  outer_index == 5 // -> )
            {
                // terminal state is reached
                // calculate the product of the two values
                let val_l: i32 = curr_val[0].parse().unwrap();
                let val_r: i32 = curr_val[1].parse().unwrap();
                let product = val_l * val_r; 
                total +=  product;

                println!("{} * {} = {} \t total: {}", val_l, val_r, product, total);
                outer_index = 0;
                curr_val_index = 0;
                curr_val = vec![String::new(), String::new()];

                continue;
            }

            outer_index += 1;

        } else {
            // println!("{}, {}", i, outer_index );
            if i.is_numeric() && (outer_index == 4 || outer_index == 5)
            {
                println!("{}", idx);
                curr_val[curr_val_index].push(i);

            } else {

                outer_index = 0;
                curr_val_index = 0;
                curr_val = vec![String::new(), String::new()]

            }


        }

    }

    println!("{}", total);    
    
    return
}