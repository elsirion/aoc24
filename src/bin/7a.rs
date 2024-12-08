use itertools::Itertools;

fn parse_line(line: &str) -> (u64, Vec<u64>) {
    let mut parts = line.split(": ");
    let result = parts.next().unwrap().parse().unwrap();
    let operands = parts
        .next()
        .unwrap()
        .split(" ")
        .map(|x| x.parse().unwrap())
        .collect();
    (result, operands)
}

fn parse_input() -> Vec<(u64, Vec<u64>)> {
    std::io::stdin()
        .lines()
        .map(|line_res| {
            let line = line_res.expect("stream error");
            parse_line(&line)
        })
        .collect()
}

#[derive(Debug, Clone, Copy)]
enum Operator {
    Add,
    Multiply,
}

fn evaluate_term(operands: &[u64], operators: &[Operator]) -> u64 {
    assert_eq!(operands.len(), operators.len() + 1);

    operands[1..]
        .into_iter()
        .zip(operators)
        .fold(operands[0], |acc, (operand, operator)| match operator {
            Operator::Add => acc + operand,
            Operator::Multiply => acc * operand,
        })
}

fn find_valid_operator_combination(operands: &[u64], target: u64) -> Option<Vec<Operator>> {
    let num_operators = operands.len() - 1;
    let mut operators_combinations = (0..num_operators)
        .map(|_| [Operator::Add, Operator::Multiply])
        .multi_cartesian_product();

    operators_combinations
        .find(|operators| evaluate_term(operands, operators.as_ref()) == target)
        .clone()
}

fn main() {
    let expressions = parse_input();

    let sum_valid_results = expressions
        .into_iter()
        .filter_map(|(result, operands)| {
            find_valid_operator_combination(&operands, result).map(|_| result)
        })
        .sum::<u64>();

    println!("Sum of valid results: {}", sum_valid_results);
}
