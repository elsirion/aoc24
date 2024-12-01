use std::io::{stdin, BufRead};

pub fn parse_input_lists() -> (Vec<u64>, Vec<u64>) {
    stdin().lock().lines().map(|line_res| {
        let line = line_res.expect("stream error");
        let mut parts = line.split_whitespace();
        let first = parts.next().expect("malformed input").parse::<u64>().expect("malformed input");
        let second = parts.next().expect("malformed input").parse::<u64>().expect("malformed input");

        (first, second)
    }).unzip()
}