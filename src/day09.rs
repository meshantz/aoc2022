use crate::parser;
use std::{collections::HashSet, num::ParseIntError, str::FromStr};
use strum_macros::EnumString;

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
struct Move {
    direction: Direction,
    amount: u8,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Vector2 {
    x: i32,
    y: i32,
}

impl FromStr for Move {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(" ");
        let parts: Vec<&str> = parts.collect();
        Ok(Move {
            direction: Direction::from_str(parts.get(0).expect("bad move direction").trim())
                .expect("invalid direction"),
            amount: parts
                .get(1)
                .expect("bad move amount")
                .trim()
                .parse()
                .expect("invalid amount"),
        })
    }
}

impl Vector2 {
    fn new() -> Vector2 {
        Vector2 { x: 0, y: 0 }
    }

    fn from(x: i32, y: i32) -> Vector2 {
        Vector2 { x, y }
    }

    fn move_by(&mut self, other: &Vector2) {
        self.x += other.x;
        self.y += other.y;
    }

    fn follow(&mut self, other: &Vector2) {
        if self.touching(other) {
            return ();
        }
        let delta = other.subtract(self);
        if delta.x == 0 {
            // follow in y
            if delta.y > 0 {
                self.move_by(&Vector2::from(0, 1));
            } else {
                self.move_by(&Vector2::from(0, -1));
            }
        } else if delta.y == 0 {
            // follow in x
            if delta.x > 0 {
                self.move_by(&Vector2::from(1, 0));
            } else {
                self.move_by(&Vector2::from(-1, 0));
            }
        } else {
            let vx = if delta.x > 0 { 1 } else { -1 };
            let vy = if delta.y > 0 { 1 } else { -1 };
            self.move_by(&Vector2::from(vx, vy));
        }
    }

    fn touching(&self, other: &Vector2) -> bool {
        let touching_vectors = [
            Vector2::from(-1, -1),
            Vector2::from(-1, 0),
            Vector2::from(-1, 1),
            Vector2::from(0, -1),
            Vector2::from(0, 0),
            Vector2::from(0, 1),
            Vector2::from(1, -1),
            Vector2::from(1, 0),
            Vector2::from(1, 1),
        ];
        for v in touching_vectors {
            if self.add(&v) == *other {
                return true;
            }
        }
        false
    }

    fn add(&self, other: &Vector2) -> Vector2 {
        Vector2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    fn subtract(&self, other: &Vector2) -> Vector2 {
        Vector2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Direction {
    fn unit_vector(&self) -> Vector2 {
        match self {
            Direction::Down => Vector2::from(0, -1),
            Direction::Up => Vector2::from(0, 1),
            Direction::Left => Vector2::from(-1, 0),
            Direction::Right => Vector2::from(1, 0),
        }
    }
}

pub fn solve() {
    part1("data/day09.example");
    part1("data/day09.txt");
}

fn part1(filename: &str) {
    let data: Vec<Move> = parser::records_from_lines(filename);
    let mut head = Vector2::new();
    let mut tail = Vector2::new();
    let mut visited = HashSet::new();

    visited.insert(Vector2::from(tail.x, tail.y));

    for move_ in data {
        // println!("{:?}", move_);
        for _ in 0..move_.amount {
            head.move_by(&move_.direction.unit_vector());
            tail.follow(&head);
            visited.insert(Vector2::from(tail.x, tail.y));

            // println!(
            //     ".. move 1 {:?}: H={:?} T={:?} T visited: {}",
            //     move_.direction,
            //     head,
            //     tail,
            //     visited.len()
            // );
        }
    }

    println!("Part 1: {}", visited.len());
}
