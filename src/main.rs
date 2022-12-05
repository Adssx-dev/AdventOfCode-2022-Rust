mod problems;

use problems::day1::*;
use problems::day2::*;
use problems::day3::*;
use problems::day4::*;
use problems::day5::*;


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
        println!("Day 5 part 1 : {}", day5_pt1());
        println!("Day 5 part 2 : {}", day5_pt2());
    }
    else {
        
    }
}
