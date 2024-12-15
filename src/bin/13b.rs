use itertools::Itertools;
use std::fmt::{Debug, Formatter};
use std::iter::once;
use std::ops::{Add, Mul, Sub};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Coordinates {
    x: i64,
    y: i64,
}

impl Coordinates {
    fn is_within_bounds_inclusive(&self, bounds: Coordinates) -> bool {
        self.x >= 0 && self.y >= 0 && self.x <= bounds.x && self.y <= bounds.y
    }

    fn divided_by(&self, other: Coordinates) -> Option<i64> {
        let no_remainder = {
            let x_rem = self.x % other.x;
            let y_rem = self.y % other.y;
            x_rem == 0 && y_rem == 0
        };

        if !no_remainder {
            return None;
        }

        let x_quot = self.x / other.x;
        let y_quot = self.y / other.y;
        (x_quot == y_quot).then_some(x_quot)
    }
}

impl Debug for Coordinates {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(x={}, y={})", self.x, self.y)
    }
}

impl Add for Coordinates {
    type Output = Coordinates;

    fn add(self, rhs: Self) -> Self::Output {
        Coordinates {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Coordinates {
    type Output = Coordinates;

    fn sub(self, rhs: Self) -> Self::Output {
        Coordinates {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul<u64> for Coordinates {
    type Output = Coordinates;

    fn mul(self, rhs: u64) -> Self::Output {
        Coordinates {
            x: self.x * rhs as i64,
            y: self.y * rhs as i64,
        }
    }
}

const COST_BUTTON_A: u64 = 3;
const COST_BUTTON_B: u64 = 1;

#[derive(Debug, Clone, Copy)]
struct Machine {
    a_rel_move: Coordinates,
    b_rel_move: Coordinates,
    prize: Coordinates,
}

const PRIZE_OFFSET: Coordinates = Coordinates {
    x: 10000000000000,
    y: 10000000000000,
};

fn parse_input<'a>(lines: impl Iterator<Item = &'a str>) -> Vec<Machine> {
    let button_regex =
        regex::Regex::new("Button ([AB]): X\\+([0-9]+), Y\\+([0-9]+)").expect("Valid regex");
    let parse_button_line = move |line: &str| {
        button_regex.captures(line).map(|cap| {
            let x = cap.get(2).unwrap().as_str().parse().unwrap();
            let y = cap.get(3).unwrap().as_str().parse().unwrap();
            Coordinates { x, y }
        })
    };

    let prize_regex = regex::Regex::new("Prize: X=([0-9]+), Y=([0-9]+)").expect("Valid regex");
    let parse_prize_line = move |line: &str| {
        prize_regex.captures(line).map(|cap| {
            let x = cap.get(1).unwrap().as_str().parse().unwrap();
            let y = cap.get(2).unwrap().as_str().parse().unwrap();
            Coordinates { x, y }
        })
    };

    lines
        .tuples()
        .map(|(l1, l2, l3, l4)| {
            assert_eq!(l4, "");
            Machine {
                a_rel_move: parse_button_line(l1).unwrap(),
                b_rel_move: parse_button_line(l2).unwrap(),
                prize: parse_prize_line(l3).unwrap(),
            }
        })
        .collect()
}

fn min_price_steps(machine: &Machine) -> Option<(u64, u64)> {
    let xa = machine.a_rel_move.x;
    let ya = machine.a_rel_move.y;
    let xb = machine.b_rel_move.x;
    let yb = machine.b_rel_move.y;
    let x = machine.prize.x;
    let y = machine.prize.y;

    let a = (yb * x - xb * y) / (xa * yb - xb * ya);
    let b = (xa * y - ya * x) / (xa * yb - xb * ya);

    if x == xa * a + xb * b && y == ya * a + yb * b {
        Some((a as u64, b as u64))
    } else {
        None
    }
}

fn max_prizes_min_tokens(machines: &[Machine]) -> u64 {
    machines
        .iter()
        .filter_map(|machine| {
            let (a_count, b_count) = min_price_steps(machine)?;
            Some(a_count * COST_BUTTON_A + b_count * COST_BUTTON_B)
        })
        .sum()
}

fn apply_offset(machines: impl IntoIterator<Item = Machine>) -> Vec<Machine> {
    machines
        .into_iter()
        .map(|machine| Machine {
            a_rel_move: machine.a_rel_move,
            b_rel_move: machine.b_rel_move,
            prize: machine.prize + PRIZE_OFFSET,
        })
        .collect()
}

fn main() {
    let lines = std::io::stdin()
        .lines()
        .map(Result::unwrap)
        .chain(once(String::new()))
        .collect::<Vec<_>>();
    let machines = apply_offset(parse_input(lines.iter().map(|s| s.as_str())));
    let result = max_prizes_min_tokens(&machines);
    println!("Min tokens: {}", result);
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    #[test]
    fn sample() {
        let sample_input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";

        let input = parse_input(sample_input.lines());
        assert_eq!(super::max_prizes_min_tokens(&input), 480);
    }
}
