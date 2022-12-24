use std::{
    collections::HashSet,
    fs,
    ops::{Add, Mul, Rem, Sub},
    option,
};

const MOVE: [Pos; 4] = [
    Pos { x: 0, y: -1 },
    Pos { x: 0, y: 1 },
    Pos { x: -1, y: 0 },
    Pos { x: 1, y: 0 },
];

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Pos {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Blizzard {
    pos: Pos,
    dir: Dir,
}

impl Add for Pos {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Pos {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Rem for Pos {
    type Output = Self;

    fn rem(self, other: Self) -> Self {
        Self {
            x: self.x.rem_euclid(other.x),
            y: self.y.rem_euclid(other.y),
        }
    }
}

impl Mul<i32> for Pos {
    type Output = Self;

    fn mul(self, scalar: i32) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl Blizzard {
    fn at(self, minute: i32, valley: Pos) -> Pos {
        let correction = Pos { x: 1, y: 1 };
        let new_pos = self.pos - correction;
        let new_pos = new_pos + MOVE[self.dir as usize] * minute;
        let new_pos = new_pos % valley;
        new_pos + correction
    }
}

fn parse(raw: &str) -> (Pos, HashSet<Blizzard>) {
    let mut blizzards = HashSet::new();
    let mut valley = Pos { x: 0, y: 0 };
    for (y, line) in raw.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            let dir: Option<Dir> = match char {
                '^' => Some(Dir::Up),
                'v' => Some(Dir::Down),
                '<' => Some(Dir::Left),
                '>' => Some(Dir::Right),
                _ => None,
            };
            match dir {
                Some(dir) => {
                    let pos = Pos {
                        x: x as i32,
                        y: y as i32,
                    };
                    blizzards.insert(Blizzard { pos, dir });
                }
                None => (),
            }
            valley.x = valley.x.max(x as i32);
        }
        valley.y = valley.y.max(y as i32);
    }
    let valley = valley - Pos { x: 1, y: 1 };
    (valley, blizzards)
}

fn all_moves(elf_pos: Pos, blizzards: &HashSet<Pos>, valley: Pos, target: Pos) -> Vec<Pos> {
    let mut move_options = Vec::new();

    if !blizzards.contains(&elf_pos) {
        move_options.push(elf_pos); // can stay where you are
    }
    for dir in 0..4 {
        let possible = elf_pos + MOVE[dir];
        if possible == target {
            move_options.push(possible);
            return move_options; // short-circuit... who cares about the rest.
        }
        if blizzards.contains(&possible) {
            continue;
        }
        if possible.x < 1 || possible.x > valley.x || possible.y < 1 || possible.y > valley.y {
            continue;
        }
        move_options.push(possible);
    }

    move_options
}

fn draw_valley(valley: Pos, blizzards: &HashSet<Pos>) {
    for y in 0..valley.y + 2 {
        for x in 0..valley.x + 2 {
            if x < 1 || x > valley.x || y < 1 || y > valley.y {
                print!("#");
            } else {
                match blizzards.get(&Pos { x, y }) {
                    Some(b) => print!("b"),
                    None => print!("."),
                }
            }
        }
        println!()
    }
    println!()
}

fn traverse(
    starting_move: i32,
    start: Pos,
    target: Pos,
    initial: &HashSet<Blizzard>,
    valley: Pos,
) -> i32 {
    let mut current = Vec::new();
    current.push(start);

    let mut minute = starting_move;
    while true {
        minute += 1;
        let blizzards: HashSet<Pos> =
            HashSet::from_iter(initial.iter().map(|b| b.at(minute, valley)));
        let mut next = Vec::new();
        for elf_pos in &current {
            next.extend(all_moves(*elf_pos, &blizzards, valley, target));
        }
        if next.contains(&target) {
            break;
        }
        next.sort();
        next.dedup();
        current = next.clone();
    }

    minute
}

fn part1(raw: &str) {
    let (valley, initial) = parse(&raw);

    let elves = Pos { x: 1, y: 0 };
    let target = Pos {
        x: valley.x,
        y: valley.y + 1,
    };
    let result = traverse(0, elves, target, &initial, valley);

    println!("Part 1: {}", result);
}

fn part2(raw: &str) {
    let (valley, initial) = parse(&raw);

    let elves = Pos { x: 1, y: 0 };
    let target = Pos {
        x: valley.x,
        y: valley.y + 1,
    };
    let result = traverse(0, elves, target, &initial, valley);
    let result = traverse(result, target, elves, &initial, valley);
    let result = traverse(result, elves, target, &initial, valley);

    println!("Part 2: {}", result);
}

pub fn solve() {
    let raw = fs::read_to_string("data/day24.example").unwrap();
    part1(&raw);
    part2(&raw);

    let raw = fs::read_to_string("data/day24.txt").unwrap();
    part1(&raw);
    part2(&raw);
}
