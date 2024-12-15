use itertools::Itertools;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

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

fn tree_heuristic(
    robot_counts_at_coord: &HashMap<Coordinates, usize>,
    bounds: Coordinates,
) -> bool {
    const NEIGHBOR_THRESHOLD: usize = 2;
    const BOT_THRESHOLD_PERCENT: usize = 20;
    const NEIGHBOR_POSITIONS: [Coordinates; 4] = [
        Coordinates { x: 1, y: 0 },
        Coordinates { x: -1, y: 0 },
        Coordinates { x: 0, y: 1 },
        Coordinates { x: 0, y: -1 },
    ];

    let bot_threshold =
        (robot_counts_at_coord.values().sum::<usize>() * BOT_THRESHOLD_PERCENT) / 100;

    let robots_with_neighbors = robot_counts_at_coord
        .keys()
        .filter(|&&bot| {
            NEIGHBOR_POSITIONS
                .iter()
                .filter(|&&neighbor| {
                    robot_counts_at_coord
                        .get(&(bot.wrapping_add_bounds(neighbor, bounds)))
                        .is_some()
                })
                .count()
                > NEIGHBOR_THRESHOLD
        })
        .count();

    robots_with_neighbors > bot_threshold
}

fn main() {
    let lines = std::io::stdin()
        .lines()
        .map(Result::unwrap)
        .take_while(|line| !line.is_empty())
        .collect::<Vec<_>>();
    let robots = parse_input(lines.iter().map(|s| s.as_str()));
    let bounds = Coordinates { x: 101, y: 103 };
    let loop_after = bounds.x * bounds.y;

    for seconds in 0..loop_after {
        let robots_res = robots
            .iter()
            .map(|r| robot_position_after_n_steps(*r, seconds, bounds))
            .counts();
        if !tree_heuristic(&robots_res, bounds) {
            continue;
        }

        println!("Robots after {} seconds:", seconds);
        debug_print(&robots_res, bounds);

        println!();
        // stdin().read_line(&mut String::new()).unwrap();
    }
}

fn debug_print(robots_on_coords: &HashMap<Coordinates, usize>, bounds: Coordinates) {
    for y in 0..bounds.y {
        for x in 0..bounds.x {
            let coords = Coordinates { x, y };

            if let Some(num_robots) = robots_on_coords.get(&coords) {
                print!("{}", num_robots);
            } else {
                print!(" ");
            }
        }
        println!();
    }
}
