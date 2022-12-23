#![warn(clippy::pedantic)]
use std::collections::HashMap;
mod utils;
const DAY_ID: utils::DayIdType = 22;

type Clockwise = bool;
type Coord = usize;
type Coords = (Coord, Coord);
type Spots = HashMap<Coords, char>;
type Path = Vec<Action>;

#[derive(Debug)]
struct Board {
    pub spots: Spots,
    pub top_left: Coords,

    row_wraps: Vec<(Coord, Coord)>,
    col_wraps: Vec<(Coord, Coord)>,
}

#[derive(Debug, Copy, Clone)]
enum Action {
    Move(usize),
    RotateLeft,
    RotateRight,
}

impl From<&str> for Action {
    fn from(s: &str) -> Self {
        if s == "L" {
            Self::RotateLeft
        } else if s == "R" {
            Self::RotateRight
        } else {
            Self::Move(s.parse().unwrap())
        }
    }
}

fn parse_input(data: &str) -> (Board, Path) {
    let (board_lines, path_line) = data.split_once("\n\n").unwrap();

    let rows = board_lines.lines().count();
    let cols = board_lines.lines().map(str::len).max().unwrap();

    let mut spots = Spots::new();
    for (row, line) in board_lines.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ['.', '#'].contains(&ch) {
                spots.insert((row, col), ch);
            }
        }
    }

    let spaced_line = path_line.replace('L', " L ").replace('R', " R ");
    let words = spaced_line.split_ascii_whitespace().collect::<Vec<_>>();
    let path = words.iter().copied().map(Into::into).collect();

    let mut top_left = (0, 0);
    for (index, ch) in data.lines().next().unwrap().chars().enumerate() {
        if ch == '.' {
            top_left.1 = index;
            break;
        }
    }

    let mut row_wraps = vec![];
    for row in 0..rows {
        let line = board_lines.lines().nth(row).unwrap();
        let (min, _) = line
            .chars()
            .enumerate()
            .find(|(_i, ch)| ch != &' ')
            .unwrap();
        let (rev_max, _) = line
            .chars()
            .rev()
            .enumerate()
            .find(|(_i, ch)| ch != &' ')
            .unwrap();
        let max = line.len() - rev_max - 1;
        // println!("{min} - {max}");
        row_wraps.push((min, max));
    }

    let mut col_wraps = vec![];
    for col in 0..cols {
        let min = row_wraps
            .iter()
            .enumerate()
            .find(|(_, wrap)| wrap.0 <= col && col <= wrap.1)
            .unwrap()
            .0;

        // println!("{col}, min: {min}");
        let max = row_wraps
            .iter()
            .enumerate()
            .rev()
            .find(|(_, wrap)| wrap.0 <= col && col <= wrap.1)
            .unwrap()
            .0;

        col_wraps.push((min, max));
    }

    (
        Board {
            spots,
            top_left,
            row_wraps,
            col_wraps,
        },
        path,
    )
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    pub fn rotate(self, clockwise: Clockwise) -> Self {
        #[allow(clippy::match_same_arms)]
        match (self, clockwise) {
            (Direction::Left, true) => Direction::Up,
            (Direction::Left, false) => Direction::Down,
            (Direction::Right, true) => Direction::Down,
            (Direction::Right, false) => Direction::Up,
            (Direction::Up, true) => Direction::Right,
            (Direction::Up, false) => Direction::Left,
            (Direction::Down, true) => Direction::Left,
            (Direction::Down, false) => Direction::Right,
        }
    }

    pub fn facing_score(self) -> usize {
        match self {
            Direction::Left => 2,
            Direction::Right => 0,
            Direction::Up => 3,
            Direction::Down => 1,
        }
    }
}

const SIZE: usize = 50;

impl Board {
    fn wrap_simple(&self, (row, col): Coords, direction: Direction) -> (Coords, Direction) {
        let (min, max) = match direction {
            Direction::Left | Direction::Right => self.row_wraps[row],
            Direction::Up | Direction::Down => self.col_wraps[col],
        };

        let coords = match direction {
            Direction::Left => (
                row,
                if col == 0 || col - 1 < min {
                    max
                } else {
                    col - 1
                },
            ),
            Direction::Right => (row, if col + 1 > max { min } else { col + 1 }),
            Direction::Down => (if row + 1 > max { min } else { row + 1 }, col),
            Direction::Up => (
                if row == 0 || row - 1 < min {
                    max
                } else {
                    row - 1
                },
                col,
            ),
        };

        (coords, direction)
    }

    fn wrap_cube((row, col): Coords, direction: Direction) -> (Coords, Direction) {
        // cube unfolded into part 2 data:
        //      n  m
        //     ██████
        //    a█1██2█l
        //     ██████
        //     ███ k
        //    b█3█j
        //   c ███
        //  ██████
        // d█4██5█i
        //  ██████
        //  ███ h
        // e█6█g
        //  ███
        //   f

        // println!("\t{direction:?} from ({row},{col}); min/max {min}/{max}");
        match direction {
            Direction::Left => {
                if row < SIZE && col == SIZE {
                    // A row -> rev D row; right
                    ((3 * SIZE - row - 1, 0), Direction::Right)
                } else if (SIZE..2 * SIZE).contains(&row) && col == SIZE {
                    // B row -> C col; down
                    ((2 * SIZE, row - SIZE), Direction::Down)
                } else if (2 * SIZE..3 * SIZE).contains(&row) && col == 0 {
                    // D row -> rev A row; right
                    ((3 * SIZE - row - 1, SIZE), Direction::Right)
                } else if (3 * SIZE..4 * SIZE).contains(&row) && col == 0 {
                    // E row -> N col; down
                    ((0, row - 2 * SIZE), Direction::Down)
                } else {
                    ((row, col - 1), Direction::Left)
                }
            }
            Direction::Right => {
                if row < SIZE && col == (3 * SIZE - 1) {
                    // L row -> rev I row; left
                    ((3 * SIZE - 1 - row, 2 * SIZE - 1), Direction::Left)
                } else if (SIZE..2 * SIZE).contains(&row) && col == (2 * SIZE - 1) {
                    // J row -> K col; up
                    ((SIZE - 1, row + SIZE), Direction::Up)
                } else if (2 * SIZE..3 * SIZE).contains(&row) && col == (2 * SIZE - 1) {
                    // I row -> rev L row; left
                    ((3 * SIZE - 1 - row, 3 * SIZE - 1), Direction::Left)
                } else if (3 * SIZE..4 * SIZE).contains(&row) && col == (SIZE - 1) {
                    // G row -> rev H col; up
                    ((3 * SIZE - 1, row - 2 * SIZE), Direction::Up)
                } else {
                    ((row, col + 1), Direction::Right)
                }
            }
            Direction::Up => {
                if row == 2 * SIZE && col < SIZE {
                    // C col -> B row; right
                    ((SIZE + col, SIZE), Direction::Right)
                } else if row == 0 && (SIZE..2 * SIZE).contains(&col) {
                    // N col -> E row; right
                    ((col + 2 * SIZE, 0), Direction::Right)
                } else if row == 0 && (2 * SIZE..3 * SIZE).contains(&col) {
                    // M col -> F col; up
                    ((4 * SIZE - 1, col - 2 * SIZE), Direction::Up)
                } else {
                    ((row - 1, col), Direction::Up)
                }
            }
            Direction::Down => {
                if row == (4 * SIZE - 1) && col < SIZE {
                    // F col -> M col; down
                    ((0, col + 2 * SIZE), Direction::Down)
                } else if row == (3 * SIZE - 1) && (SIZE..2 * SIZE).contains(&col) {
                    // H col -> G row; left
                    ((col + 2 * SIZE, SIZE - 1), Direction::Left)
                } else if row == (SIZE - 1) && (2 * SIZE..3 * SIZE).contains(&col) {
                    // K col -> J row; left
                    ((col - SIZE, 2 * SIZE - 1), Direction::Left)
                } else {
                    ((row + 1, col), Direction::Down)
                }
            }
        }
    }

    pub fn walk(
        &self,
        from: Coords,
        steps: usize,
        direction: Direction,
        cube_wrap: bool,
    ) -> (Coords, Direction) {
        let (mut coords, mut direction) = (from, direction);

        'steps: for _ in 0..steps {
            let candidate = if cube_wrap {
                Self::wrap_cube(coords, direction)
            } else {
                self.wrap_simple(coords, direction)
            };
            // println!(
            //     "\tcandidate: {candidate:?} - {:?}",
            //     self.spots.get(&candidate.0)
            // );
            match self.spots.get(&candidate.0) {
                Some('.') => {
                    (coords, direction) = candidate;
                }
                Some('#') => {
                    break 'steps;
                }
                _ => panic!("Fell off the map! {coords:?}"),
            }
        }

        (coords, direction)
    }

    #[cfg(test)]
    pub fn fly_around(from: Coords, direction: Direction) -> (Coords, Direction) {
        let (mut coords, mut direction) = (from, direction);
        for _ in 0..(4 * SIZE) {
            let candidate = Self::wrap_cube(coords, direction);
            (coords, direction) = candidate;
        }
        (coords, direction)
    }

    #[cfg(test)]
    pub fn walk_corner(from: Coords, direction: Direction, clockwise: bool) -> (Coords, Direction) {
        println!("corner walk: {from:?}, {direction:?}, clockwise: {clockwise}");
        let (mut coords, mut direction) = (from, direction);
        for _ in 0..3 {
            println!("\tbefore: {coords:?}, {direction:?}");
            let candidate = Self::wrap_cube(coords, direction);
            (coords, direction) = candidate;
            direction = direction.rotate(clockwise);
            println!("\t after: {coords:?}, {direction:?}");
        }
        (coords, direction)
    }
}

fn traverse_map(board: &Board, path: &Path, cube_wrap: bool) -> usize {
    let mut coords = board.top_left;
    let mut direction = Direction::Right;

    for action in path.iter() {
        match action {
            Action::Move(steps) => {
                (coords, direction) = board.walk(coords, *steps, direction, cube_wrap);
            }
            Action::RotateLeft => direction = direction.rotate(false),
            Action::RotateRight => direction = direction.rotate(true),
        }
        // println!(
        //     "Move from ({row},{col}) {steps} steps \"{direction:?}\", then turn {}",
        //     if clockwise { "RIGHT" } else { "LEFT" }
        // );
        // println!("\tended at ({row},{col}) ({direction:?})");
    }

    let (row, col) = coords;
    1000 * (row + 1) + 4 * (col + 1) + direction.facing_score()
}

fn solve_part1((board, path): &(Board, Path)) -> usize {
    traverse_map(board, path, false)
}

fn solve_part2((board, path): &(Board, Path)) -> usize {
    traverse_map(board, path, true)
}

generate_main!();

// generate_tests!(6032, 5031);

#[cfg(test)]
mod tests {
    use super::*;

    // see [`wrap_cube`] of [`super::Board`] for unfolded face plan...
    const FACE_CORNERS: [Coords; 24] = [
        // face 1 corners
        (0, SIZE),
        (0, 2 * SIZE - 1),
        (SIZE - 1, 2 * SIZE - 1),
        (SIZE - 1, SIZE),
        // face 2 corners
        (0, 2 * SIZE),
        (0, 3 * SIZE - 1),
        (SIZE - 1, 3 * SIZE - 1),
        (SIZE - 1, 2 * SIZE),
        // face 3 corners
        (SIZE, SIZE),
        (SIZE, 2 * SIZE - 1),
        (2 * SIZE - 1, 2 * SIZE - 1),
        (2 * SIZE - 1, SIZE),
        // face 4 corners
        (2 * SIZE, 0),
        (2 * SIZE, SIZE - 1),
        (3 * SIZE - 1, SIZE - 1),
        (3 * SIZE - 1, 0),
        // face 5 corners
        (2 * SIZE, SIZE),
        (2 * SIZE, 2 * SIZE - 1),
        (3 * SIZE - 1, 2 * SIZE - 1),
        (3 * SIZE - 1, SIZE),
        // face 6 corners
        (3 * SIZE, 0),
        (3 * SIZE, SIZE - 1),
        (4 * SIZE - 1, SIZE - 1),
        (4 * SIZE - 1, 0),
    ];

    #[test]
    fn test_part1() {
        let data = utils::string_from_sample(22);
        let input = parse_input(&data);
        assert_eq!(6032, solve_part1(&input));
    }

    #[test]
    fn test_p2_cube_wraps() {
        for start in FACE_CORNERS {
            for direction in [
                Direction::Left,
                Direction::Right,
                Direction::Up,
                Direction::Down,
            ] {
                let (finish, finish_direction) = Board::fly_around(start, direction);
                assert_eq!(start, finish);
                assert_eq!(direction, finish_direction);
            }
        }
    }

    const CORNERS_CHECKS: [[(Direction, bool); 2]; 4] = [
        [(Direction::Up, false), (Direction::Left, true)], // top left
        [(Direction::Right, false), (Direction::Up, true)], // top right
        [(Direction::Right, true), (Direction::Down, false)], // bottom right
        [(Direction::Down, true), (Direction::Left, false)], // bottom left
    ];

    #[test]
    fn test_p2_corner_wraps() {
        (0..FACE_CORNERS.len()).for_each(|i| {
            let corner_type = i % 4;
            for (direction, clockwise) in CORNERS_CHECKS[corner_type] {
                let start = FACE_CORNERS[i];
                let (finish, finish_direction) = Board::walk_corner(start, direction, clockwise);
                assert_eq!(start, finish);
                assert_eq!(direction, finish_direction);
            }
        });
    }
}
