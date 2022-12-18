use std::{
    collections::{HashMap, HashSet},
    fs,
    ops::Add,
};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Cube3 {
    x: i32,
    y: i32,
    z: i32,
}

impl Add for Cube3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

fn parse_cubes(raw: &str) -> HashSet<Cube3> {
    let cubes: Vec<Cube3> = raw
        .lines()
        .map(|line| {
            let parts: Vec<i32> = line
                .split(",")
                .collect::<Vec<&str>>()
                .iter()
                .map(|p| p.parse::<i32>().unwrap())
                .collect();
            Cube3 {
                x: parts[0],
                y: parts[1],
                z: parts[2],
            }
        })
        .collect();
    HashSet::from_iter(cubes.into_iter())
}

fn part1(raw: &str) {
    let cubes = parse_cubes(raw);
    let mut free_faces = HashMap::new();

    let directions = [
        Cube3 { x: 1, y: 0, z: 0 },
        Cube3 { x: -1, y: 0, z: 0 },
        Cube3 { x: 0, y: 1, z: 0 },
        Cube3 { x: 0, y: -1, z: 0 },
        Cube3 { x: 0, y: 0, z: 1 },
        Cube3 { x: 0, y: 0, z: -1 },
    ];

    for cube in &cubes {
        for dir in &directions {
            let consider = *cube + *dir;
            let mut count = free_faces.entry(*cube).or_insert(0);
            *count += 1 - cubes.contains(&consider) as i32;
        }
    }

    let sum: i32 = free_faces.values().sum();
    println!("Part 1: {}", sum);
}

pub fn solve() {
    let raw = fs::read_to_string("data/day18.example").unwrap();
    part1(&raw);

    let raw = fs::read_to_string("data/day18.txt").unwrap();
    part1(&raw);
}
