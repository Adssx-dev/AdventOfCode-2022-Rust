use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::str::Chars;

pub fn day3_pt1() -> i32 {
    let file = File::open("inputs/day3.txt").unwrap();
    let reader = BufReader::new(file);

    reader.lines()
        .map(|l| l.unwrap()) // Unwrap Result<Rtring>
        .map(|l| (l[0..(l.len()/2)].to_owned(), l[(l.len()/2)..l.len()].to_owned())) // Split strings in two equal parts
        .map(|(s1, s2)| s1.chars().into_iter() // Iterate over all chars from first half
            .filter(|c_in_str1| s2.clone().chars().any(|c_in_str2| *c_in_str1 == c_in_str2)) // Keep only characters present in both strings
            .next()) // Previous line is supposed to return only 1 char, so we take the first one
        .map(|char| char.unwrap() as i32 - 'a' as i32) // Convert to integer
        .map(|ch| match ch { // Calculate priority
            c if c >= 0 => c + 1, // Map lower case from [0-25] to [1-26]
            c if c < 0 => c + 59, // Map upper case characters from [-32, -7] to [27-52]
            _ => panic!("unreachable")
        })
        .sum()
}

pub fn day3_pt2() -> i32 {
    let file = File::open("inputs/day3.txt").unwrap();
    let reader = BufReader::new(file);

    // Convert to a vector as we need to access elements by group of 3 and I could not find a way to do this with iterators
    let tmp_vec = reader.lines()
        .map(|l| l.unwrap())
        .collect::<Vec<String>>();

    let mut sum = 0;

    for i in 0..tmp_vec.len() / 3{
        let idx = 3 * i;
        let value = tmp_vec[idx].chars().into_iter()
            .filter(|c_in_str1| tmp_vec[idx + 1].chars().into_iter().any(|c_in_str2| *c_in_str1 == c_in_str2)) // Keep only chars present also in string 2
            .filter(|c_in_str1| tmp_vec[idx + 2].chars().into_iter().any(|c_in_str3| *c_in_str1 == c_in_str3)) // Keep only chars present in string 3
            .next() // We should have only 1 char by now
            .map(|ch| ch as i32 - 'a' as i32) // Same conversion as before
            .map(|ch| match ch {
                c if c >= 0 => c + 1,
                c if c < 0 => c + 59,
                _ => panic!("unreachable")
            });
        sum = sum + value.unwrap();
    }

    sum
}