use std::fs;

fn make_packet(val: &str) -> Vec<(i32, i32)> {
    let mut depth = 0;
    let mut collect = Vec::new();
    let mut packet = Vec::new();
    let chars_iter: Vec<char> = val.chars().collect();
    for i in 0..val.len() {
        let char = chars_iter.get(i);
        match char {
            Some('[') => {
                packet.push((depth, -1));
                depth += 1;
            }
            Some(']') => {
                // packet.push((depth, -2));
                depth -= 1;
            }
            Some(',') => (), // noop
            Some(n) => {
                let next_char = chars_iter.get(i + 1);
                collect.push(*n);
                match next_char {
                    Some('[') => (), // fully collected
                    Some(']') => (), // fully collected
                    Some(',') => (), // fully collected
                    None => panic!("Unexpected end of packet!"),
                    Some(_) => continue, // more in this number, keep collecting
                }
                let integer: i32 = collect.iter().collect::<String>().parse().unwrap();
                packet.push((depth, integer));
                collect.clear();
            }
            None => (),
        }
    }
    packet
}

// fn is_open_brace(val: i32) -> bool {
//     match val {
//         -1 => true,
//         _ => false,
//     }
// }

// fn is_close_brace(val: i32) -> bool {
//     match val {
//         -2 => true,
//         _ => false,
//     }
// }

fn is_brace(val: i32) -> bool {
    match val {
        -1 => true,
        // -2 => true,
        _ => false,
    }
}

fn part1(raw: &str) {
    let mut sum = 0;
    for (index, pair_raw) in raw.split("\n\n").enumerate() {
        let (left, right) = pair_raw.split_once("\n").unwrap();
        let left = make_packet(left.trim());
        let right = make_packet(right.trim());

        let mut left_stack = left.clone();
        let mut right_stack = right.clone();
        let mut is_correct_order: Option<bool> = None;

        left_stack.reverse();
        right_stack.reverse();
        // println!("Validating {}", index);
        while is_correct_order == None {
            let left_val = left_stack.pop();
            let right_val = right_stack.pop();

            if left_val == None && right_val == None {
                panic!("Ran out of input!");
            } else if left_val == None {
                is_correct_order = Some(true);
                continue;
            } else if right_val == None {
                is_correct_order = Some(false);
                continue;
            }

            let (left_depth, left_val) = left_val.unwrap();
            let (right_depth, right_val) = right_val.unwrap();
            if is_brace(left_val) && is_brace(right_val) && left_depth == right_depth {
                // both start lists. consume and continue.
                // println!("two starting parens, consume and continue");
            } else if is_brace(left_val) && is_brace(right_val) {
                // one of our lists ended before the other.
                // println!("one list ended short");
                if left_depth < right_depth {
                    is_correct_order = Some(true);
                } else {
                    is_correct_order = Some(false);
                }
            } else if !is_brace(left_val) && !is_brace(right_val) {
                // println!("both ints");
                if left_depth != right_depth {
                    // one of our lists ended before the other.
                    // println!("one list ended short(2)");
                    // FIXME: too simple. we need to know expected depth of the list...
                    if left_depth < right_depth {
                        is_correct_order = Some(true);
                    } else {
                        is_correct_order = Some(false);
                    }
                } else if left_val < right_val {
                    is_correct_order = Some(true);
                } else if left_val > right_val {
                    is_correct_order = Some(false);
                }
            } else if is_brace(left_val) {
                // println!("re-push right");
                left_stack.push((left_depth, left_val));
                right_stack.push((right_depth + 1, right_val));
                right_stack.push((right_depth, -1)); // add a brace
            } else {
                // println!("re-push left");
                left_stack.push((left_depth + 1, left_val));
                left_stack.push((left_depth, -1)); // add a brace
                right_stack.push((right_depth, right_val));
            }
        }
        let is_correct_order = is_correct_order.unwrap();
        // println!("{:?}", left);
        // println!("{:?}", right);
        // println!("{:?}", is_correct_order);
        // println!();
        if is_correct_order {
            sum += index + 1;
        }
    }

    println!("Part 1: {}", sum);
}

pub fn solve() {
    let raw = fs::read_to_string("data/day13.example").unwrap();
    part1(&raw);

    let raw = fs::read_to_string("data/day13.txt").unwrap();
    part1(&raw);
}
