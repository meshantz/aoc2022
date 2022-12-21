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

    fn apply(&self, left: i32, right: i32) -> i32 {
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
    value: Option<i32>,
    operation: Option<Op>,
    variable: Option<String>,
}

impl ValOrOp {
    fn as_val(v: i32) -> ValOrOp {
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

pub fn solve() {
    let raw = fs::read_to_string("data/day21.example").unwrap();
    let mut monkeys = HashMap::new();
    for line in raw.lines() {
        let (monkey, operation) = line.split_once(":").unwrap();
        let operation = operation.trim();
        match operation.parse::<i32>() {
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

    let mut resolve = monkeys.remove("root").unwrap();
    let mut index = 0;

    while index < resolve.len() {
        let current = resolve[index].clone();
        if current.variable != None {
            let find = current.variable.unwrap().clone();
            let find = find.as_str();
            let insert = monkeys.remove(find).unwrap();
            resolve.splice(index..index + 1, insert);
        } else {
            index += 1
        }
    }

    // FIXME: something not right about either the order or the operations...
    let mut stack = Vec::new();
    for step in resolve {
        if step.value != None {
            let value = step.value.unwrap();
            stack.push(value);
            println!("Pushed {}", value);
        } else {
            let operation = step.operation.unwrap();
            let left = stack.pop().unwrap();
            let right = stack.pop().unwrap();
            println!("Apply {:?} to {}, {}", operation, left, right);
            let result = operation.apply(left, right);
            stack.push(result);
        }
    }
    let final_result = stack.pop().unwrap();
    println!("Part 1: {}", final_result);
}
