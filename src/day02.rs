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
    Win,
    Loss,
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
}

#[derive(Debug)]
struct Record {
    opponent: Shape,
    response: Shape,
}

impl FromStr for Record {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(" ");
        let parts: Vec<&str> = parts.collect();
        Ok(Record {
            opponent: Shape::from_str(parts.get(0).expect("bad opponent").trim())
                .expect("invalid shape (ABC)"),
            response: Shape::from_str(parts.get(1).expect("bad response").trim())
                .expect("invalid shape (XYZ)"),
        })
    }
}

impl Record {
    fn evaluate(&self) -> RoundResult {
        match self.response {
            Shape::Rock => match self.opponent {
                Shape::Rock => RoundResult::Draw,
                Shape::Paper => RoundResult::Loss,
                Shape::Scissors => RoundResult::Win,
            },
            Shape::Paper => match self.opponent {
                Shape::Rock => RoundResult::Win,
                Shape::Paper => RoundResult::Draw,
                Shape::Scissors => RoundResult::Loss,
            },
            Shape::Scissors => match self.opponent {
                Shape::Rock => RoundResult::Loss,
                Shape::Paper => RoundResult::Win,
                Shape::Scissors => RoundResult::Draw,
            },
        }
    }

    fn value(&self) -> u32 {
        match self.response {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}

pub fn solve() {
    let data: Vec<Record> = parser::records_from_lines("data/day02.example");
    println!("Example Result DAY 02 PART 1: {}", part1(&data));
    // println!("Example Result DAY 02 PART 2: {}", part2(&data));

    let data: Vec<Record> = parser::records_from_lines("data/day02.txt");
    println!("Final Result DAY 02 PART 1: {}", part1(&data));
    // println!("Final Result DAY 02 PART 2: {}", part2(&data));
}

fn part1(data: &Vec<Record>) -> u32 {
    let mut sum = 0;
    for d in data {
        sum += d.value() + d.evaluate().value();
    }
    sum
}
