use std::fs::File;
use std::io::{self, prelude::*, BufReader};

struct SectionAssignments {
    start : i32, // inclusive
    end : i32 //inclusive
}

impl SectionAssignments {
    // Construct a SectionAssignment from an iterator, by only taking first 2 elements
    pub fn from_iter(mut iter : impl Iterator<Item = i32>) -> SectionAssignments {
        SectionAssignments {
            start:iter.next().unwrap().clone(),
            end:iter.next().unwrap().clone()
        }
    }

    // Determines wheter *any of the two assignments* fully contains the other
    pub fn fully_contains(&self, other : &SectionAssignments) -> bool {
        (self.start <= other.start && self.end >= other.end) ||
            (other.start <= self.start && other.end >= self.end)
    }
    
    // Determines if there is an overlap between the two assignments
    // <=> 1D collision
    pub fn overlap(&self, other : &SectionAssignments) -> bool {
        (self.start >= other.start && self.start <= other.end) ||
            (self.end >= other.start && self.end <= other.end) ||
            self.fully_contains(other)
    }

}

// Convert a line to a tuple of SectionAssignments
fn line_to_sections(line : &str) -> (SectionAssignments, SectionAssignments) {
    let mut l = line.split(',')
        .map(|range| SectionAssignments::from_iter(
            range.split('-')
            .map(|chars| chars.parse::<i32>().unwrap()) // Convert strings to their numerical value
            .take(2))) // There should be only 2 elements : before and after the "-"
        .take(2); // There should be only 2 SectionAssignments : before and after the ","
    (l.next().unwrap(), l.next().unwrap())    
}

pub fn day4_pt1() -> i32 {
    let file = File::open("inputs/day4.txt").unwrap();
    let reader = BufReader::new(file);

    reader.lines()
        .map(|l| l.unwrap())
        .map(|line| line_to_sections(&line)) // Convert lines to section assignments
        .filter(|(assign1, assing2)| assign1.fully_contains(assing2)) // Keep only lines that contains sections that fully overlap
        .count() as i32 // Calculate the number of matching lines
}


pub fn day4_pt2() -> i32 {
    let file = File::open("inputs/day4.txt").unwrap();
    let reader = BufReader::new(file);

    reader.lines()
        .map(|l| l.unwrap())
        .map(|line| line_to_sections(&line)) // Convert lines to section assignments
        .filter(|(assign1, assing2)| assign1.overlap(assing2)) // Keep only lines that contains sections that overlap
        .count() as i32 // Calculate the number of matching lines
}