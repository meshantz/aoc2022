use std::{collections::HashMap, fs};

#[derive(Debug, Clone, Copy)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl Op {
    fn from(v: &str) -> Op {
        match v {
            "+" => Op::Add,
            "-" => Op::Sub,
            "*" => Op::Mul,
            "/" => Op::Div,
            _ => panic!("Invalid operation"),
        }
    }

    fn inverse(&self) -> Op {
        match self {
            Op::Add => Op::Sub,
            Op::Sub => Op::Add,
            Op::Mul => Op::Div,
            Op::Div => Op::Mul,
        }
    }

    fn apply(&self, left: i64, right: i64) -> i64 {
        match self {
            Op::Add => left + right,
            Op::Sub => left - right,
            Op::Mul => left * right,
            Op::Div => left / right,
        }
    }
}

#[derive(Debug, Clone)]
struct ValOrOp {
    value: Option<i64>,
    operation: Option<Op>,
    variable: Option<String>,
}

impl ValOrOp {
    fn as_val(v: i64) -> ValOrOp {
        ValOrOp {
            value: Some(v),
            operation: None,
            variable: None,
        }
    }

    fn as_op(o: Op) -> ValOrOp {
        ValOrOp {
            value: None,
            operation: Some(o),
            variable: None,
        }
    }

    fn as_var(v: &str) -> ValOrOp {
        ValOrOp {
            value: None,
            operation: None,
            variable: Some(String::from(v)),
        }
    }
}

fn calculate1(part: i32, resolve: &Vec<ValOrOp>) {
    let mut stack = Vec::new();
    for step in resolve {
        if step.value != None {
            let value = step.value.unwrap();
            stack.push(value);
            //println!("Pushed {}", value);
        } else {
            let operation = step.operation.unwrap();
            let right = stack.pop().unwrap();
            let left = stack.pop().unwrap();
            //println!("Apply {:?} to {}, {}", operation, left, right);
            let result = operation.apply(left, right);
            stack.push(result);
        }
    }
    let final_result = stack.pop().unwrap();
    println!("Part {}: {}", part, final_result);
}

fn calculate2(part: i32, resolve: &Vec<ValOrOp>) {
    let mut stack = Vec::new();
    let mut alt_stack = Vec::new();
    for step in resolve {
        if step.value != None {
            let value = step.value.unwrap();
            stack.push(value);
            //println!("Pushed {}", value);
        } else {
            let operation = step.operation.unwrap();
            let right = stack.pop().unwrap();
            let left = stack.pop().unwrap();
            if left == 0 {
                alt_stack.push(ValOrOp::as_op(operation.inverse()));
                alt_stack.push(ValOrOp::as_val(right));
                stack.push(0);
            } else if right == 0 {
                alt_stack.push(ValOrOp::as_op(operation.inverse()));
                alt_stack.push(ValOrOp::as_val(left));
                stack.push(0);
            } else {
                //println!("Apply {:?} to {}, {}", operation, left, right);
                let result = operation.apply(left, right);
                if result == 0 {
                    panic!("Caclulated 0. Invalid assumption")
                }
                stack.push(result);
            }
        }
    }

    alt_stack.push(ValOrOp::as_val(0));
    alt_stack.reverse();
    calculate1(part, &alt_stack);
}

fn make_monkeys(raw: &str) -> HashMap<&str, Vec<ValOrOp>> {
    let mut monkeys = HashMap::new();
    for line in raw.lines() {
        let (monkey, operation) = line.split_once(":").unwrap();
        let operation = operation.trim();
        match operation.parse::<i64>() {
            Ok(i) => monkeys.insert(monkey, vec![ValOrOp::as_val(i)]),
            Err(err) => {
                let parts: Vec<&str> = operation.split(" ").collect();
                monkeys.insert(
                    monkey,
                    vec![
                        ValOrOp::as_var(parts[0]),
                        ValOrOp::as_var(parts[2]),
                        ValOrOp::as_op(Op::from(parts[1])),
                    ],
                )
            }
        };
    }
    monkeys
}

fn postfixify(monkeys: &mut HashMap<&str, Vec<ValOrOp>>, keep_human: bool) -> Vec<ValOrOp> {
    let mut resolve = monkeys.remove("root").unwrap();
    let mut index = 0;
    while index < resolve.len() {
        let current = resolve[index].clone();
        if current.variable != None {
            let find = current.variable.unwrap().clone();
            if keep_human && find == "humn" {
                // 0 is not in the set, maybe we calculate to it though...
                resolve.splice(index..index + 1, vec![ValOrOp::as_val(0)]);
                index += 1;
                continue;
            }
            let find = find.as_str();
            let insert = monkeys.remove(find).unwrap();
            resolve.splice(index..index + 1, insert);
        } else {
            index += 1
        }
    }
    resolve
}

fn part1(raw: &str) {
    let mut monkeys = make_monkeys(raw);
    let resolve = postfixify(&mut monkeys, false);
    calculate1(1, &resolve);
}

fn part2(raw: &str) {
    let mut monkeys = make_monkeys(raw);
    let root = monkeys.remove("root").unwrap();
    monkeys.insert(
        "root",
        vec![root[0].clone(), root[1].clone(), ValOrOp::as_op(Op::Sub)],
    );
    let resolve = postfixify(&mut monkeys, true);
    calculate2(2, &resolve);
}

pub fn solve() {
    let raw = fs::read_to_string("data/day21.example").unwrap();
    part1(&raw);
    part2(&raw); // works (301)

    let raw = fs::read_to_string("data/day21.txt").unwrap();
    part1(&raw);
    part2(&raw); // incorrect (9626251584895)
}
