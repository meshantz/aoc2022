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
    let test_packet = &packets[0].first;

    println!(
        "{:?}",
        int_value(PacketPart {
            kind: PacketPartType::Int,
            value: "23",
            iter_loc: -1,
        })
    )
}
