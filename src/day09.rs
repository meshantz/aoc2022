use crate::parser;
use std::{
    collections::{HashMap, HashSet},
    num::ParseIntError,
    str::FromStr,
};
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

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
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
    part1("data/day09.example", false);
    part1("data/day09.txt", false);
    part2("data/day09.example2", false);
    part2("data/day09.txt", false);
}

fn part1(filename: &str, show_steps: bool) {
    let data: Vec<Move> = parser::records_from_lines(filename);
    let visited = visit(&data, 2, show_steps);
    println!("Part 1: {}", visited.len());
}

fn part2(filename: &str, show_steps: bool) {
    let data: Vec<Move> = parser::records_from_lines(filename);
    let visited = visit(&data, 10, show_steps);
    println!("Part 1: {}", visited.len());
}

fn visit(data: &Vec<Move>, rope_size: usize, show_steps: bool) -> HashSet<Vector2> {
    let mut visited = HashSet::new();
    let mut rope = Vec::new();

    for _ in 0..rope_size {
        rope.push(Vector2::new());
    }

    visited.insert(Vector2::from(0, 0));

    for move_ in data {
        // println!("{:?}", move_);
        for _ in 0..move_.amount {
            rope[0].move_by(&move_.direction.unit_vector());
            for i in 1..rope_size {
                let leader = rope[i - 1].clone();
                rope[i].follow(&leader);
            }
            let tail = rope.last().unwrap();
            visited.insert(Vector2::from(tail.x, tail.y));

            // println!(
            //     ".. move 1 {:?}: H={:?} T={:?} T visited: {}",
            //     move_.direction,
            //     head,
            //     tail,
            //     visited.len()
            // );
            if show_steps {
                show_state(&rope)
            }
        }
    }

    visited
}

fn show_state(rope: &Vec<Vector2>) {
    let mut rope_map = HashMap::new();
    // reverse so that the end of the rope is covered
    for (i, knot) in rope.iter().enumerate().rev() {
        rope_map.insert(Vector2::from(knot.x, knot.y), i);
    }

    for j in (-5..15).rev() {
        for i in -11..14 {
            let v = rope_map.get(&Vector2::from(i, j));
            match v {
                Some(0) => print!("H"),
                Some(10) => print!("T"),
                Some(i) => print!("{}", i),
                None => print!("."),
            }
        }
        println!();
    }
    println!();
}
