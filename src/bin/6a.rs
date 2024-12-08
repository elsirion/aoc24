use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::io::stdin;
use std::ops::Add;

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
    let matrix: Vec<Vec<char>> = stdin()
        .lines()
        .map(|line_res| line_res.expect("stream error").chars().collect())
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
    let (x, y) = matrix
        .iter()
        .enumerate()
        .find_map(|(y, row)| row.iter().position(|&c| c == '^').map(|x| (x, y)))
        .expect("no guard found");

    Coordinates {
        x: x as i64,
        y: y as i64,
    }
}

fn get_obstacles(matrix: &[Vec<char>]) -> HashSet<Coordinates> {
    matrix
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter().enumerate().filter_map(move |(x, &c)| {
                if c == '#' {
                    Some(Coordinates {
                        x: x as i64,
                        y: y as i64,
                    })
                } else {
                    None
                }
            })
        })
        .collect()
}

fn generate_trace(
    field_size: Coordinates,
    obstacles: HashSet<Coordinates>,
    start_position: Coordinates,
) -> Vec<Coordinates> {
    let mut trace = vec![start_position];
    let mut current_pos = start_position;
    let mut direction = Direction::Up;

    loop {
        let next_pos = current_pos + direction.to_move();

        if !next_pos.is_within_bounds(field_size) {
            break;
        }

        if obstacles.contains(&next_pos) {
            direction = direction.turn_right();
            continue;
        }

        trace.push(next_pos);
        current_pos = next_pos;
    }

    trace
}

fn main() {
    let matrix = parse_char_matrix();

    let field_size = get_field_size(&matrix);
    let guard_pos = get_guard_position(&matrix);
    let obstacles = get_obstacles(&matrix);

    let trace = generate_trace(field_size, obstacles.iter().copied().collect(), guard_pos);
    let unique_positions = trace.iter().collect::<HashSet<_>>();

    print!("Unique positions: {}", unique_positions.len());
}
