use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[derive(PartialEq, Clone)]
enum Shape {
    Rock,
    Paper,
    Scissors
}

#[derive(PartialEq, Clone)]
enum Outcome {
    Won,
    Draw,
    Lost
}

impl Outcome {
    pub fn new(c : char) -> Outcome {
        match c {
            'X' => Outcome::Lost,
            'Y' => Outcome::Draw,
            'Z' => Outcome::Won,
            _ => panic!("Invalid character")
        }
    }

    pub fn value(self) -> i32{
        match self {
            Outcome::Lost => 0,
            Outcome::Draw => 3,
            Outcome::Won => 6
        }
    }
}

impl Shape {
    pub fn new(c : char) -> Shape {
        match c {
            'A' | 'X' => Shape::Rock,
            'B' | 'Y' => Shape::Paper,
            'C' | 'Z' => Shape::Scissors,
            _ => panic!("Unexpected character")
        }
    }

    pub fn new_from_outcome(opponent_shape : Shape, out : Outcome) -> Shape {
        match out {
            Outcome::Lost => Shape::get_loosing_shape(opponent_shape),
            Outcome::Draw => opponent_shape,
            Outcome::Won  => Shape::get_winning_shape(opponent_shape) 
        }
    }

    pub fn value(self) -> i32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3
        }
    }

    pub fn get_loosing_shape(s : Shape) -> Shape {
        match s {
            Shape::Rock => Shape::Scissors,
            Shape::Paper => Shape::Rock,
            Shape::Scissors => Shape::Paper
        }
    }

    pub fn get_winning_shape(s : Shape) -> Shape {
        match s {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissors,
            Shape::Scissors => Shape::Rock
        }
    }

    pub fn calculate_outcome(self, other : Shape) -> Outcome {
        if self == other {
            Outcome::Draw
        }
        else {
            let won = (self == Shape::Rock && other == Shape::Scissors) ||
                (self == Shape::Paper && other == Shape::Rock) ||
                (self == Shape::Scissors && other == Shape::Paper);
            match won {
                true => Outcome::Won,
                false => Outcome::Lost
            }
        }
    }

    pub fn play(self, other : Shape)  -> i32 {
        let outcome = self.clone().calculate_outcome(other);
        outcome.value() + self.value()
    }
}

pub fn day2_pt1() {
    let file = File::open("inputs/day2.txt").unwrap();
    let reader = BufReader::new(file);

    let score : i32 = reader.lines()
        .map(|line| line.unwrap().chars().collect::<Vec<char>>())
        .map(|chars| (Shape::new(chars[0]), Shape::new(chars[2])))
        .map(|(opponent_shape, my_shape)| my_shape.play(opponent_shape))
        .sum();
    println!("{}", score);
}


pub fn day2_pt2() {
    let file = File::open("inputs/day2.txt").unwrap();
    let reader = BufReader::new(file);

    let score : i32 = reader.lines()
        .map(|line| line.unwrap().chars().collect::<Vec<char>>())
        .map(|chars| (Shape::new(chars[0]), Outcome::new(chars[2])))
        .map(|(opponent_shape, expected_outcome)| (opponent_shape.clone(), Shape::new_from_outcome(opponent_shape, expected_outcome)))
        .map(|(opponent_shape, my_shape)| my_shape.play(opponent_shape))
        .sum();
    println!("{}", score);
}