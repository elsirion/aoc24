use std::collections::HashSet;
use std::io::stdin;
use std::ops::{Add, Mul};
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coordinates {
    x: i64,
    y: i64,
}

impl Coordinates {
    fn is_within_bounds(&self, bounds: Coordinates) -> bool {
        self.x >= 0 && self.y >= 0 && self.x < bounds.x && self.y < bounds.y
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

impl Mul<i64> for Coordinates {
    type Output = Coordinates;

    fn mul(self, rhs: i64) -> Self::Output {
        Coordinates {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn to_move(&self) -> Coordinates {
        match self {
            Direction::Up => Coordinates { x: 0, y: -1 },
            Direction::Down => Coordinates { x: 0, y: 1 },
            Direction::Left => Coordinates { x: -1, y: 0 },
            Direction::Right => Coordinates { x: 1, y: 0 },
        }
    }

    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

fn parse_char_matrix() -> Vec<Vec<char>> {
    let matrix: Vec<Vec<char>> = stdin().lines()
        .map(|line_res|
            line_res.expect("stream error")
                .chars()
                .collect()
        )
        .collect();

    assert!(matrix.iter().map(|row| row.len()).all_equal());

    matrix
}

fn get_field_size(matrix: &[Vec<char>]) -> Coordinates {
    assert!(matrix.iter().map(|row| row.len()).all_equal());
    let width = matrix[0].len();
    let height = matrix.len();

    Coordinates {
        x: width as i64,
        y: height as i64,
    }
}

fn get_guard_position(matrix: &[Vec<char>]) -> Coordinates {
    let (x, y) = matrix.iter().enumerate().find_map(|(y, row)| {
        row.iter().position(|&c| c == '^').map(|x| (x, y))
    }).expect("no guard found");

    Coordinates {
        x: x as i64,
        y: y as i64,
    }
}

fn get_obstacles(matrix: &[Vec<char>]) -> HashSet<Coordinates> {
    matrix.iter().enumerate().flat_map(|(y, row)| {
        row.iter().enumerate().filter_map(move |(x, &c)| {
            if c == '#' {
                Some(Coordinates { x: x as i64, y: y as i64 })
            } else {
                None
            }
        })
    }).collect()
}

fn trace_iter(obstacles: HashSet<Coordinates>, start_position: Coordinates, start_direction: Direction) -> impl Iterator<Item = (Direction, Coordinates)> {
    let mut current_pos = start_position;
    let mut direction = Direction::Up;

    let steps = std::iter::repeat(()).filter_map(move |()| {
        let next_pos = current_pos + direction.to_move();

        if obstacles.contains(&next_pos) {
            direction = direction.turn_right();
            return None;
        }
        current_pos = next_pos;

        Some((direction, next_pos))
    });

    std::iter::once((start_direction, start_position)).chain(steps)
}

fn generate_trace(field_size: Coordinates, obstacles: HashSet<Coordinates>, start_position: Coordinates, start_direction: Direction) -> Vec<(Direction, Coordinates)> {
    trace_iter(obstacles, start_position, start_direction)
        .take_while(|(_, coords)| coords.is_within_bounds(field_size))
        .collect()
}

fn possible_diversion_points(field_size: Coordinates, trace: Vec<(Direction, Coordinates)>, obstacles: &HashSet<Coordinates>) -> Vec<Coordinates> {
    let Some((start_direction, start_position)) = trace.first().copied() else {
        return vec![];
    };

    trace.into_iter().skip(1)
        .map(|(_, coords)| coords)
        .unique()
        .filter_map(|potential_obstacle| {
            let mut obstacles = obstacles.clone();
            assert!(obstacles.insert(potential_obstacle));

            let is_loop = !trace_iter(obstacles, start_position, start_direction)
                .take_while(|(_, coords)| coords.is_within_bounds(field_size))
                .all_unique();

            is_loop.then_some(potential_obstacle)
        }).collect()
}

fn main() {
    let matrix = parse_char_matrix();

    let field_size = get_field_size(&matrix);
    let guard_pos = get_guard_position(&matrix);
    let obstacles = get_obstacles(&matrix);

    let trace = generate_trace(field_size, obstacles.iter().copied().collect(), guard_pos, Direction::Up);
    println!("Unique visited fields: {}", trace.iter().map(|(_, coords)| coords).unique().count());

    let possible_diversion_points = possible_diversion_points(field_size, trace, &obstacles).len();

    print!("Diversion points: {possible_diversion_points}");
}