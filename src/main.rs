use clap::Parser;
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, EnumString};

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
// NEXTMOD
mod parser;

#[derive(Debug, Clone, PartialEq, EnumString, EnumIter)]
enum Days {
    All,
    Day01,
    Day02,
    Day03,
    Day04,
    Day05,
    Day06,
    Day09,
    Day10,
    Day11,
    Day12,
    Day13,
    Day14,
    Day15,
    Day16,
    Day17,
    Day18,
    Day19,
    Day20,
    Day21,
    Day22,
    Day23,
    Day24,
    Day25,
    // NEXTENUM
}

#[derive(Parser)]
struct Cli {
    day: Days,
}

impl Days {
    fn run_one(&self) {
        println!("running {:?}", self);
        match self {
            Days::Day01 => day01::solve(),
            Days::Day02 => day02::solve(),
            Days::Day03 => day03::solve(),
            Days::Day04 => day04::solve(),
            Days::Day05 => day05::solve(),
            Days::Day06 => day06::solve(),
            Days::Day09 => day09::solve(),
            Days::Day10 => day10::solve(),
            Days::Day11 => day11::solve(),
            Days::Day12 => day12::solve(),
            Days::Day13 => day13::solve(),
            Days::Day14 => day14::solve(),
            Days::Day15 => day15::solve(),
            Days::Day16 => day16::solve(),
            Days::Day17 => day17::solve(),
            Days::Day18 => day18::solve(),
            Days::Day19 => day19::solve(),
            Days::Day20 => day20::solve(),
            Days::Day21 => day21::solve(),
            Days::Day22 => day22::solve(),
            Days::Day23 => day23::solve(),
            Days::Day24 => day24::solve(),
            Days::Day25 => day25::solve(),
            // NEXTMATCH
            _ => panic!("Requested day [{:?}] not implemented!", self),
        }
    }

    fn run_all() {
        println!("running all");
        for day in Days::iter() {
            match day {
                Days::All => continue,
                _ => (),
            }
            day.run_one();
        }
    }

    fn run(&self) {
        match self {
            Days::All => Days::run_all(),
            _ => self.run_one(),
        }
    }
}

fn main() {
    let args = Cli::parse();
    args.day.run();
}
