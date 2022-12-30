use std::{collections::VecDeque, fs};

use regex::Regex;

#[derive(Debug, Clone, Copy)]
struct Blueprint {
    id: u32,
    ore_ore: u32,
    clay_ore: u32,
    obsidian_ore: u32,
    obsidian_clay: u32,
    geode_ore: u32,
    geode_obsidian: u32,
}

#[derive(Debug, Clone, Copy)]
struct State {
    minute: u8,
    time_limit: u8,
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32,
    ore_bots: u32,
    clay_bots: u32,
    obsidian_bots: u32,
    geode_bots: u32,
}

impl State {
    fn new(time_limit: u8) -> Self {
        Self {
            minute: 0,
            time_limit,
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
            ore_bots: 1,
            clay_bots: 0,
            obsidian_bots: 0,
            geode_bots: 0,
        }
    }
}

#[derive(Debug)]
enum Bot {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

// returns the number of turns to produce the given bot for each possible bot
fn next_bot_options(state: State, bp: Blueprint) -> Vec<(i32, Bot)> {
    let mut options = Vec::new();

    // cull anything that hasn't build an obsidian-collector by 2/3 of the time
    // if state.minute >= state.time_limit / 3 * 2 && state.obsidian_bots == 0 {
    //     return options;
    // }
    // cull anything that hasn't build an geode-cracker by 4/5 of the time
    // if state.minute >= state.time_limit / 12 * 11 && state.geode_bots == 0 {
    //     return options;
    // }

    if state.obsidian_bots > 0 && state.ore_bots > 0 {
        let obsidian_turns = ((bp.geode_obsidian.saturating_sub(state.obsidian)) as f32
            / state.obsidian_bots as f32)
            .ceil();
        let ore_turns =
            ((bp.geode_ore.saturating_sub(state.ore)) as f32 / state.ore_bots as f32).ceil();
        let turns = ore_turns.max(obsidian_turns) as i32 + 1;
        // if turns < 6 {
        options.push((turns, Bot::Geode));
        // }
    }

    // never suggest building more bots than the most needed for a single turn creation.
    let max_obsidian_bots = bp.geode_obsidian; // / 2;
    if state.clay_bots > 0 && state.ore_bots > 0 && state.obsidian_bots < max_obsidian_bots {
        let clay_turns =
            ((bp.obsidian_clay.saturating_sub(state.clay)) as f32 / state.clay_bots as f32).ceil();
        let ore_turns =
            ((bp.obsidian_ore.saturating_sub(state.ore)) as f32 / state.ore_bots as f32).ceil();
        let turns = ore_turns.max(clay_turns) as i32 + 1;
        // if turns < 6 {
        options.push((turns, Bot::Obsidian));
        // }
    }

    if state.ore_bots > 0 {
        let max_clay_bots = bp.obsidian_clay; // / 2;
        let max_ore_bots = bp
            .ore_ore
            .max(bp.clay_ore)
            .max(bp.obsidian_ore)
            .max(bp.geode_ore);
        // let max_ore_bots = max_ore_bots / 2;
        if state.clay_bots < max_clay_bots {
            let ore_turns =
                ((bp.clay_ore.saturating_sub(state.ore)) as f32 / state.ore_bots as f32).ceil();
            // if ore_turns < 5.0 {
            options.push((ore_turns as i32 + 1, Bot::Clay));
            // }
        }
        if state.ore_bots < max_ore_bots {
            let ore_turns =
                ((bp.ore_ore.saturating_sub(state.ore)) as f32 / state.ore_bots as f32).ceil();
            // if ore_turns < 5.0 {
            options.push((ore_turns as i32 + 1, Bot::Ore));
            // }
        }
    }

    options
}

fn advance_state(state: State, turns: u32, new_bot: Bot, bp: Blueprint) -> State {
    let mut new_state = State {
        minute: state.minute + turns as u8,
        time_limit: state.time_limit,
        ore: state.ore + turns * state.ore_bots,
        clay: state.clay + turns * state.clay_bots,
        obsidian: state.obsidian + turns * state.obsidian_bots,
        geode: state.geode + turns * state.geode_bots,
        ore_bots: state.ore_bots,
        clay_bots: state.clay_bots,
        obsidian_bots: state.obsidian_bots,
        geode_bots: state.geode_bots,
    };
    match new_bot {
        Bot::Ore => {
            new_state.ore_bots += 1;
            new_state.ore -= bp.ore_ore;
        }
        Bot::Clay => {
            new_state.clay_bots += 1;
            new_state.ore -= bp.clay_ore;
        }
        Bot::Obsidian => {
            new_state.obsidian_bots += 1;
            new_state.ore -= bp.obsidian_ore;
            new_state.clay -= bp.obsidian_clay;
        }
        Bot::Geode => {
            new_state.geode_bots += 1;
            new_state.ore -= bp.geode_ore;
            new_state.obsidian -= bp.geode_obsidian;
        }
    }
    new_state
}

fn run_blueprint(bp: Blueprint, time_limit: u8) -> u32 {
    let state = State::new(time_limit);
    // println!("Initial state: {:?}", state);
    let mut queue = VecDeque::new();
    let mut results = Vec::new();

    for (turns, bot) in next_bot_options(state, bp) {
        queue.push_back((state, turns as u32, bot));
    }

    let mut evaluations = 0;
    // println!("Starting search queue: {:?}", queue);
    while let Some((state, turns, bot)) = queue.pop_front() {
        evaluations += 1;
        // if abort > 10000 {
        //     break;
        // }
        // println!(
        //     "Assessing next state for: {:?}, {}, {:?}",
        //     state, turns, bot
        // );
        if state.minute + turns as u8 > time_limit {
            let turns_left = time_limit - state.minute;
            let geodes = state.geode + state.geode_bots * turns_left as u32;
            // println!("  State exits timeframe with {} geodes", geodes);
            results.push(geodes);
        } else {
            let next_state = advance_state(state, turns, bot, bp);
            for (turns, bot) in next_bot_options(next_state, bp) {
                // println!(
                //     "  State results in new branch: {:?}, {}, {:?}",
                //     next_state, turns, bot
                // );
                queue.push_back((next_state, turns as u32, bot));
            }
        }
    }

    println!("Blueprint {} required {} evaluations", bp.id, evaluations);
    let best = results.iter().max().unwrap();
    *best
}

fn make_blueprints(raw: &str) -> Vec<Blueprint> {
    let re = Regex::new(r" (\d+):.* (\d+) .* (\d+) .* (\d+) .* (\d+) .* (\d+) .* (\d+) ").unwrap();
    let mut blueprints = Vec::new();
    for line in raw.lines() {
        for cap in re.captures_iter(line) {
            blueprints.push(Blueprint {
                id: cap[1].parse().unwrap(),
                ore_ore: cap[2].parse().unwrap(),
                clay_ore: cap[3].parse().unwrap(),
                obsidian_ore: cap[4].parse().unwrap(),
                obsidian_clay: cap[5].parse().unwrap(),
                geode_ore: cap[6].parse().unwrap(),
                geode_obsidian: cap[7].parse().unwrap(),
            });
        }
    }
    blueprints
}

fn part1(blueprints: &Vec<Blueprint>) {
    let mut sum = 0;
    for bp in blueprints {
        // let bp = blueprints[1];
        let best = run_blueprint(*bp, 24);
        println!("Turns for bp {}: {}", bp.id, best);
        // break;
        sum += best * bp.id as u32;
    }

    println!("Part 1: {}", sum);
}

fn part2(blueprints: &Vec<Blueprint>) {
    let mut prod = 1;
    let time_limit = 32;
    println!("At {} minutes...", time_limit);
    for bp in blueprints {
        // let bp = blueprints[1];
        let best = run_blueprint(*bp, time_limit);
        println!("Turns for bp {}: {}", bp.id, best);
        // break;
        prod *= best;
    }

    println!("Part 1: {}", prod);
}

pub fn solve() {
    let raw = fs::read_to_string("data/day19.example").unwrap();
    let blueprints = make_blueprints(&raw);
    part1(&blueprints);
    // part2(&blueprints);

    let raw = fs::read_to_string("data/day19.txt").unwrap();
    let blueprints = make_blueprints(&raw);
    part1(&blueprints);
}
