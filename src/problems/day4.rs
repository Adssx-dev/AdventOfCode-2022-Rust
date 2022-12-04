use std::fs::File;
use std::io::{self, prelude::*, BufReader};

struct SectionAssignments {
    start : u32, // inclusive
    end : u32 //inclusive
}

impl SectionAssignments {
    pub fn from_iter(mut iter : impl Iterator<Item = u32>) -> SectionAssignments {
        SectionAssignments {
            start:iter.next().unwrap().clone(),
            end:iter.next().unwrap().clone()
        }
    }

    pub fn fully_contains(&self, other : &SectionAssignments) -> bool {
        (self.start <= other.start && self.end >= other.end) ||
            (other.start <= self.start && other.end >= self.end)
    }
    

    pub fn overlap(&self, other : &SectionAssignments) -> bool {
        (self.start >= other.start && self.start <= other.end) ||
            (self.end >= other.start && self.end <= other.end) ||
            self.fully_contains(other)
    }

}

fn line_to_sections(line : &str) -> (SectionAssignments, SectionAssignments) {
    let mut l = line.split(',')
        .map(|range| SectionAssignments::from_iter(
            range.split('-')
            .map(|chars| chars.parse::<u32>().unwrap())
            .take(2)))
        .take(2);
    (l.next().unwrap(), l.next().unwrap())    
}

pub fn day4_pt1() {
    let file = File::open("inputs/day4.txt").unwrap();
    let reader = BufReader::new(file);

    let overlapped = reader.lines()
        .map(|l| l.unwrap())
        .map(|line| line_to_sections(&line))
        .filter(|(assign1, assing2)| assign1.fully_contains(assing2))
        .count();

    println!("{}", overlapped);
}


pub fn day4_pt2() {
    let file = File::open("inputs/day4.txt").unwrap();
    let reader = BufReader::new(file);

    let overlapped = reader.lines()
        .map(|l| l.unwrap())
        .map(|line| line_to_sections(&line))
        .filter(|(assign1, assing2)| assign1.overlap(assing2))
        .count();

    println!("{}", overlapped);
}