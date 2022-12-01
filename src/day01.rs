use std::{collections::HashMap, num::ParseIntError, str::FromStr};

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
    calories: Option<u32>,
}

impl FromStr for Record {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Record {
            calories: match s.parse() {
                Ok(c) => Some(c),
                Err(_) => None,
            },
        })
    }
}

pub fn solve() {
    println!("Part 1");

    let data: Vec<Record> = parser::records_from_lines("data/day01.example");
    println!("Example Result: {}", part1(&data));

    let data: Vec<Record> = parser::records_from_lines("data/day01.txt");
    println!("Final Result: {}", part1(&data));
}

fn part1(data: &Vec<Record>) -> u32 {
    let mut elf_calories = HashMap::new();
    let mut elf: usize = 0;
    let mut max_calorie_elf: usize = 0;

    for d in data {
        match d.calories {
            Some(c) => {
                let cur_max = elf_calories.get(&max_calorie_elf).copied().unwrap_or(0);
                let cal_count = elf_calories.entry(elf).or_insert(0u32);
                *cal_count += c;
                if *cal_count > cur_max {
                    max_calorie_elf = elf;
                }
            }
            None => elf += 1,
        }
    }
    elf_calories.get(&max_calorie_elf).copied().unwrap_or(0)
}
