use std::fs;

use regex::Regex;

#[derive(Debug)]
struct Blueprint {
    id: i32,
    ore_cost: i32,
    clay_cost: i32,
    obsidian_ore: i32,
    obsidian_clay: i32,
    geode_ore: i32,
    geode_obsidian: i32,
}

pub fn solve() {
    let raw = fs::read_to_string("data/day19.txt").unwrap();
    let re = Regex::new(r" (\d+):.* (\d+) .* (\d+) .* (\d+) .* (\d+) .* (\d+) .* (\d+) ").unwrap();
    let mut blueprints = Vec::new();
    for line in raw.lines() {
        for cap in re.captures_iter(line) {
            blueprints.push(Blueprint {
                id: cap[1].parse().unwrap(),
                ore_cost: cap[2].parse().unwrap(),
                clay_cost: cap[3].parse().unwrap(),
                obsidian_ore: cap[4].parse().unwrap(),
                obsidian_clay: cap[5].parse().unwrap(),
                geode_ore: cap[6].parse().unwrap(),
                geode_obsidian: cap[7].parse().unwrap(),
            });
        }
    }
}
