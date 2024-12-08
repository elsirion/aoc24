use itertools::Itertools;
use std::io::stdin;

#[derive(Debug, Clone, Copy)]
struct Coordinates {
    x: i64,
    y: i64,
}

impl Coordinates {
    fn checked_bounded_add(&self, other: Coordinates, max: Coordinates) -> Option<Coordinates> {
        let new_x = self.x.checked_add(other.x)?;
        let new_y = self.y.checked_add(other.y)?;

        if new_x > max.x || new_y > max.y || new_x < 0 || new_y < 0 {
            None
        } else {
            Some(Coordinates { x: new_x, y: new_y })
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

fn find_xmas_times(matrix: Vec<Vec<char>>) -> usize {
    let a_coordinates = matrix
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter().enumerate().filter_map(move |(x, c)| {
                if *c == 'A' {
                    Some(Coordinates {
                        x: x as i64,
                        y: y as i64,
                    })
                } else {
                    None
                }
            })
        })
        .flatten()
        .collect::<Vec<_>>();

    a_coordinates
        .into_iter()
        .filter_map(|x_coordinate| check_xmas_from_coordinate(&matrix, x_coordinate).then_some(()))
        .count()
}

fn get_char_at(matrix: &[Vec<char>], coords: Coordinates) -> Option<char> {
    matrix
        .get(coords.y as usize)?
        .get(coords.x as usize)
        .copied()
}

fn check_xmas_from_coordinate(matrix: &[Vec<char>], coordinates: Coordinates) -> bool {
    const DIAG1: [Coordinates; 2] = [Coordinates { x: 1, y: 1 }, Coordinates { x: -1, y: -1 }];
    const DIAG2: [Coordinates; 2] = [Coordinates { x: 1, y: -1 }, Coordinates { x: -1, y: 1 }];
    const MATCH: [char; 2] = ['M', 'S'];

    let max = Coordinates {
        x: (matrix[0].len() - 1) as i64,
        y: (matrix.len() - 1) as i64,
    };

    let check_match = |rel_coords: &[Coordinates; 2],
                       match_iter: Box<dyn Iterator<Item = char>>| {
        rel_coords.iter().zip(match_iter).all(|(rel_coords, c)| {
            let Some(abs_coords) = coordinates.checked_bounded_add(*rel_coords, max) else {
                return false;
            };
            get_char_at(matrix, abs_coords) == Some(c)
        })
    };

    [DIAG1, DIAG2].iter().all(|rel_coords| {
        check_match(rel_coords, Box::new(MATCH.iter().copied()))
            || check_match(rel_coords, Box::new(MATCH.iter().rev().copied()))
    })
}

fn main() {
    let matrix = parse_char_matrix();
    let xmas_times = find_xmas_times(matrix);
    println!("XMAS times: {xmas_times}");
}
