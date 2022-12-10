use crate::parser;
use std::{fs, num::ParseIntError, str::FromStr};

#[derive(Debug)]
enum Keyword {
    noop,
    addx,
}

#[derive(Debug)]
struct Instruction {
    keyword: Keyword,
    value: Option<i32>,
    cycles: i32,
}

#[derive(Debug)]
struct State {
    X: i32,
}

impl FromStr for Instruction {
    type Err = ParseIntError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = line.split(" ").collect();
        if parts.len() == 1 {
            return Ok(Instruction {
                keyword: Keyword::noop,
                value: None,
                cycles: 1,
            });
        }
        Ok(Instruction {
            keyword: Keyword::addx,
            value: Some(parts[1].parse().unwrap()),
            cycles: 2,
        })
    }
}

impl Instruction {
    fn end_cycle(&self, cycle: i32, state: &mut State) {
        match self.keyword {
            Keyword::addx => {
                if cycle == self.cycles {
                    state.X += self.value.unwrap();
                }
            }
            _ => (),
        }
    }
}

pub fn solve() {
    let program: Vec<Instruction> = parser::records_from_lines("data/day10.example2");
    part1(&program);

    let program: Vec<Instruction> = parser::records_from_lines("data/day10.txt");
    part1(&program);
}

fn part1(program: &Vec<Instruction>) {
    let mut state = State { X: 1 };
    let mut cycle = 0;
    let mut signal_strength = Vec::new();

    for inst in program {
        for inst_cycle in 0..inst.cycles {
            cycle += 1;
            // println!("During {}: {:?} | {:?}", cycle, inst, state);

            let is_interesting = ((cycle + 20) % 40) == 0;
            if is_interesting {
                // println!("During {}: {:?} | {:?}", cycle, inst, state);
                signal_strength.push(state.X * cycle);
            }

            let cursor = (cycle - 1) % 40;
            if state.X - 1 == cursor || state.X == cursor || state.X + 1 == cursor {
                print!("#");
            } else {
                print!(".");
            }

            if cycle % 40 == 0 {
                println!();
            }

            inst.end_cycle(inst_cycle + 1, &mut state);
            // println!("After  {}: {:?} | {:?}", cycle, inst, state);
        }
    }

    println!("Part 1: {}", signal_strength.iter().sum::<i32>());
}
