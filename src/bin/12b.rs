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

#[derive(Debug, Clone, Default)]
struct FieldDimensions {
    area: usize,
    fences: Vec<(FenceDirection, Coordinates)>,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum FenceDirection {
    North,
    East,
    South,
    West,
}

impl FenceDirection {
    fn to_rel_coords(&self) -> Coordinates {
        match self {
            FenceDirection::North => Coordinates::new(0, -1),
            FenceDirection::East => Coordinates::new(1, 0),
            FenceDirection::South => Coordinates::new(0, 1),
            FenceDirection::West => Coordinates::new(-1, 0),
        }
    }
}

const ALL_DIRECTIONS: [FenceDirection; 4] = [
    FenceDirection::North,
    FenceDirection::East,
    FenceDirection::South,
    FenceDirection::West,
];

fn field_dimensions(matrix: &[Vec<char>]) -> HashMap<(Coordinates, char), FieldDimensions> {
    let size = get_field_size(matrix);
    let mut all_coords = coords_iter(size).collect::<BTreeSet<_>>();
    let mut dimensions = HashMap::<(Coordinates, char), FieldDimensions>::new();

    'patches: while let Some(initial_coords) = all_coords.pop_first() {
        let current_field_type = matrix[initial_coords.y as usize][initial_coords.x as usize];
        let current_dimensions = dimensions
            .entry((initial_coords, current_field_type))
            .or_default();
        let mut fields_to_check_neighbors_of = vec![initial_coords];

        'fields: while let Some(check_field) = fields_to_check_neighbors_of.pop() {
            current_dimensions.area += 1;
            'neighbors: for neighbor_direction in ALL_DIRECTIONS {
                let neighbor = check_field + neighbor_direction.to_rel_coords();
                if neighbor.is_within_bounds(size)
                    && matrix[neighbor.y as usize][neighbor.x as usize] == current_field_type
                {
                    if all_coords.remove(&neighbor) {
                        fields_to_check_neighbors_of.push(neighbor);
                    }
                } else {
                    current_dimensions
                        .fences
                        .push((neighbor_direction, check_field));
                }
            }
        }
    }

    dimensions
}

fn dimension_to_cost(dim: &FieldDimensions) -> usize {
    let sides = {
        let num_horizontal_segments = {
            let horizontal_fences = dim
                .fences
                .iter()
                .filter(|(dir, _)| matches!(dir, FenceDirection::North | FenceDirection::South))
                .copied()
                .into_group_map_by(|(dir, coords)| (*dir, coords.y));
            horizontal_fences
                .values()
                .map(|coords| num_continuous_sections(coords.iter().map(|(_, coords)| coords.x)))
                .sum::<usize>()
        };
        let num_vertical_segments = {
            let vertical_fences = dim
                .fences
                .iter()
                .filter(|(dir, _)| matches!(dir, FenceDirection::East | FenceDirection::West))
                .copied()
                .into_group_map_by(|(dir, coords)| (*dir, coords.x));
            vertical_fences
                .values()
                .map(|coords| num_continuous_sections(coords.iter().map(|(_, coords)| coords.y)))
                .sum::<usize>()
        };

        num_horizontal_segments + num_vertical_segments
    };

    dim.area * sides
}

fn num_continuous_sections(coords: impl Iterator<Item = i64>) -> usize {
    let num_breaks = coords
        .sorted()
        .tuple_windows()
        .filter(|(a, b)| a.abs_diff(*b) != 1)
        .count();
    num_breaks + 1
}

fn main() {
    let numbers = parse_char_matrix();
    let dimensions = field_dimensions(&numbers);

    let cost = dimensions
        .values()
        .map(|dim| dimension_to_cost(dim))
        .sum::<usize>();

    println!("Total cost: {}", cost);
}
