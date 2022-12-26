#![warn(clippy::pedantic)]
use std::collections::HashSet;

mod utils;
const DAY_ID: utils::DayIdType = 24;

type Coord = usize;
type Coords = (Coord, Coord);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    pub fn to_char(self) -> char {
        match self {
            Direction::Left => '<',
            Direction::Right => '>',
            Direction::Up => '^',
            Direction::Down => 'v',
        }
    }

    pub fn of(self, (row, col): Coords) -> Coords {
        match self {
            Direction::Left => (row, col - 1),
            Direction::Right => (row, col + 1),
            Direction::Up => (row - 1, col),
            Direction::Down => (row + 1, col),
        }
    }
}

impl TryFrom<char> for Direction {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '<' => Ok(Direction::Left),
            '>' => Ok(Direction::Right),
            '^' => Ok(Direction::Up),
            'v' => Ok(Direction::Down),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
struct Maze {
    rows: usize,
    cols: usize,
    inner: Vec<Vec<HashSet<Direction>>>,
}

impl Maze {
    fn new_inner(rows: usize, cols: usize) -> Vec<Vec<HashSet<Direction>>> {
        vec![vec![HashSet::default(); cols]; rows]
    }

    pub fn from_str(input: &str) -> Self {
        let rows = input.lines().count();
        let cols = input.lines().next().unwrap().len();
        let mut inner = Self::new_inner(rows, cols);

        for (row, line) in input.lines().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                if let Ok(direction) = Direction::try_from(ch) {
                    inner[row][col].insert(direction);
                }
            }
        }

        Self { rows, cols, inner }
    }

    fn start() -> Coords {
        (0, 1)
    }

    pub fn finish(&self) -> Coords {
        (self.rows - 1, self.cols - 2)
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        for row in 0..self.rows {
            for col in 0..self.cols {
                let ch = if (row, col) == Self::start() || (row, col) == self.finish() {
                    ' '
                } else if row == 0 || row == self.rows - 1 || col == 0 || col == self.cols - 1 {
                    '#'
                } else {
                    match self.inner[row][col].len() {
                        0 => '.',
                        1 => self.inner[row][col].iter().next().unwrap().to_char(),
                        x => x.to_string().chars().next().unwrap(),
                    }
                };
                print!("{ch}");
            }
            println!();
        }
    }

    fn wrapped_coords(&self, (row, col): Coords) -> Coords {
        let row = if row == 0 {
            self.rows - 2
        } else if row == self.rows - 1 {
            1
        } else {
            row
        };
        let col = if col == 0 {
            self.cols - 2
        } else if col == self.cols - 1 {
            1
        } else {
            col
        };
        (row, col)
    }

    pub fn move_blizzards(&mut self) {
        let (rows, cols) = (self.rows, self.cols);
        let mut next = Self::new_inner(rows, cols);

        for row in 0..rows {
            for col in 0..cols {
                for blizzard in &self.inner[row][col] {
                    let (new_row, new_col) = self.wrapped_coords(match blizzard {
                        Direction::Left => (row, col - 1),
                        Direction::Right => (row, col + 1),
                        Direction::Up => (row - 1, col),
                        Direction::Down => (row + 1, col),
                    });
                    next[new_row][new_col].insert(*blizzard);
                }
            }
        }

        self.inner = next;
    }

    // assumes blizzards have moved on this turn already
    pub fn is_direction_free(&self, (row, col): Coords, direction: Direction) -> bool {
        // TODO - remove start/stop explicit checks?

        // extra checks for from-start, to-finish
        if (row, col) == Self::start() && direction != Direction::Down {
            return false;
        }
        if direction == Direction::Down && (row, col) == Direction::Up.of(self.finish()) {
            return true;
        }

        // extra checks for from-finish, to-start (p2)
        if (row, col) == self.finish() && direction != Direction::Up {
            return false;
        }
        if direction == Direction::Up && (row, col) == Direction::Down.of(Self::start()) {
            return true;
        }

        if match direction {
            Direction::Left => col <= 1,
            Direction::Right => col >= self.cols - 2,
            Direction::Up => row <= 1,
            Direction::Down => row >= self.rows - 2,
        } {
            return false;
        }
        let (new_row, new_col) = match direction {
            Direction::Left => (row, col - 1),
            Direction::Right => (row, col + 1),
            Direction::Up => (row - 1, col),
            Direction::Down => (row + 1, col),
        };
        self.inner[new_row][new_col].is_empty()
    }

    // assumes blizzards have moved on this turn already
    pub fn can_wait_at(&self, (row, col): Coords) -> bool {
        self.inner[row][col].is_empty()
    }

    // assumes blizzards have moved on this turn already
    pub fn available_moves_at(&self, coords: Coords) -> Vec<Coords> {
        let mut actions = vec![];

        if self.can_wait_at(coords) {
            actions.push(coords);
        }

        for direction in [
            Direction::Left,
            Direction::Up,
            Direction::Right,
            Direction::Down,
        ] {
            if self.is_direction_free(coords, direction) {
                actions.push(direction.of(coords));
            }
        }

        actions
    }
}

fn parse_input(data: &str) -> &str {
    data
}

fn flood_maze(maze: &mut Maze, from: Coords, to: Coords) -> usize {
    let mut currently_reached = HashSet::<Coords>::from([from]);

    let mut time = 0;
    loop {
        if currently_reached.contains(&to) {
            return time;
        }

        maze.move_blizzards();

        let mut next_reached = HashSet::<Coords>::new();
        for pos in currently_reached {
            for m in maze.available_moves_at(pos) {
                next_reached.insert(m);
            }
        }

        currently_reached = next_reached;
        time += 1;
    }
}

fn solve_part1(input: &str) -> usize {
    let mut maze = Maze::from_str(input);
    let from = Maze::start();
    let to = maze.finish();
    flood_maze(&mut maze, from, to)
}

fn solve_part2(input: &str) -> usize {
    let mut maze = Maze::from_str(input);
    let from = Maze::start();
    let to = maze.finish();
    let path1 = flood_maze(&mut maze, from, to);
    let path2 = flood_maze(&mut maze, to, from);
    let path3 = flood_maze(&mut maze, from, to);
    path1 + path2 + path3
}

generate_main!();

generate_tests!(18, 18 + 23 + 13);
