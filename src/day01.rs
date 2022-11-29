use std::{num::ParseIntError, str::FromStr};

use strum_macros::EnumString;

use crate::parser;

#[derive(Debug, Clone, PartialEq, EnumString)]
enum Direction {
    #[strum(serialize = "U")]
    Up,
    #[strum(serialize = "D")]
    Down,
    #[strum(serialize = "L")]
    Left,
    #[strum(serialize = "R")]
    Right,
}

#[derive(Debug)]
struct Record {
    magnitude: i32,
    direction: Direction,
}

impl FromStr for Record {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(" ");
        let parts: Vec<&str> = parts.collect();
        Ok(Record {
            magnitude: parts
                .get(0)
                .expect("couldn't find integer part")
                .trim()
                .parse()?,
            direction: Direction::from_str(parts.get(1).expect("couldn't find string part").trim())
                .expect("invalid direction"),
        })
    }
}

pub fn solve() {
    let data: Vec<Record> = parser::records_from_lines("data/test.numbers");

    for d in data {
        println!("line in file was {:?}", d)
    }
}
