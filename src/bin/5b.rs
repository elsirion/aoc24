use std::collections::{HashMap, HashSet};

fn parse_input() -> (Vec<(u64, u64)>, Vec<Vec<u64>>) {
    let mut lines_iter = std::io::stdin().lines().map(|line_res| {
        line_res.expect("stream error")
    });

    let rules = (&mut lines_iter).take_while(|line| !line.is_empty()).map(|line| {
        let mut parts = line.split("|");
        let a = parts.next().expect("no a").parse().expect("not a number");
        let b = parts.next().expect("no b").parse().expect("not a number");
        (a, b)
    }).collect::<Vec<_>>();

    let orderings = lines_iter.map(|ordering_str| {
        ordering_str.split(',').map(|num_str| num_str.parse().expect("not a number")).collect()
    }).collect::<Vec<_>>();

    (rules, orderings)
}

fn precedence_map(rules: Vec<(u64, u64)>) -> HashMap<u64, HashSet<u64>> {
    let mut precedence_map = HashMap::<u64, HashSet<u64>>::new();
    for (a, b) in rules {
        precedence_map.entry(b).or_default().insert(a);
    }
    precedence_map
}

fn is_ordering_valid(ordering: &[u64], precedence_map: &HashMap<u64, HashSet<u64>>) -> bool {
    for (idx, num) in ordering.iter().copied().enumerate() {
        for later_num in &ordering[idx + 1..] {
            if precedence_map.get(&num).map_or(false, |nums_before_set| nums_before_set.contains(later_num)) {
                return false;
            }
        }
    }
    true
}

fn topo_sort_stable(values: Vec<u64>, precedence_map: &HashMap<u64, HashSet<u64>>) -> Vec<u64> {
    let mut remaining = values;
    let mut ordered = Vec::new();

    while let Some(value) = remaining.first().copied() {
        let sorted_precedence_values = precedence_map.get(&value).map(|precedence_set| {
            let values_with_precedence = remaining[1..].iter().copied().filter(|value| precedence_set.contains(value)).collect::<Vec<_>>();
            topo_sort_stable(values_with_precedence, precedence_map)
        }).unwrap_or_default();

        remaining.retain_mut(|v| !sorted_precedence_values.contains(&v) && *v != value);

        ordered.extend(sorted_precedence_values);
        ordered.push(value);
    }

    ordered
}

fn main() {
    let (rules, orderings) = parse_input();

    let precedence_map = precedence_map(rules);

    let sum_mid_val = orderings.into_iter().filter_map(|ordering| {
        if is_ordering_valid(&ordering, &precedence_map) {
            return None;
        }

        let ordered = topo_sort_stable(ordering, &precedence_map);
        Some(ordered[ordered.len() / 2])
    }).sum::<u64>();

    println!("Sum of invalid ordering middle values: {}", sum_mid_val);
}