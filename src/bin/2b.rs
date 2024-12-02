use itertools::Itertools;
use aoc24::parse_input_reports;

enum Direction {
    Up,
    Down,
    Equal
}

fn is_safe_inner(report: &[u64]) -> bool {
    let deltas = report.iter().tuple_windows().map(|(a, b)| {
        (*a as i64) - (*b as i64)
    }).collect::<Vec<_>>();

    let is_going_same_direction = deltas.iter().map(|x | x.is_negative()).all_equal();
    let is_within_bounds = deltas.iter().all(|x| *x != 0 && x.abs() <= 3);

    is_going_same_direction && is_within_bounds
}

fn is_safe(report: &[u64]) -> bool {
    if is_safe_inner(report) {
        return true;
    }

    // Small enough inputs to brute force
    for remove_item_idx in 0..report.len() {
        let clean_report = report.iter().copied().enumerate().filter_map(|(idx, item_value)| {
            (idx != remove_item_idx).then_some(item_value)
        }).collect::<Vec<_>>();

        if is_safe_inner(&clean_report) {
            return true;
        }
    }

    false
}
fn main() {
    let reports = parse_input_reports();
    let safe_reports = reports.iter()
        .filter_map(|report| is_safe(report.as_ref()).then_some(()))
        .count();
    println!("Safe reports: {safe_reports}");
}