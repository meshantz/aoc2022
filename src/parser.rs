use std::{fmt::Debug, fs, str::FromStr};

pub fn records_from_lines<T>(filename: &str) -> Vec<T>
where
    T: FromStr,
    T::Err: Debug,
{
    let raw = fs::read_to_string(filename).unwrap_or_else(|_err| {
        panic!(
            "we're hardcoding all filenames, but didn't find this one {}",
            filename
        )
    });
    let mut data: Vec<T> = Vec::new();

    for r in raw.lines() {
        data.push(r.parse().expect("failed to parse input file"));
    }

    data
}
