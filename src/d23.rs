// #![warn(clippy::pedantic)]
use std::collections::{HashMap, HashSet};

mod utils;
const DAY_ID: utils::DayIdType = 23;

type Coord = i64;
type Coords = (Coord, Coord);

#[derive(Debug)]
struct Elves {
    elves: HashSet<Coords>,
    intent_order: Vec<([Neighbor; 3], Coords)>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Neighbor {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

impl Elves {
    pub fn from_coords(coords: &[Coords]) -> Self {
        Self {
            elves: coords.iter().copied().collect(),
            intent_order: vec![
                ([Neighbor::N, Neighbor::NE, Neighbor::NW], (-1, 0)),
                ([Neighbor::S, Neighbor::SE, Neighbor::SW], (1, 0)),
                ([Neighbor::W, Neighbor::NW, Neighbor::SW], (0, -1)),
                ([Neighbor::E, Neighbor::NE, Neighbor::SE], (0, 1)),
            ],
        }
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        let min_y = self.elves.iter().map(|elf| elf.0).min().unwrap();
        let max_y = self.elves.iter().map(|elf| elf.0).max().unwrap();
        let min_x = self.elves.iter().map(|elf| elf.1).min().unwrap();
        let max_x = self.elves.iter().map(|elf| elf.1).max().unwrap();
        // println!("({min_y}~{max_y},{min_x}~{max_x})");
        for row in min_y..=max_y {
            for col in min_x..=max_x {
                let ch = if self.elves.contains(&(row, col)) {
                    '#'
                } else {
                    '.'
                };
                print!("{ch}");
            }
            println!();
        }
    }

    pub fn count_empties(&self) -> usize {
        let min_y = self.elves.iter().map(|elf| elf.0).min().unwrap();
        let max_y = self.elves.iter().map(|elf| elf.0).max().unwrap();
        let min_x = self.elves.iter().map(|elf| elf.1).min().unwrap();
        let max_x = self.elves.iter().map(|elf| elf.1).max().unwrap();
        (max_y.abs_diff(min_y) + 1) as usize * (max_x.abs_diff(min_x) + 1) as usize
            - self.elves.len()
    }

    const NEIGHBOR_CANDIDATES: [(Coords, Neighbor); 8] = [
        ((-1, 0), Neighbor::N),
        ((-1, 1), Neighbor::NE),
        ((0, 1), Neighbor::E),
        ((1, 1), Neighbor::SE),
        ((1, 0), Neighbor::S),
        ((1, -1), Neighbor::SW),
        ((0, -1), Neighbor::W),
        ((-1, -1), Neighbor::NW),
    ];

    fn neighbors_of(&self, (row, col): Coords) -> Vec<Neighbor> {
        Self::NEIGHBOR_CANDIDATES
            .iter()
            .filter(|((rdx, cdx), _neighbor)| self.elves.contains(&(row + rdx, col + cdx)))
            .map(|(_coords, neighbor)| *neighbor)
            .collect()
    }

    pub fn scatter(&mut self) -> usize {
        let mut intents = HashMap::<Coords, usize>::new();
        let mut want_to_move = HashMap::<Coords, Coords>::new();

        for elf in &self.elves {
            let neighbors = self.neighbors_of(*elf);
            if neighbors.is_empty() {
                continue;
            }
            for (directions, (dy, dx)) in &self.intent_order {
                if directions.iter().all(|n| !neighbors.contains(n)) {
                    let target = (elf.0 + dy, elf.1 + dx);
                    // println!("{elf:?} could move to ({target:?})");
                    *intents.entry(target).or_insert(0) += 1;
                    want_to_move.insert(*elf, target);
                    break;
                }
            }
        }

        let mut elves_moved = 0;
        for (from, target) in want_to_move {
            if let Some(count) = intents.get(&target) {
                if count > &1 {
                    continue;
                } else {
                    self.elves.remove(&from);
                    self.elves.insert(target);
                    elves_moved += 1;
                }
            }
        }

        self.intent_order.rotate_left(1);

        elves_moved
    }
}

type Data = (usize, Vec<Coords>);

fn parse_input(data: &str) -> Data {
    let dimensions = data.lines().next().unwrap().len();

    let mut elves = vec![];
    for (row, line) in data.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch == '#' {
                elves.push((row as Coord, col as Coord));
            }
        }
    }
    (dimensions, elves)
}

fn solve_part1((_dimensions, elves): &Data) -> usize {
    let mut elves = Elves::from_coords(elves);

    for _ in 0..10 {
        elves.scatter();
    }

    elves.count_empties()
}

fn solve_part2((_dimensions, elves): &Data) -> usize {
    let mut elves = Elves::from_coords(elves);

    let mut rounds = 1;
    while elves.scatter() > 0 {
        rounds += 1;
    }

    rounds
}

generate_main!();

generate_tests!(110, 20);
