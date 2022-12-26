use std::fs;

fn to_snafu(val: i64) -> String {
    let mut v = Vec::new();

    let mut sum = 0;
    let mut place = 0;
    let mut remainder = val;

    while sum < val {
        let pow = 5i64.pow(place);
        let mut digit = (remainder / pow) % 5;
        if digit > 2 {
            digit -= 5;
        }
        sum += digit * pow;
        remainder -= digit * pow;
        match digit {
            -2 => v.push(String::from("=")),
            -1 => v.push(String::from("-")),
            d => v.push(d.to_string()),
        }
        place += 1;
    }

    v.reverse();
    v.join("")
}

fn to_base5v1(val: i64) -> String {
    let mut v = Vec::new();

    let mut place = 0;
    let mut pow = 5i64.pow(place);
    while val / pow > 0 {
        place += 1;
        pow = 5i64.pow(place);
    }

    let mut remainder = val;
    for place in (0..place).rev() {
        let pow = 5i64.pow(place);
        v.push((remainder / pow).to_string());
        remainder -= (remainder / pow) * pow;
    }

    v.join("")
}

fn to_base5v2(val: i64) -> String {
    let mut v = Vec::new();

    let mut sum = 0;
    let mut place = 0;

    while sum < val {
        let pow = 5i64.pow(place);
        let digit = (val / pow) % 5;
        sum += digit * pow;
        v.push(digit.to_string());
        place += 1;
    }

    v.reverse();
    v.join("")
}

fn from_snafu(val: &str) -> i64 {
    let mut sum = 0;
    let size = val.len();
    for (i, char) in val.chars().enumerate() {
        let place = size - i - 1;
        let place = 5i64.pow(place as u32);
        sum += match char {
            '2' => 2 * place,
            '1' => 1 * place,
            '0' => 0 * place,
            '-' => -1 * place,
            '=' => -2 * place,
            _ => panic!("invalid SNAFU digit"),
        }
    }

    sum
}

fn snafu_sum(raw: &str) -> String {
    to_snafu(raw.lines().map(|s| from_snafu(s)).sum::<i64>())
}

pub fn solve() {
    let raw = fs::read_to_string("data/day25.example").unwrap();
    println!("Sum: {}", snafu_sum(&raw));

    let raw = fs::read_to_string("data/day25.txt").unwrap();
    println!("Sum: {}", snafu_sum(&raw));
}
