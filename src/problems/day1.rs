use std::fs::File;
use std::io::{prelude::*, BufReader};

// Loads the data into a vector, each element is the sum of calories for each elf
pub fn load_data() -> Vec<i32> {
    let mut vec = vec!(0);

    let file = File::open("inputs/day1.txt").unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(str) => match str.as_str() { 
                "" => vec.push(0),
                number_as_str => {
                    *(vec.last_mut().unwrap()) = vec.last().unwrap() + number_as_str.parse::<i32>().unwrap();
                }
            }
            Err(_) => panic!("Could not load file"),
        }
    }
    vec
}

pub fn day1_pt1() -> i32 {
    let vec = load_data();

    // Get the maximum of calories <=> maximum of the list
    *vec.iter().max().unwrap()
}

pub fn day1_pt2() -> i32 {
    let mut vec = load_data();

    
    vec.sort_by(|a, b| b.cmp(a)); // Sort descending
                                             
                                             

    vec.iter()
        .take(3)  // Take first 3 elements
        .sum()  // sum them
}

#[cfg(test)]
mod tests {
    use crate::problems::day1::*;

    #[test]
    fn day1_pt1_test() {
        let result = day1_pt1();
        assert_eq!(result, 71506);
    }

    #[test]
    fn day1_pt2_test() {
        let result = day1_pt2();
        assert_eq!(result, 209603);
    }
}
