use problems::day1::{self, day1_pt1};
use problems::day2::{day2_pt1, day2_pt2};

mod problems;
use crate::problems::day1::*;
use crate::problems::day2::*;
use crate::problems::day3::*;
use crate::problems::day4::*;



fn main() {
    let full = true;
    if full {
        println!("Day 1 part 1 : {}", day1_pt1());
        println!("Day 1 part 2 : {}", day1_pt2());
        println!("Day 2 part 1 : {}", day2_pt1());
        println!("Day 2 part 2 : {}", day2_pt2());
        println!("Day 3 part 1 : {}", day3_pt1());
        println!("Day 3 part 2 : {}", day3_pt2());
        println!("Day 4 part 1 : {}", day4_pt1());
        println!("Day 4 part 2 : {}", day4_pt2());
    }
    else {

    }
}
