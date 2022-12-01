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
    let data: Vec<Record> = parser::records_from_lines("data/day01.example");
    println!("Example Result PART 1: {}", part1(&data));
    println!("Example Result PART 2: {}", part2(&data));

    let data: Vec<Record> = parser::records_from_lines("data/day01.txt");
    println!("Final Result PART 1: {}", part1(&data));
    println!("Final Result PART 2: {}", part2(&data));
}

fn make_elves(data: &Vec<Record>) -> HashMap<usize, u32> {
    let mut elf_calories = HashMap::new();
    let mut elf: usize = 0;

    for d in data {
        match d.calories {
            Some(c) => {
                let cal_count = elf_calories.entry(elf).or_insert(0u32);
                *cal_count += c;
            }
            None => elf += 1,
        }
    }
    elf_calories
}

fn part1(data: &Vec<Record>) -> u32 {
    let elf_calories = make_elves(data);
    let mut calorie_list: Vec<u32> = elf_calories.values().cloned().collect();
    calorie_list.sort_by(|a, b| b.cmp(a));
    calorie_list[0]
}

fn part2(data: &Vec<Record>) -> u32 {
    let elf_calories = make_elves(data);
    let mut calorie_list: Vec<u32> = elf_calories.values().cloned().collect();
    calorie_list.sort_by(|a, b| b.cmp(a));
    calorie_list[0] + calorie_list[1] + calorie_list[2]
}
