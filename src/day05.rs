use std::fs;

pub fn solve() {
    let raw = fs::read_to_string("data/day05.example").unwrap();
    println!("Example Result PART 1: {}", part1(&raw));
    println!("Example Result PART 2: {}", part2(&raw));

    let raw = fs::read_to_string("data/day05.txt").unwrap();
    println!("Final Result PART 1: {}", part1(&raw));
    println!("Final Result PART 2: {}", part2(&raw));
}

fn part1(raw: &str) -> String {
    let (stacks, commands) = parse(raw);
    let mut stacks = stacks;
    let stack_count = stacks.len();

    for command in commands.lines() {
        let raw: Vec<&str> = command.split(" ").collect();
        let (count, from, to): (u32, usize, usize) = (
            raw[1].parse().unwrap(),
            raw[3].parse().unwrap(),
            raw[5].parse().unwrap(),
        );

        for _ in 0..count {
            let moving = stacks[from - 1].pop().unwrap();
            stacks[to - 1].push(moving);
        }
    }

    let mut tops = Vec::new();
    for i in 0..stack_count {
        tops.push(String::from(stacks[i as usize].pop().unwrap()));
    }

    tops.join("")
}

fn part2(raw: &str) -> String {
    let (stacks, commands) = parse(raw);
    let mut stacks = stacks;
    let stack_count = stacks.len();
    let mut reshuffler = Vec::new();

    for command in commands.lines() {
        let raw: Vec<&str> = command.split(" ").collect();
        let (count, from, to): (u32, usize, usize) = (
            raw[1].parse().unwrap(),
            raw[3].parse().unwrap(),
            raw[5].parse().unwrap(),
        );

        for _ in 0..count {
            reshuffler.push(stacks[from - 1].pop().unwrap());
        }
        for _ in 0..count {
            stacks[to - 1].push(reshuffler.pop().unwrap());
        }
    }

    let mut tops = Vec::new();
    for i in 0..stack_count {
        tops.push(String::from(stacks[i as usize].pop().unwrap()));
    }

    tops.join("")
}

fn parse(raw: &str) -> (Vec<Vec<char>>, &str) {
    let parts: Vec<&str> = raw.split("\n\n").collect();
    let (stacks, commands) = (parts[0], parts[1]);
    let stacks_reversed: Vec<&str> = stacks.lines().rev().collect();
    let stack_count: u8 = stacks_reversed[0]
        .trim()
        .split(" ")
        .last()
        .unwrap()
        .parse()
        .unwrap();

    let mut stacks: Vec<Vec<char>> = Vec::new();
    for _ in 0..stack_count {
        stacks.push(Vec::new());
    }

    for l in &stacks_reversed[1..stacks_reversed.len()] {
        let line_chars: Vec<char> = l.chars().collect();
        for i in 0..stack_count {
            let pos = (i as usize) * 4 + 1;
            let pos_char = line_chars[pos];
            if pos_char == ' ' {
                continue;
            }
            stacks[i as usize].push(pos_char)
        }
    }

    (stacks, commands)
}
