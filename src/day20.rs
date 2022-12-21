use std::{
    collections::{HashSet, VecDeque},
    fs,
};

fn rotate_any(working: &mut VecDeque<(usize, i64)>, val: i64) {
    while working[0].1 != val {
        let popped = working.pop_front().unwrap();
        working.push_back(popped);
    }
}

fn rotate_to(working: &mut VecDeque<(usize, i64)>, val: (usize, i64)) {
    while working[0] != val {
        let popped = working.pop_front().unwrap();
        working.push_back(popped);
    }
}

fn mix(working: &mut VecDeque<(usize, i64)>) {
    let (mix_index, mix_val) = working.pop_front().unwrap();
    let mix_dir = mix_val.signum();

    let mix_val = mix_val.abs();
    for _ in 0..mix_val % working.len() as i64 {
        if mix_dir > 0 {
            let popped = working.pop_front().unwrap();
            working.push_back(popped);
        } else {
            let popped = working.pop_back().unwrap();
            working.push_front(popped);
        }
    }
    let mix_val = mix_val * mix_dir;
    working.push_back((mix_index, mix_val));
}

fn part1(raw: &str) {
    let mut encoded = VecDeque::new();

    for (index, line) in raw.lines().enumerate() {
        let val: i64 = line.parse().unwrap();
        encoded.push_back((index, val));
    }

    let mut decode = encoded.clone();
    // println!("Initial Data: {:?}", decode);

    while let Some((i, v)) = encoded.pop_front() {
        rotate_to(&mut decode, (i, v));
        mix(&mut decode);
        // println!("Rotate {}: {:?}", v, decode);
    }
    rotate_any(&mut decode, 0);
    println!(
        "Part 1: {}",
        decode[1000 % decode.len()].1
            + decode[2000 % decode.len()].1
            + decode[3000 % decode.len()].1
    );
}

fn part2(raw: &str) {
    let key = 811589153;
    let mut encoded = VecDeque::new();

    for (index, line) in raw.lines().enumerate() {
        let val: i64 = line.parse().unwrap();
        encoded.push_back((index, val * key));
    }

    let mut decode = encoded.clone();
    // println!("Initial Data: {:?}", decode);

    let mut count = 0;
    let size = encoded.len();
    while let Some((i, v)) = encoded.pop_front() {
        rotate_to(&mut decode, (i, v));
        mix(&mut decode);
        // println!("Rotate {}: {:?}", v, decode);
        encoded.push_back((i, v));
        count += 1;
        if count > size * 10 {
            break;
        }
    }
    rotate_any(&mut decode, 0);
    println!(
        "Part 2: {}",
        decode[1000 % decode.len()].1
            + decode[2000 % decode.len()].1
            + decode[3000 % decode.len()].1
    );
}

pub fn solve() {
    let raw = fs::read_to_string("data/day20.example").unwrap();
    part1(&raw);
    part2(&raw);

    let raw = fs::read_to_string("data/day20.txt").unwrap();
    part1(&raw);
    part2(&raw);
}
