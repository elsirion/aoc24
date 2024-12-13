use daggy::{Dag, NodeIndex};
use itertools::Itertools;
use petgraph::visit::Dfs;
use std::collections::HashSet;
use std::fmt::{Debug, Formatter};
use std::io::stdin;
use std::ops::Add;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Coordinates {
    x: i64,
    y: i64,
}

impl Coordinates {
    fn is_within_bounds(&self, bounds: Coordinates) -> bool {
        self.x >= 0 && self.y >= 0 && self.x < bounds.x && self.y < bounds.y
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

fn parse_topo_map() -> Vec<Vec<u8>> {
    let matrix: Vec<Vec<u8>> = stdin()
        .lines()
        .map(|line_res| {
            line_res
                .expect("stream error")
                .chars()
                .map(|c| c.to_digit(10).expect("invalid digit") as u8)
                .collect()
        })
        .collect();

    assert!(matrix.iter().map(|row| row.len()).all_equal());

    matrix
}

fn get_field_size(matrix: &[Vec<u8>]) -> Coordinates {
    assert!(matrix.iter().map(|row| row.len()).all_equal());
    let width = matrix[0].len();
    let height = matrix.len();

    Coordinates {
        x: width as i64,
        y: height as i64,
    }
}

fn get_paths_from_point(
    matrix: &[Vec<u8>],
    start_point: Coordinates,
) -> impl Iterator<Item = (Coordinates, Coordinates)> + '_ {
    const DIRECTIONS: [Coordinates; 4] = [
        Coordinates { x: 1, y: 0 },
        Coordinates { x: -1, y: 0 },
        Coordinates { x: 0, y: 1 },
        Coordinates { x: 0, y: -1 },
    ];

    let bounds = get_field_size(matrix);
    let point_height = matrix[start_point.y as usize][start_point.x as usize];
    DIRECTIONS
        .iter()
        .map(move |dir| start_point + *dir)
        .filter_map(move |point| {
            if !point.is_within_bounds(bounds) {
                return None;
            }

            let height = matrix[point.y as usize][point.x as usize];
            if height == point_height + 1 {
                Some((start_point, point))
            } else {
                None
            }
        })
}

fn coords_iter(bounds: Coordinates) -> impl Iterator<Item = Coordinates> {
    (0..bounds.y)
        .cartesian_product(0..bounds.x)
        .map(|(y, x)| Coordinates { x, y })
}

fn matrix_to_dag(matrix: &[Vec<u8>]) -> Dag<(), ()> {
    let bounds = get_field_size(matrix);
    let coords_to_idx = |coords: Coordinates| (coords.y * bounds.x + coords.x) as u32;
    let edges = coords_iter(get_field_size(matrix))
        .flat_map(|coords| {
            get_paths_from_point(matrix, coords)
                .map(move |(from, to)| (coords_to_idx(from), coords_to_idx(to), ()))
        })
        .collect::<Vec<_>>();

    Dag::from_edges(edges).expect("Is valid graph")
}

fn get_indices_of_value(matrix: &[Vec<u8>], value: u8) -> Vec<u32> {
    matrix
        .iter()
        .flat_map(|row| row.iter().copied())
        .enumerate()
        .filter_map(|(idx, height)| (height == value).then_some(idx as u32))
        .collect()
}

fn get_start_indices(matrix: &[Vec<u8>]) -> Vec<u32> {
    get_indices_of_value(matrix, 0)
}

fn get_finish_indices(matrix: &[Vec<u8>]) -> Vec<u32> {
    get_indices_of_value(matrix, 9)
}

fn reachable_finish_indices(
    dag: &Dag<(), ()>,
    start_index: u32,
    finish_indices: &HashSet<u32>,
) -> Vec<u32> {
    let graph = dag.graph();
    let mut dfs = Dfs::new(graph, NodeIndex::new(start_index as usize));
    let mut reachable_finish_indices = Vec::new();

    while let Some(node) = dfs.next(graph) {
        let node_idx = node.index() as u32;
        if finish_indices.contains(&node_idx) {
            reachable_finish_indices.push(node_idx);
        }
    }

    reachable_finish_indices
}

fn main() {
    let matrix = parse_topo_map();
    let dag = matrix_to_dag(&matrix);

    let start_indices = get_start_indices(&matrix);
    let finish_indices_set = get_finish_indices(&matrix)
        .into_iter()
        .collect::<HashSet<_>>();

    let total_score = start_indices
        .iter()
        .map(|start_index| reachable_finish_indices(&dag, *start_index, &finish_indices_set).len())
        .sum::<usize>();

    println!("Total score: {}", total_score);
}
