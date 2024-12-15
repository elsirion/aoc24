use itertools::Itertools;
use std::fmt::{Debug, Formatter};
use std::iter::once;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Coordinates {
    x: i64,
    y: i64,
}

impl Coordinates {
    fn wrapping_mul_bounds(&self, scalar: i64, bounds: Coordinates) -> Coordinates {
        let x = (self.x * scalar).rem_euclid(bounds.x);
        let y = (self.y * scalar).rem_euclid(bounds.y);
        Coordinates { x, y }
    }

    fn wrapping_add_bounds(&self, other: Coordinates, bounds: Coordinates) -> Coordinates {
        let x = (self.x + other.x).rem_euclid(bounds.x);
        let y = (self.y + other.y).rem_euclid(bounds.y);
        Coordinates { x, y }
    }
}

impl Debug for Coordinates {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(x={}, y={})", self.x, self.y)
    }
}

#[derive(Debug, Clone, Copy)]
struct Robot {
    position: Coordinates,
    velocity: Coordinates,
}

fn parse_input<'a>(lines: impl Iterator<Item = &'a str>) -> Vec<Robot> {
    let robot_regex =
        regex::Regex::new("p=(-?\\d+),(-?\\d+)\\s+v=(-?\\d+),(-?\\d+)").expect("Valid regex");
    let parse_robot_line = move |line: &str| {
        robot_regex.captures(line).map(|cap| {
            let position = {
                let x = cap.get(1).unwrap().as_str().parse().unwrap();
                let y = cap.get(2).unwrap().as_str().parse().unwrap();
                Coordinates { x, y }
            };
            let velocity = {
                let x = cap.get(3).unwrap().as_str().parse().unwrap();
                let y = cap.get(4).unwrap().as_str().parse().unwrap();
                Coordinates { x, y }
            };
            Robot { position, velocity }
        })
    };

    lines.flat_map(parse_robot_line).collect()
}

fn robot_position_after_n_steps(robot: Robot, n: i64, bounds: Coordinates) -> Coordinates {
    robot
        .position
        .wrapping_add_bounds(robot.velocity.wrapping_mul_bounds(n as i64, bounds), bounds)
}

fn safety_score_after_n_steps(robots: &[Robot], n: i64, bounds: Coordinates) -> usize {
    let coords_to_quadrant = |coords: Coordinates| -> Option<u8> {
        let x_boundary = bounds.x / 2;
        let y_boundary = bounds.y / 2;
        let x = if coords.x < x_boundary {
            0
        } else if coords.x > x_boundary {
            1
        } else {
            return None;
        };
        let y = if coords.y < y_boundary {
            0
        } else if coords.y > y_boundary {
            1
        } else {
            return None;
        };
        Some(x + y * 2)
    };

    robots
        .iter()
        .map(|r| dbg!(robot_position_after_n_steps(*r, n, bounds)))
        .counts_by(coords_to_quadrant)
        .into_iter()
        .filter_map(|(key, value)| key.map(|_| value))
        .product::<usize>()
}

fn main() {
    let lines = std::io::stdin()
        .lines()
        .map(Result::unwrap)
        .chain(once(String::new()))
        .collect::<Vec<_>>();
    let robots = parse_input(lines.iter().map(|s| s.as_str()));
    let bounds = Coordinates { x: 101, y: 103 };
    let safety_score = safety_score_after_n_steps(&robots, 100, bounds);
    println!("Safety score: {}", safety_score);
}

fn debug_print(robots: impl IntoIterator<Item = Coordinates>, bounds: Coordinates) {
    let robots_on_coords = dbg!(robots.into_iter().counts());
    let x_mid = bounds.x / 2;
    let y_mid = bounds.y / 2;

    for y in 0..bounds.y {
        for x in 0..bounds.x {
            let coords = Coordinates { x, y };

            if x == x_mid || y == y_mid {
                print!(" ");
                continue;
            } else if let Some(num_robots) = robots_on_coords.get(&coords) {
                print!("{}", num_robots);
            } else {
                print!(".");
            }
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use crate::{debug_print, parse_input, robot_position_after_n_steps, Coordinates};

    #[test]
    fn sample() {
        let sample_board_size = Coordinates { x: 11, y: 7 };
        let sample_input = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
";

        let robots = parse_input(sample_input.lines());
        assert_eq!(robots.len(), 12);

        let robots_iter = robots
            .iter()
            .map(|r| robot_position_after_n_steps(*r, 100, sample_board_size));
        debug_print(robots_iter, sample_board_size);

        let safety_score = super::safety_score_after_n_steps(&robots, 100, sample_board_size);
        assert_eq!(safety_score, 12)
    }
}
