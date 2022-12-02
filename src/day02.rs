use crate::parser;
use std::{num::ParseIntError, str::FromStr};
use strum_macros::EnumString;

#[derive(Debug, Clone, PartialEq, EnumString)]
enum Shape {
    #[strum(serialize = "A", serialize = "X")]
    Rock,
    #[strum(serialize = "B", serialize = "Y")]
    Paper,
    #[strum(serialize = "C", serialize = "Z")]
    Scissors,
}

#[derive(Debug, Clone, PartialEq, EnumString)]
enum RoundResult {
    #[strum(serialize = "Z")]
    Win,
    #[strum(serialize = "X")]
    Loss,
    #[strum(serialize = "Y")]
    Draw,
}

impl RoundResult {
    fn value(&self) -> u32 {
        match self {
            RoundResult::Loss => 0,
            RoundResult::Draw => 3,
            RoundResult::Win => 6,
        }
    }

    fn shape_to_get(&self, against: &Shape) -> Shape {
        match against {
            Shape::Rock => match self {
                RoundResult::Draw => Shape::Rock,
                RoundResult::Loss => Shape::Paper,
                RoundResult::Win => Shape::Scissors,
            },
            Shape::Paper => match self {
                RoundResult::Draw => Shape::Paper,
                RoundResult::Loss => Shape::Rock,
                RoundResult::Win => Shape::Scissors,
            },
            Shape::Scissors => match self {
                RoundResult::Draw => Shape::Scissors,
                RoundResult::Loss => Shape::Paper,
                RoundResult::Win => Shape::Rock,
            },
        }
    }
}

#[derive(Debug)]
struct RecordPart1 {
    opponent: Shape,
    response: Shape,
}

#[derive(Debug)]
struct RecordPart2 {
    opponent: Shape,
    response: RoundResult,
}

impl FromStr for RecordPart1 {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(" ");
        let parts: Vec<&str> = parts.collect();
        Ok(RecordPart1 {
            opponent: Shape::from_str(parts.get(0).expect("bad opponent").trim())
                .expect("invalid shape (ABC)"),
            response: Shape::from_str(parts.get(1).expect("bad response").trim())
                .expect("invalid shape (XYZ)"),
        })
    }
}

impl FromStr for RecordPart2 {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(" ");
        let parts: Vec<&str> = parts.collect();
        Ok(RecordPart2 {
            opponent: Shape::from_str(parts.get(0).expect("bad opponent").trim())
                .expect("invalid part 2 shape (ABC)"),
            response: RoundResult::from_str(parts.get(1).expect("bad response").trim())
                .expect("invalid response (XYZ)"),
        })
    }
}

trait Round {
    fn round_value(&self) -> u32;
}

fn evaluate(a: &Shape, b: &Shape) -> RoundResult {
    match b {
        Shape::Rock => match a {
            Shape::Rock => RoundResult::Draw,
            Shape::Paper => RoundResult::Loss,
            Shape::Scissors => RoundResult::Win,
        },
        Shape::Paper => match a {
            Shape::Rock => RoundResult::Win,
            Shape::Paper => RoundResult::Draw,
            Shape::Scissors => RoundResult::Loss,
        },
        Shape::Scissors => match a {
            Shape::Rock => RoundResult::Loss,
            Shape::Paper => RoundResult::Win,
            Shape::Scissors => RoundResult::Draw,
        },
    }
}

fn value(shape: &Shape) -> u32 {
    match shape {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissors => 3,
    }
}

impl RecordPart1 {
    fn evaluate(&self) -> RoundResult {
        evaluate(&self.opponent, &self.response)
    }

    fn value(&self) -> u32 {
        value(&self.response)
    }
}

impl Round for RecordPart1 {
    fn round_value(&self) -> u32 {
        self.value() + self.evaluate().value()
    }
}

impl Round for RecordPart2 {
    fn round_value(&self) -> u32 {
        value(&self.response.shape_to_get(&self.opponent)) + self.response.value()
    }
}

pub fn solve() {
    let filename = "data/day02.example";
    println!("Example Result DAY 02 PART 1: {}", part1(filename));
    println!("Example Result DAY 02 PART 2: {}", part2(filename));

    let filename = "data/day02.txt";
    println!("Final Result DAY 02 PART 1: {}", part1(filename));
    println!("Final Result DAY 02 PART 2: {}", part2(filename));
}

fn sum_rounds<T>(data: &Vec<T>) -> u64
where
    T: Round,
{
    let mut sum = 0;
    for d in data {
        sum += d.round_value() as u64;
    }
    sum
}

fn part1(filename: &str) -> u64 {
    let data: Vec<RecordPart1> = parser::records_from_lines(filename);
    sum_rounds(&data)
}

fn part2(filename: &str) -> u64 {
    let data: Vec<RecordPart2> = parser::records_from_lines(filename);
    sum_rounds(&data)
}
