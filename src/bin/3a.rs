use regex;

fn parse_mul_statements(line: &str) -> Vec<(u64, u64)> {
    let re = regex::Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    re.captures_iter(line).map(|cap| {
        let a = cap[1].parse().expect("not a number");
        let b = cap[2].parse().expect("not a number");
        (a, b)
    }).collect()
}

fn main() {
    let lines = std::io::stdin().lines().map(|res| res.expect("stream error"));
    let prod_sum: u64 = lines.flat_map(|line| {
        parse_mul_statements(&line).into_iter().map(|(a, b)| a * b)
    }).sum();

    println!("Product sum: {}", prod_sum);
}