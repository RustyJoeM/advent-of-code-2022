#![warn(clippy::pedantic)]
use std::collections::HashSet;

mod utils;
const DAY_ID: utils::DayIdType = 9;

type Res = usize;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

type StepSize = usize;
type Step = (Direction, StepSize);

fn parse_input(data: &str) -> Vec<Step> {
    data.lines()
        .map(|x| {
            let (dir, size) = x.split_once(' ').unwrap();
            let direction = match dir {
                "U" => Direction::Up,
                "D" => Direction::Down,
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => unreachable!(),
            };
            let size = size.parse::<StepSize>().unwrap();
            (direction, size)
        })
        .collect()
}

type Coordinate = i32;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Position {
    pub x: Coordinate,
    pub y: Coordinate,
}

impl Position {
    pub fn distance(self, other: Position) -> StepSize {
        let dx = self.x.abs_diff(other.x) as StepSize;
        let dy = self.y.abs_diff(other.y) as StepSize;
        if dx == 1 && dy == 1 {
            return 1;
        }
        dx + dy
    }

    pub fn step_once(&mut self, direction: Direction) {
        match direction {
            Direction::Up => self.y -= 1,
            Direction::Down => self.y += 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        }
    }
}

fn solve_part1(data: &[Step]) -> Res {
    let mut head = Position { x: 0, y: 0 };
    let mut tail = Position { x: 0, y: 0 };

    let mut tail_positions: HashSet<Position> = HashSet::new();

    for (direction, size) in data.iter() {
        for _ in 0..*size {
            let last_head = head;
            head.step_once(*direction);

            if head.distance(tail) > 1 {
                tail = last_head;
            }
            tail_positions.insert(tail);
        }
    }

    tail_positions.len()
}

fn solve_part2(data: &[Step]) -> Res {
    let mut knots = vec![Position { x: 0, y: 0 }; 10];

    let mut tail_positions: HashSet<Position> = HashSet::new();

    for (direction, size) in data.iter() {
        for _ in 0..*size {
            knots[0].step_once(*direction);

            for i in 1..knots.len() {
                let head = knots[i - 1];
                let tail = &mut knots[i];

                if head.distance(*tail) > 1 {
                    tail.x += (head.x - tail.x).signum();
                    tail.y += (head.y - tail.y).signum();
                } else {
                    break;
                }
            }
            tail_positions.insert(knots[9]);
        }
    }

    tail_positions.len()
}

generate_main!();

generate_tests!(13, 1);
