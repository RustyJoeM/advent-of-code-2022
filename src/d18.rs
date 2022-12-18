#![warn(clippy::pedantic)]
use std::collections::{HashSet, VecDeque};

mod utils;
const DAY_ID: utils::DayIdType = 18;

type Coord = i64;
type Cube = (Coord, Coord, Coord);

fn parse_input(data: &str) -> Vec<Cube> {
    data.lines()
        .map(|line| {
            let coords = line.split(',').collect::<Vec<_>>();
            let x = coords[0].parse().unwrap();
            let y = coords[1].parse().unwrap();
            let z = coords[2].parse().unwrap();
            (x, y, z)
        })
        .collect()
}

fn solve_part1(data: &[Cube]) -> usize {
    let pieces = data.iter().copied().collect::<HashSet<Cube>>();

    let mut exposed = 0;

    (0..data.len()).for_each(|i| {
        let (x, y, z) = data[i];
        for coords in [
            (x + 1, y, z),
            (x - 1, y, z),
            (x, y + 1, z),
            (x, y - 1, z),
            (x, y, z + 1),
            (x, y, z - 1),
        ] {
            if !pieces.contains(&coords) {
                exposed += 1;
            }
        }
    });

    exposed
}

fn solve_part2(data: &[Cube]) -> usize {
    let pieces = data.iter().copied().collect::<HashSet<Cube>>();

    let mut x_bounds = (999, 0);
    let mut y_bounds = (999, 0);
    let mut z_bounds = (999, 0);
    for &(x, y, z) in data {
        x_bounds.0 = x_bounds.0.min(x);
        x_bounds.1 = x_bounds.1.max(x);
        y_bounds.0 = y_bounds.0.min(y);
        y_bounds.1 = y_bounds.1.max(y);
        z_bounds.0 = z_bounds.0.min(z);
        z_bounds.1 = z_bounds.1.max(z);
    }

    let mut flooded = HashSet::<Cube>::new();
    let mut flood_queue = VecDeque::<Cube>::from(vec![(x_bounds.0, y_bounds.0, z_bounds.0)]);
    while !flood_queue.is_empty() {
        let (x, y, z) = flood_queue.pop_back().unwrap();

        for coords in [
            (x + 1, y, z),
            (x - 1, y, z),
            (x, y + 1, z),
            (x, y - 1, z),
            (x, y, z + 1),
            (x, y, z - 1),
        ] {
            if coords.0 < x_bounds.0 - 1 || coords.0 > x_bounds.1 + 1 {
                continue;
            }
            if coords.1 < y_bounds.0 - 1 || coords.1 > y_bounds.1 + 1 {
                continue;
            }
            if coords.2 < z_bounds.0 - 1 || coords.2 > z_bounds.1 + 1 {
                continue;
            }
            if flooded.contains(&coords)
                || pieces.contains(&coords)
                || flood_queue.contains(&coords)
            {
                continue;
            }
            flooded.insert(coords);
            flood_queue.push_back(coords);
        }
    }

    let mut exposed = 0;

    (0..data.len()).for_each(|i| {
        let (x, y, z) = data[i];
        for coords in [
            (x + 1, y, z),
            (x - 1, y, z),
            (x, y + 1, z),
            (x, y - 1, z),
            (x, y, z + 1),
            (x, y, z - 1),
        ] {
            if flooded.contains(&coords) {
                exposed += 1;
            }
        }
    });

    exposed
}

generate_main!();

generate_tests!(64, 58);
