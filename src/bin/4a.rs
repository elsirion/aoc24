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

    fn scalar_mul(&self, scalar: i64) -> Coordinates {
        Coordinates {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Angle {
    Horizontal,
    Vertical,
    DiagDown,
    DiagUp,
}

#[derive(Debug, Clone, Copy)]
struct Direction {
    angle: Angle,
    backwards: bool,
}

fn all_directions() -> Vec<Direction> {
    const ANGLES: [Angle; 4] = [
        Angle::Horizontal,
        Angle::Vertical,
        Angle::DiagDown,
        Angle::DiagUp,
    ];

    ANGLES
        .iter()
        .flat_map(|angle: &Angle| {
            [false, true].iter().map(move |backwards| Direction {
                angle: *angle,
                backwards: *backwards,
            })
        })
        .collect()
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
    let x_coordinates = matrix
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter().enumerate().filter_map(move |(x, c)| {
                if *c == 'X' {
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

    let all_directions = all_directions();

    x_coordinates
        .into_iter()
        .map(|x_coordinate| {
            all_directions
                .iter()
                .filter_map(|direction| {
                    check_xmas_from_coordinate(&matrix, x_coordinate, *direction).then_some(())
                })
                .count()
        })
        .sum()
}

fn get_char_at(matrix: &[Vec<char>], coords: Coordinates) -> Option<char> {
    matrix
        .get(coords.y as usize)?
        .get(coords.x as usize)
        .copied()
}

fn get_coordinates_from_point(
    matrix: &[Vec<char>],
    coords: Coordinates,
    direction: Direction,
) -> Option<Vec<Coordinates>> {
    let max = Coordinates {
        x: (matrix[0].len() - 1) as i64,
        y: (matrix.len() - 1) as i64,
    };

    let single_step_forward = match direction.angle {
        Angle::Horizontal => Coordinates { x: 1, y: 0 },
        Angle::Vertical => Coordinates { x: 0, y: 1 },
        Angle::DiagDown => Coordinates { x: 1, y: 1 },
        Angle::DiagUp => Coordinates { x: 1, y: -1 },
    };

    let single_step_directed = if direction.backwards {
        single_step_forward.scalar_mul(-1)
    } else {
        single_step_forward
    };

    (0..4)
        .map(|step| coords.checked_bounded_add(single_step_directed.scalar_mul(step), max))
        .collect::<Option<Vec<_>>>()
}

fn check_xmas_from_coordinate(
    matrix: &[Vec<char>],
    coordinates: Coordinates,
    direction: Direction,
) -> bool {
    const XMAS: &str = "XMAS";

    let Some(xmas_coordinates) = get_coordinates_from_point(matrix, coordinates, direction) else {
        return false;
    };

    xmas_coordinates
        .into_iter()
        .zip(XMAS.chars())
        .all(|(coords, c)| get_char_at(matrix, coords) == Some(c))
}

fn main() {
    let matrix = parse_char_matrix();
    let xmas_times = find_xmas_times(matrix);
    println!("XMAS times: {xmas_times}");
}
