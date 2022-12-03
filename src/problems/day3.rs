use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::str::Chars;

pub fn day3_pt1() {
    let file = File::open("inputs/day3.txt").unwrap();
    let reader = BufReader::new(file);

    let x : i32= reader.lines()
        .map(|l| l.unwrap())
        .map(|l| (l[0..(l.len()/2)].to_owned(), l[(l.len()/2)..l.len()].to_owned()))
        .map(|(s1, s2)| s1.chars().into_iter()
            .filter(|c_in_str1| s2.clone().chars().any(|c_in_str2| *c_in_str1 == c_in_str2))
            .next())
        .map(|char| char.unwrap() as i32 - 'a' as i32)
        .map(|ch| match ch {
            c if c >= 0 => c + 1,
            c if c < 0 => c + 59,
            _ => panic!("unreachable")
        })
        .sum();
    println!("{}", x)
}

pub fn day3_pt2() {
    let file = File::open("inputs/day3.txt").unwrap();
    let reader = BufReader::new(file);

    let tmp_vec = reader.lines()
        .map(|l| l.unwrap())
        .map(|s| s)
        .collect::<Vec<String>>();

    let mut sum = 0;

    for i in 0..tmp_vec.len() / 3{
        let idx = 3 * i;
        let value = tmp_vec[idx].chars().into_iter()
            .filter(|c_in_str1| tmp_vec[idx + 1].chars().into_iter().any(|c_in_str2| *c_in_str1 == c_in_str2))
            .filter(|c_in_str1| tmp_vec[idx + 2].chars().into_iter().any(|c_in_str3| *c_in_str1 == c_in_str3))
            .next()
            .map(|ch| ch as i32 - 'a' as i32)
            .map(|ch| match ch {
                c if c >= 0 => c + 1,
                c if c < 0 => c + 59,
                _ => panic!("unreachable")
            });
        sum = sum + value.unwrap();
                
    }

    println!("{}", sum)
}