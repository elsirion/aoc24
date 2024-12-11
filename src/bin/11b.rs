use std::collections::HashMap;

fn parse_int_list() -> Vec<u64> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("read error");
    input
        .split(' ')
        .map(|number| number.trim().parse::<u64>().expect("malformed input"))
        .collect()
}

struct StackElement {
    total_count_pre: usize,
    number: u64,
    remaining_branch: Option<u64>,
}

fn num_stones_after_n_iterations(
    n: usize,
    number: u64,
    // (num, depth) -> num_stones
    cache: &mut HashMap<(u64, usize), usize>,
) -> usize {
    let mut stack = Vec::<StackElement>::with_capacity(75);

    let mut current = number;
    let mut leaf_count_so_far = 0;
    loop {
        let cached = cache.get(&(current, stack.len()));
        if stack.len() == n || cached.is_some() {
            leaf_count_so_far += if let Some(cached) = cached {
                *cached
            } else {
                let leaf = stack.pop().expect("stack underflow");
                let leaf_count = if leaf.remaining_branch.is_some() {
                    2
                } else {
                    1
                };
                cache.insert((leaf.number, stack.len()), leaf_count);
                leaf_count
            };

            while let Some(&StackElement {
                remaining_branch: None,
                ..
            }) = stack.last()
            {
                let stack_elem = stack.pop().expect("stack underflow");
                cache.insert(
                    (stack_elem.number, stack.len()),
                    leaf_count_so_far - stack_elem.total_count_pre,
                );
            }

            let Some(last) = stack.last_mut() else {
                return leaf_count_so_far;
            };

            current = last.remaining_branch.take().expect("we removed al Nones");
        }

        if current == 0 {
            stack.push(StackElement {
                total_count_pre: leaf_count_so_far,
                number: current,
                remaining_branch: None,
            });
            current = 1;
            continue;
        }

        let num_len = current.ilog10() + 1;
        if num_len % 2 == 0 {
            let half = num_len / 2;
            let left = current / 10u64.pow(half as u32);
            let right = current % 10u64.pow(half as u32);
            stack.push(StackElement {
                total_count_pre: leaf_count_so_far,
                number: current,
                remaining_branch: Some(left),
            });
            current = right;
            continue;
        }

        stack.push(StackElement {
            total_count_pre: leaf_count_so_far,
            number: current,
            remaining_branch: None,
        });
        current *= 2024;
    }
}

fn main() {
    let numbers = parse_int_list();
    let mut cache = HashMap::new();
    let total_num = numbers
        .into_iter()
        .map(|num| num_stones_after_n_iterations(75, num, &mut cache))
        .sum::<usize>();

    println!("Number of stones after 75 iterations: {}", total_num);
    println!("Final cache size: {}", cache.len());
}
