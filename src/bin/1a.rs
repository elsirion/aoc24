use aoc24::parse_input_lists;
use itertools::Itertools;
use std::collections::HashSet;

fn calculate_similarity(a: Vec<u64>, b: Vec<u64>) -> u64 {
    let a_unique = a.into_iter().collect::<HashSet<_>>();
    let b_count_lookup = b.into_iter().counts();

    a_unique
        .into_iter()
        .map(|a| {
            let b_count = *b_count_lookup.get(&a).unwrap_or(&0usize) as u64;
            a * b_count
        })
        .sum()
}

fn main() {
    let (a, b) = parse_input_lists();
    let similarity = calculate_similarity(a, b);
    println!("Similarity: {similarity}");
}
