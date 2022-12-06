use std::{
    collections::{HashSet, VecDeque},
    fs,
};

pub fn solve() {
    let raw = fs::read_to_string("data/day06.examples").unwrap();
    for line in raw.lines() {
        println!("{}: start-of-packet: {}", line, for_line(&line, 4));
        println!("{}: start-of-message: {}", line, for_line(&line, 14));
    }
    let raw = fs::read_to_string("data/day06.txt").unwrap();
    println!("Final result start-of-packet:  {}", for_line(&raw, 4));
    println!("Final result start-of-message: {}", for_line(&raw, 14));
}

fn for_line(stream: &str, buffer_length: usize) -> usize {
    let mut buffer = VecDeque::new();
    let mut validator = HashSet::new();
    for (index, current) in stream.chars().enumerate() {
        buffer.push_front(current);
        if index < buffer_length - 1 {
            continue;
        }

        validator.clear();
        for sc in &buffer {
            validator.insert(sc.clone());
        }

        // println!("Considering {:?}: {}", buffer, validator.len());
        if validator.len() == buffer_length {
            return index + 1;
        }

        if buffer.len() >= buffer_length {
            buffer.pop_back();
        }
    }

    panic!("4 distinct chars not found!")
}
