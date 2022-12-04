use crate::parser;
use std::{num::ParseIntError, str::FromStr};

#[derive(Debug)]
struct ElfRange {
    start: u32,
    end: u32,
}

#[derive(Debug)]
struct Record {
    first: ElfRange,
    second: ElfRange,
}

impl FromStr for Record {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let elfs: Vec<&str> = s.split(",").collect();
        let elf1_range: Vec<&str> = elfs[0].split("-").collect();
        let elf2_range: Vec<&str> = elfs[1].split("-").collect();

        Ok(Record {
            first: ElfRange {
                start: elf1_range[0].parse().unwrap(),
                end: elf1_range[1].parse().unwrap(),
            },
            second: ElfRange {
                start: elf2_range[0].parse().unwrap(),
                end: elf2_range[1].parse().unwrap(),
            },
        })
    }
}

pub fn solve() {
    let data: Vec<Record> = parser::records_from_lines("data/day04.example");
    println!("Example Result PART 1: {}", part1(&data));
    println!("Example Result PART 2: {}", part2(&data));

    let data: Vec<Record> = parser::records_from_lines("data/day04.txt");
    println!("Final Result PART 1: {}", part1(&data));
    println!("Final Result PART 2: {}", part2(&data));
}

fn part1(data: &Vec<Record>) -> u32 {
    let mut sum: u32 = 0;
    for pair in data {
        if (pair.first.start >= pair.second.start && pair.first.end <= pair.second.end)
            || (pair.second.start >= pair.first.start && pair.second.end <= pair.first.end)
        {
            sum += 1;
        }
    }
    sum
}

fn part2(data: &Vec<Record>) -> u32 {
    let mut sum: u32 = 0;
    for pair in data {
        if (pair.first.end < pair.second.end || pair.first.start > pair.second.end)
            && (pair.second.end < pair.first.end || pair.second.start > pair.first.end)
        {
            continue;
        }
        sum += 1;
    }
    sum
}
