use itertools::Itertools;
use std::fmt::{Debug, Formatter};
use std::io::stdin;
use std::ops::{Add, Mul};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
enum FieldState {
    Empty,
    RBox,
    LBox,
    Wall,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '^' => Some(Direction::Up),
            'v' => Some(Direction::Down),
            '<' => Some(Direction::Left),
            '>' => Some(Direction::Right),
            _ => None,
        }
    }

    fn to_coords(&self) -> Coordinates {
        match self {
            Direction::Up => Coordinates::new(0, -1),
            Direction::Down => Coordinates::new(0, 1),
            Direction::Left => Coordinates::new(-1, 0),
            Direction::Right => Coordinates::new(1, 0),
        }
    }
}

#[derive(Clone)]
struct Map {
    matrix: Vec<Vec<FieldState>>,
    robot_coords: Coordinates,
}

impl Map {
    fn from_lines<'a>(lines: impl Iterator<Item = &'a str>) -> Self {
        let mut robot = None;
        let matrix: Vec<Vec<FieldState>> = lines
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .flat_map(|(x, c)| match c {
                        '.' => [FieldState::Empty, FieldState::Empty],
                        'O' => [FieldState::LBox, FieldState::RBox],
                        '#' => [FieldState::Wall, FieldState::Wall],
                        '@' => {
                            robot = Some(Coordinates::new((x * 2) as i64, y as i64));
                            [FieldState::Empty, FieldState::Empty]
                        }
                        _ => panic!("invalid character"),
                    })
                    .collect()
            })
            .collect();

        assert!(matrix.iter().map(|row| row.len()).all_equal());

        Map {
            matrix,
            robot_coords: robot.expect("no robot found"),
        }
    }

    fn get_field(&self, coords: Coordinates) -> Option<FieldState> {
        let x: usize = coords.x.try_into().ok()?;
        let y: usize = coords.y.try_into().ok()?;
        self.matrix.get(y)?.get(x).copied()
    }

    /// Tries to push the object at the given coordinates in the given direction. Returns if it was successful.
    fn try_push(&mut self, coords: Coordinates, direction: Direction) -> bool {
        let field_state = self.get_field(coords).expect("invalid coordinates");

        match field_state {
            FieldState::Empty => true,
            FieldState::Wall => false,
            FieldState::LBox | FieldState::RBox => match direction {
                Direction::Up | Direction::Down => self.try_push_box_vertically(coords, direction),
                Direction::Left | Direction::Right => {
                    self.try_push_box_horizontally(coords, direction)
                }
            },
        }
    }

    fn try_push_box_vertically(&mut self, coords: Coordinates, direction: Direction) -> bool {
        debug_assert!(matches!(direction, Direction::Up | Direction::Down));
        debug_assert!(matches!(
            self.get_field(coords).expect("invalid coordinates"),
            FieldState::LBox | FieldState::RBox
        ));

        if self.vert_pushable(coords, direction) {
            self.vert_push(coords, direction);
            true
        } else {
            false
        }
    }

    fn vert_pushable(&self, coords: Coordinates, direction: Direction) -> bool {
        let box_part = self.get_field(coords).expect("invalid coordinates");
        let second_coords = if box_part == FieldState::LBox {
            coords + Coordinates::new(1, 0)
        } else {
            coords + Coordinates::new(-1, 0)
        };

        // TODO: optimize with early return if necessary
        let primary_push_coords = coords + direction.to_coords();
        let primary_pushable = match self.get_field(primary_push_coords).expect("out of bounds") {
            FieldState::Empty => true,
            FieldState::Wall => false,
            FieldState::RBox | FieldState::LBox => {
                self.vert_pushable(primary_push_coords, direction)
            }
        };

        let secondary_push_coords = second_coords + direction.to_coords();
        let secondary_pushable = match self
            .get_field(secondary_push_coords)
            .expect("out of bounds")
        {
            FieldState::Empty => true,
            FieldState::Wall => false,
            FieldState::RBox | FieldState::LBox => {
                self.vert_pushable(secondary_push_coords, direction)
            }
        };

        primary_pushable && secondary_pushable
    }

    fn vert_push(&mut self, coords: Coordinates, direction: Direction) {
        let box_part = self.get_field(coords).expect("invalid coordinates");
        let second_coords = if box_part == FieldState::LBox {
            coords + Coordinates::new(1, 0)
        } else {
            coords + Coordinates::new(-1, 0)
        };

        let primary_push_coords = coords + direction.to_coords();
        let secondary_push_coords = second_coords + direction.to_coords();

        if matches!(
            self.get_field(primary_push_coords).expect("out of bounds"),
            FieldState::LBox | FieldState::RBox
        ) {
            self.vert_push(primary_push_coords, direction);
        }

        if matches!(
            self.get_field(secondary_push_coords)
                .expect("out of bounds"),
            FieldState::LBox | FieldState::RBox
        ) {
            self.vert_push(secondary_push_coords, direction);
        }

        self.matrix[primary_push_coords.y as usize][primary_push_coords.x as usize] =
            self.matrix[coords.y as usize][coords.x as usize];
        self.matrix[coords.y as usize][coords.x as usize] = FieldState::Empty;
        self.matrix[secondary_push_coords.y as usize][secondary_push_coords.x as usize] =
            self.matrix[second_coords.y as usize][second_coords.x as usize];
        self.matrix[second_coords.y as usize][second_coords.x as usize] = FieldState::Empty;
    }

    fn try_push_box_horizontally(&mut self, coords: Coordinates, direction: Direction) -> bool {
        debug_assert!(matches!(direction, Direction::Left | Direction::Right));
        debug_assert!(matches!(
            self.get_field(coords).expect("invalid coordinates"),
            FieldState::LBox | FieldState::RBox
        ));

        let bounds = self.bounds();
        let mut coords_iter = (0i64..)
            .map(|i| (i, coords + direction.to_coords() * i))
            .take_while(|(_, c)| c.is_within_bounds(bounds));

        let (dist, next_non_box) = coords_iter
            .find(|(_dist, coords)| {
                let field_state = self.get_field(*coords).expect("invalid coordinates");
                !matches!(field_state, FieldState::LBox | FieldState::RBox)
            })
            .expect("no non-box field found");

        match self.get_field(next_non_box) {
            Some(FieldState::Empty) => {
                let rev_iter = (0..=dist)
                    .map(|i| coords + direction.to_coords() * i)
                    .rev()
                    .tuple_windows();
                for (to, from) in rev_iter {
                    self.matrix[to.y as usize][to.x as usize] =
                        self.matrix[from.y as usize][from.x as usize];
                }
                self.matrix[coords.y as usize][coords.x as usize] = FieldState::Empty;
                true
            }
            Some(FieldState::Wall) => false,
            None | Some(FieldState::LBox) | Some(FieldState::RBox) => {
                unreachable!()
            }
        }
    }

    fn bounds(&self) -> Coordinates {
        let width = self.matrix[0].len();
        let height = self.matrix.len();

        Coordinates {
            x: width as i64,
            y: height as i64,
        }
    }

    fn transform_map(&mut self, instructions: &[Direction]) {
        for &direction in instructions {
            let new_coords = self.robot_coords + direction.to_coords();
            if self.try_push(new_coords, direction) {
                self.robot_coords = new_coords;
            }
        }
    }

    fn box_coordinate_sums(&self) -> usize {
        self.matrix
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter().enumerate().filter_map(move |(x, field)| {
                    if *field == FieldState::LBox {
                        Some(y * 100 + x)
                    } else {
                        None
                    }
                })
            })
            .sum::<usize>()
    }
}

impl Debug for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (y, row) in self.matrix.iter().enumerate() {
            for (x, field) in row.iter().enumerate() {
                if Coordinates::new(x as i64, y as i64) == self.robot_coords {
                    write!(f, "@")?;
                } else {
                    let c = match field {
                        FieldState::Empty => '.',
                        FieldState::LBox => '[',
                        FieldState::RBox => ']',
                        FieldState::Wall => '#',
                    };
                    write!(f, "{}", c)?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn parse_instructions<'a>(lines: impl Iterator<Item = &'a str>) -> Vec<Direction> {
    lines
        .flat_map(|line| line.chars().filter_map(Direction::from_char))
        .collect()
}

fn parse_input<'a>(lines: impl Iterator<Item = &'a str>) -> (Map, Vec<Direction>) {
    let mut lines = lines;
    let map = Map::from_lines(&mut lines.by_ref().take_while(|line| !line.is_empty()));
    let instructions = parse_instructions(lines);
    (map, instructions)
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

impl Debug for Coordinates {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(x={}, y={})", self.x, self.y)
    }
}

fn main() {
    let lines = stdin().lines().map(Result::unwrap).collect::<Vec<_>>();
    let (mut map, instructions) = parse_input(lines.iter().map(String::as_str));
    map.transform_map(&instructions);
    println!("Coordinate sums: {}", map.box_coordinate_sums());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_large() {
        let input = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
";

        let (mut map, instructions) = parse_input(input.lines());
        map.transform_map(&instructions);
        assert_eq!(map.box_coordinate_sums(), 9021);
    }
}
