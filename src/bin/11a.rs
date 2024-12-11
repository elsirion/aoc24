fn parse_int_list() -> Vec<u64> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("read error");
    input
        .split(' ')
        .map(|number| number.trim().parse::<u64>().expect("malformed input"))
        .collect()
}

fn apply_rules(numbers: Vec<u64>) -> Vec<u64> {
    numbers
        .into_iter()
        .flat_map(|number| {
            if number == 0 {
                return vec![1];
            }

            let number_string = format!("{}", number);
            if number_string.len() % 2 == 0 {
                let half = number_string.len() / 2;
                let left = number_string[..half].parse::<u64>().unwrap();
                let right = number_string[half..].parse::<u64>().unwrap();
                return vec![left, right];
            }

            vec![number * 2024]
        })
        .collect()
}

fn apply_rules_n_times(numbers: Vec<u64>, n: usize) -> Vec<u64> {
    (0..n).fold(numbers, |numbers, _| apply_rules(numbers))
}

fn main() {
    let numbers = parse_int_list();
    let stones_after_25 = apply_rules_n_times(numbers, 25);
    println!(
        "Number of stones after 25 iterations: {}",
        stones_after_25.len()
    );
}
