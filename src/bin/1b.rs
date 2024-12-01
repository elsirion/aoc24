use aoc24::parse_input_lists;

fn calculate_total_diff(mut a: Vec<u64>, mut b: Vec<u64>) -> u64 {
    a.sort();
    b.sort();

    a.into_iter().zip(b).map(|(a, b)| a.abs_diff(b)).sum()
}

fn main() {
    let (a, b) = parse_input_lists();
    let total_diff = calculate_total_diff(a, b);
    println!("Total difference: {total_diff}");
}
