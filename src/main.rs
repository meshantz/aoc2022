use clap::Parser;
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, EnumString};

mod day01;
mod day02;
mod day03;
mod parser;

#[derive(Debug, Clone, PartialEq, EnumString, EnumIter)]
enum Days {
    All,
    Day01,
    Day02,
    Day03,
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
