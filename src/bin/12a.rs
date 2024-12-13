use itertools::Itertools;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::fmt::{Debug, Formatter};
use std::io::stdin;
use std::ops::Add;

fn parse_char_matrix() -> Vec<Vec<char>> {
    let matrix: Vec<Vec<char>> = stdin()
        .lines()
        .map(|line_res| line_res.expect("stream error").chars().collect())
        .collect();

    assert!(matrix.iter().map(|row| row.len()).all_equal());

    matrix
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
struct Coordinates {
    x: i64,
    y: i64,
}

impl Coordinates {
    const fn new(x: i64, y: i64) -> Self {
        Coordinates { x, y }
    }

    fn is_within_bounds(&self, bounds: Coordinates) -> bool {
        self.x >= 0 && self.y >= 0 && self.x < bounds.x && self.y < bounds.y
    }
}

impl Debug for Coordinates {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(x={}, y={})", self.x, self.y)
    }
}

fn coords_iter(bounds: Coordinates) -> impl Iterator<Item = Coordinates> {
    (0..bounds.y)
        .cartesian_product(0..bounds.x)
        .map(|(y, x)| Coordinates { x, y })
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

fn get_field_size(matrix: &[Vec<char>]) -> Coordinates {
    assert!(matrix.iter().map(|row| row.len()).all_equal());
    let width = matrix[0].len();
    let height = matrix.len();

    Coordinates {
        x: width as i64,
        y: height as i64,
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct FieldDimensions {
    area: usize,
    circumference: usize,
}

fn area_circumference_map(matrix: &[Vec<char>]) -> HashMap<(Coordinates, char), FieldDimensions> {
    const NEIGHBORS: [Coordinates; 4] = [
        Coordinates::new(-1, 0),
        Coordinates::new(1, 0),
        Coordinates::new(0, -1),
        Coordinates::new(0, 1),
    ];

    let size = get_field_size(matrix);
    let mut all_coords = coords_iter(size).collect::<BTreeSet<_>>();
    let mut dimensions = HashMap::<(Coordinates, char), FieldDimensions>::new();

    'patches: while let Some(initial_coords) = all_coords.pop_first() {
        let current_field_type = matrix[initial_coords.y as usize][initial_coords.x as usize];
        let current_dimensions = dimensions
            .entry((initial_coords, current_field_type))
            .or_insert(FieldDimensions {
                area: 1,
                circumference: 0,
            });
        let mut fields_to_check_neighbors_of = vec![initial_coords];

        'fields: while let Some(check_field) = fields_to_check_neighbors_of.pop() {
            'neighbors: for neighbor in NEIGHBORS.iter().map(|&neighbor| check_field + neighbor) {
                if neighbor.is_within_bounds(size)
                    && matrix[neighbor.y as usize][neighbor.x as usize] == current_field_type
                {
                    if all_coords.remove(&neighbor) {
                        current_dimensions.area += 1;
                        fields_to_check_neighbors_of.push(neighbor);
                    }
                } else {
                    current_dimensions.circumference += 1;
                }
            }
        }
    }

    dimensions
}

fn main() {
    let numbers = parse_char_matrix();
    let dimensions = area_circumference_map(&numbers);

    let cost = dimensions
        .values()
        .map(|dim| dim.area * dim.circumference)
        .sum::<usize>();

    println!("Total cost: {}", cost);
}
