use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::io::stdin;
use std::ops::{Add, Mul, Sub};

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

impl Sub for Coordinates {
    type Output = Coordinates;

    fn sub(self, rhs: Self) -> Self::Output {
        Coordinates {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
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

fn get_antennas(matrix: &[Vec<char>]) -> HashMap<char, Vec<Coordinates>> {
    matrix
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter().enumerate().filter_map(move |(x, &c)| {
                c.is_ascii_alphanumeric().then_some((
                    c,
                    Coordinates {
                        x: x as i64,
                        y: y as i64,
                    },
                ))
            })
        })
        .into_group_map()
}

fn get_antinodes(field_size: Coordinates, antennas: &[Coordinates]) -> HashSet<Coordinates> {
    antennas
        .iter()
        .copied()
        .tuple_combinations()
        .flat_map(|(a, b)| {
            let delta = a - b;
            let antinode_1 = a + delta;
            let antinode_2 = b - delta;
            [antinode_1, antinode_2]
        })
        .filter(|antinode| antinode.is_within_bounds(field_size))
        .collect()
}

fn main() {
    let matrix = parse_char_matrix();

    let field_size = get_field_size(&matrix);
    let antennas = get_antennas(&matrix);

    let antinodes = antennas
        .values()
        .flat_map(|antennas| get_antinodes(field_size, antennas))
        .collect::<HashSet<_>>();

    println!("Antinodes: {}", antinodes.len());
}
