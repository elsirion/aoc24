use regex;

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Mul(u64, u64),
    Do,
    Dont,
}

fn parse_mul_statements(line: &str) -> Vec<Instruction> {
    let re = regex::Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();

    re.captures_iter(line).map(|cap| {
        match &cap[0][0..3] {
            "do(" => Instruction::Do,
            "don" => Instruction::Dont,
            "mul" => {
                let a = cap[1].parse().expect("not a number");
                let b = cap[2].parse().expect("not a number");
                Instruction::Mul(a, b)
            },
            _ => panic!("Invalid match")
        }
    }).collect()
}

fn main() {
    let lines = std::io::stdin().lines().map(|res| res.expect("stream error"));

    let prod_sum: u64 = lines.flat_map(|line| {
        parse_mul_statements(&line)
    }).fold((true, 0), |(active, sum), instruction| {
        match instruction {
            Instruction::Mul(a, b) if active => (active, sum + (a * b)),
            Instruction::Do => (true, sum),
            Instruction::Dont => (false, sum),
            _ => (active, sum)
        }
    }).1;

    println!("Product sum: {}", prod_sum);
}