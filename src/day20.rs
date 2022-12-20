use std::{
    collections::{HashSet, VecDeque},
    fs,
};

fn rotate_to(working: &mut VecDeque<i32>, val: i32) {
    while working[0] != val {
        let popped = working.pop_front().unwrap();
        working.push_back(popped);
    }
}

fn mix(working: &mut VecDeque<i32>) {
    let mix_val = working.pop_front().unwrap();
    let mix_dir = mix_val.signum();
    if mix_dir > 0 {
        working.push_back(mix_val);
    } else {
        working.push_front(mix_val);
    }

    let mix_val = mix_val.abs();
    // TODO: gotta be some good modulo arithmetic here...
    // println!(
    //     "mod: {}, div: {}",
    //     mix_val as usize % (working.len() + 1),
    //     mix_val as usize / (working.len() + 1)
    // );
    // for _ in 0..mix_val % (working.len() as i32 + 1) {
    for _ in 0..mix_val {
        if mix_dir > 0 {
            let popped = working.pop_front().unwrap();
            working.push_back(popped);
        } else {
            let popped = working.pop_back().unwrap();
            working.push_front(popped);
        }
    }
    let mix_val = mix_val * mix_dir;
    working.push_back(mix_val);
    if working[0] != mix_val {
        rotate_to(working, mix_val);
    }
    working.pop_front();
}

fn part1(raw: &str) {
    let mut encoded = VecDeque::new();
    let mut better_not_be = HashSet::new();

    for line in raw.lines() {
        let val: i32 = line.parse().unwrap();
        encoded.push_back(val);
        better_not_be.insert(val); // BOO. numbers aren't unique
    }

    let mut decode = encoded.clone();
    println!(
        "Initial Data: {:?}",
        // decode.clone().into_iter().max().unwrap()
        better_not_be.len()
    );
    // println!("Initial Data: {:?}", decode);

    while let Some(v) = encoded.pop_front() {
        rotate_to(&mut decode, v);
        mix(&mut decode);
        // println!("Rotate {}: {:?}", v, decode);
    }
    rotate_to(&mut decode, 0);
    println!(
        "Part 1: {}",
        decode[1000 % decode.len()] + decode[2000 % decode.len()] + decode[3000 % decode.len()]
    );
}

pub fn solve() {
    let raw = fs::read_to_string("data/day20.example").unwrap();
    part1(&raw);

    let raw = fs::read_to_string("data/day20.txt").unwrap();
    part1(&raw);
}
