use std::{collections::HashMap, fs};

#[derive(Debug, Clone)]
enum Operator {
    Add,
    Mul,
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<u128>,
    operator: Operator,
    operation_value: Option<u128>,
    test_divisor: u128,
    monkey_true: usize,
    monkey_false: usize,
    inspection_count: u128,
}

impl Monkey {
    fn make(monkey_raw: &str) -> Monkey {
        let lines: Vec<&str> = monkey_raw.lines().collect();
        let start_items: Vec<u128> = strip_label(lines[1])
            .split(", ")
            .map(|s: &str| s.parse::<u128>().unwrap())
            .collect();

        let op_raw: Vec<&str> = strip_label(lines[2]).split(" ").collect();
        let operator = match op_raw[3] {
            "+" => Operator::Add,
            "*" => Operator::Mul,
            _ => panic!("not an operation operator"),
        };
        let operation_value: Option<u128> = match op_raw[4].parse() {
            Ok(v) => Some(v),
            Err(_) => None,
        };

        let test_divisor: u128 = strip_label(lines[3]).split(" ").collect::<Vec<&str>>()[2]
            .parse()
            .unwrap();

        let monkey_true: usize = strip_label(lines[4]).split(" ").collect::<Vec<&str>>()[3]
            .parse()
            .unwrap();

        let monkey_false: usize = strip_label(lines[5]).split(" ").collect::<Vec<&str>>()[3]
            .parse()
            .unwrap();

        Monkey {
            items: start_items,
            operator,
            operation_value,
            test_divisor,
            monkey_true,
            monkey_false,
            inspection_count: 0,
        }
    }
}

fn make_monkeys(monkeys_string: &str) -> Vec<Monkey> {
    let mut monkeys = Vec::new();

    for monkey_raw in monkeys_string.split("\n\n") {
        monkeys.push(Monkey::make(monkey_raw));
    }

    monkeys
}

fn strip_label(value: &str) -> &str {
    value.split(": ").collect::<Vec<&str>>()[1]
}

fn part(part_num: u8, raw: &str, relieved: bool, round_count: i32) {
    let mut monkeys = make_monkeys(&raw);
    // println!("{:?}", monkeys);
    let mut tossing: HashMap<usize, Vec<u128>> = HashMap::new();
    let ring_size = monkeys.iter().fold(1u128, |a, b| a * b.test_divisor);

    // rounds
    for round in 0..round_count {
        for monkey_index in 0..monkeys.len() {
            // for current_monkey in monkeys.iter_mut() {
            let current_monkey = &mut monkeys[monkey_index];
            // println!("Monkey {}:", monkey_index);
            for item in &current_monkey.items {
                current_monkey.inspection_count += 1;
                let mut worry = *item;
                // println!("  Monkey inspects an item with a worry level of {}:", worry);
                let actual_operand = match current_monkey.operation_value {
                    Some(v) => v,
                    None => worry,
                };
                match current_monkey.operator {
                    Operator::Add => worry += actual_operand,
                    Operator::Mul => worry *= actual_operand,
                }
                // println!("    Worry level changes to {}:", worry);
                if relieved {
                    worry /= 3;
                } else {
                    worry %= ring_size;
                }
                // println!("    Worry level /3 to {}:", worry);
                let test_result = worry % current_monkey.test_divisor == 0;
                // println!("    Test result is {}", test_result);
                let push_index = match test_result {
                    true => current_monkey.monkey_true,
                    false => current_monkey.monkey_false,
                };
                tossing.entry(push_index).or_insert(Vec::new()).push(worry);
            }
            current_monkey.items.clear();

            // This rearranges the order of things. If that turns out to matter, change to a deque.
            for toss_index in 0..monkeys.len() {
                match tossing.get_mut(&toss_index) {
                    Some(tosses) => {
                        while let Some(toss_val) = tosses.pop() {
                            monkeys[toss_index].items.push(toss_val);
                        }
                    }
                    None => (),
                }
            }
            tossing.clear();
        }

        // println!("\nAfter round {}:", round + 1);
        // for monkey in &monkeys {
        //     println!("{:?}, inspected: {}", monkey.items, monkey.inspection_count);
        // }
    }

    let mut inspections = monkeys
        .iter()
        .map(|m| m.inspection_count)
        .collect::<Vec<u128>>();
    inspections.sort_by(|a, b| b.cmp(a));

    println!("Part {}: {}", part_num, inspections[0] * inspections[1]);
}

pub fn solve() {
    let raw = fs::read_to_string("data/day11.example").unwrap();
    part(1, &raw, true, 20);
    part(2, &raw, false, 10_000);

    let raw = fs::read_to_string("data/day11.txt").unwrap();
    part(1, &raw, true, 20);
    part(2, &raw, false, 10_000);
}
