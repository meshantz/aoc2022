use std::fs;

#[derive(Debug)]
enum PacketPartType {
    List,
    Int,
}

#[derive(Debug)]
struct PacketPart<'a> {
    kind: PacketPartType,
    value: &'a str,
    iter_loc: i32,
}

#[derive(Debug)]
struct PacketPair<'a> {
    first: PacketPart<'a>,
    second: PacketPart<'a>,
}

fn new_list(list_str: &str) -> PacketPart {
    PacketPart {
        kind: PacketPartType::List,
        value: list_str,
        iter_loc: -1,
    }
}

fn int_value(part: PacketPart) -> Option<i32> {
    match part.kind {
        PacketPartType::Int => Some(part.value.parse().unwrap()),
        PacketPartType::List => None,
    }
}

fn next<'a>(val: &'a mut PacketPart) -> Option<&'a str> {
    // return the next "list or int"
    None
}

fn parens(value: &str) -> Vec<(i32, i32)> {
    let mut build = Vec::new();
    let mut stack = Vec::new();

    for (i, c) in value.chars().enumerate() {
        if c == '[' {
            stack.push(i as i32);
        } else if c == ']' {
            let start = stack.pop().unwrap();
            build.push((start, i as i32));
        }
    }

    build
}

fn label_parens(val: &str, parens: &Vec<(i32, i32)>) {
    let mut w = Vec::new();
    for (i, j) in parens {
        w.push(*i as usize);
        w.push(*j as usize);
    }
    w.sort();
    println!("{}", val);
    let mut k = 0;
    for i in 0..val.len() {
        if w[k] == i {
            k += 1;
            print!("^");
        } else {
            print!(" ");
        }
    }
    println!();
}

pub fn solve() {
    let raw = fs::read_to_string("data/day13.example").unwrap();
    let mut packets = Vec::new();

    for pair_raw in raw.split("\n\n") {
        let parts: Vec<&str> = pair_raw.split("\n").collect();
        packets.push(PacketPair {
            first: new_list(parts[0]),
            second: new_list(parts[1]),
        })
    }

    // for packet in packets {
    //     println!("{:?}", packet)
    // }
    let test_packet = &packets[7].first;
    let v = parens(test_packet.value);
    label_parens(test_packet.value, &v);

    println!(
        "{:?}",
        int_value(PacketPart {
            kind: PacketPartType::Int,
            value: "23",
            iter_loc: -1,
        })
    )
}
