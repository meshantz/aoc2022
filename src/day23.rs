use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
};

const DIRECTIONS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

enum Dir {
    NW = 0,
    W = 1,
    SW = 2,
    N = 3,
    S = 4,
    NE = 5,
    E = 6,
    SE = 7,
}

fn extents(elves: &HashSet<(i32, i32)>) -> ((i32, i32), (i32, i32)) {
    let mut mins = (1000, 1000);
    let mut maxs = (0, 0);

    for elf in elves {
        mins.0 = mins.0.min(elf.0);
        mins.1 = mins.1.min(elf.1);
        maxs.0 = maxs.0.max(elf.0);
        maxs.1 = maxs.1.max(elf.1);
    }

    (mins, maxs)
}

fn make_checks() -> VecDeque<[(i32, i32); 3]> {
    let mut checks = VecDeque::new();
    checks.push_back([
        DIRECTIONS[Dir::N as usize],
        DIRECTIONS[Dir::NE as usize],
        DIRECTIONS[Dir::NW as usize],
    ]);
    checks.push_back([
        DIRECTIONS[Dir::S as usize],
        DIRECTIONS[Dir::SE as usize],
        DIRECTIONS[Dir::SW as usize],
    ]);
    checks.push_back([
        DIRECTIONS[Dir::W as usize],
        DIRECTIONS[Dir::NW as usize],
        DIRECTIONS[Dir::SW as usize],
    ]);
    checks.push_back([
        DIRECTIONS[Dir::E as usize],
        DIRECTIONS[Dir::NE as usize],
        DIRECTIONS[Dir::SE as usize],
    ]);
    checks
}

fn print_elves(elves: &HashSet<(i32, i32)>) {
    let ((min_x, min_y), (max_x, max_y)) = extents(&elves);

    for j in min_y..max_y + 1 {
        for i in min_x..max_x + 1 {
            match elves.contains(&(i, j)) {
                true => print!("#"),
                false => print!("."),
            }
        }
        println!();
    }
    println!();
}

fn part1(raw: &str) {
    let mut elves = HashSet::new();
    for (y, line) in raw.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char == '#' {
                elves.insert((x as i32, y as i32));
            }
        }
    }

    let mut checks = make_checks();
    // print_elves(&elves);

    let mut round = 0;
    let mut somebody_moved = true;
    while somebody_moved {
        round += 1;
        somebody_moved = false;
        let mut propose = HashMap::new();
        for elf in &elves {
            let must_move = DIRECTIONS
                .iter()
                .any(|e| elves.contains(&(elf.0 + e.0, elf.1 + e.1)));
            if !must_move {
                let mut proposed_movers = propose.entry(*elf).or_insert(Vec::new());
                proposed_movers.push(*elf);
                continue;
            }

            let mut did_move = false;
            for check in &checks {
                let can_move = check
                    .iter()
                    .all(|e| !elves.contains(&(elf.0 + e.0, elf.1 + e.1)));
                if can_move {
                    let proposed_position = (elf.0 + check[0].0, elf.1 + check[0].1);
                    let mut proposed_movers =
                        propose.entry(proposed_position).or_insert(Vec::new());
                    proposed_movers.push(*elf);
                    did_move = true;
                    somebody_moved = true;
                    break;
                }
            }

            if !did_move {
                let mut proposed_movers = propose.entry(*elf).or_insert(Vec::new());
                proposed_movers.push(*elf);
            }
        }

        // println!("Proposed: {:?}", propose);
        let mut next = HashSet::new();
        for (proposed, by) in &propose {
            if by.len() == 1 {
                next.insert(*proposed);
            } else {
                for elf in by {
                    next.insert(*elf);
                }
            }
        }

        elves = next.clone();
        let shuffle = checks.pop_front().unwrap();
        checks.push_back(shuffle);
        // print_elves(&elves);

        if round == 10 {
            let e = extents(&elves);
            let area = ((e.1).0 - (e.0).0 + 1).abs() * ((e.1).1 - (e.0).1 + 1).abs();
            let empty = area - elves.len() as i32;
            println!("Part 1: {}", empty);
        }
    }

    println!("Part 2: {}", round);
}

pub fn solve() {
    let raw = fs::read_to_string("data/day23.example").unwrap();
    part1(&raw);

    let raw = fs::read_to_string("data/day23.txt").unwrap();
    part1(&raw);
}
