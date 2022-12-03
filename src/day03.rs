use std::fs;

const PRIORITIES: &str = ".abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub fn solve() {
    let raw = fs::read_to_string("data/day03.example").unwrap();
    println!("Example Result DAY 03 PART 1: {}", part1(&raw));
    println!("Example Result DAY 03 PART 1: {}", part2(&raw));

    let raw = fs::read_to_string("data/day03.txt").unwrap();
    println!("Final Result DAY 03 PART 1: {}", part1(&raw));
    println!("Final Result DAY 03 PART 1: {}", part2(&raw));
}

fn part1(raw: &str) -> u32 {
    let mut priority_sum: u32 = 0;
    for rucksack in raw.lines() {
        let half = rucksack.len() / 2;
        let c1 = &rucksack[0..half];
        let c2 = &rucksack[half..rucksack.len()];
        for item in c1.chars() {
            if c2.contains(item) {
                priority_sum += PRIORITIES.find(item).unwrap() as u32;
                break;
            }
        }
    }

    priority_sum
}

fn part2(raw: &str) -> u32 {
    let mut priority_sum: u32 = 0;
    let mut group = Vec::new();

    for (i, rucksack) in raw.lines().enumerate() {
        if i % 3 == 0 {
            group.clear();
        }
        group.push(rucksack);
        if i % 3 == 2 {
            let badge = find_common(&group);
            priority_sum += PRIORITIES.find(badge).unwrap() as u32;
        }
    }

    priority_sum
}

fn find_common(group: &Vec<&str>) -> char {
    for char in group[0].chars() {
        if group[1].contains(char) && group[2].contains(char) {
            return char;
        }
    }

    panic!("no overlapping item!");
}
