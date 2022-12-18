#![warn(clippy::pedantic)]
use std::collections::HashSet;

mod utils;
const DAY_ID: utils::DayIdType = 15;

type Coord = i64;
type Coords = (Coord, Coord);

fn parse_input(data: &str) -> Vec<(Coords, Coords)> {
    // Sensor at x=3289936, y=2240812: closest beacon is at x=3232809, y=2000000
    data.lines()
        .map(|line| {
            let words = line.split_ascii_whitespace().collect::<Vec<&str>>();

            let sx = words[2][2..(words[2].len() - 1)].parse::<Coord>().unwrap();
            let sy = words[3][2..(words[3].len() - 1)].parse::<Coord>().unwrap();

            let bx = words[8][2..(words[8].len() - 1)].parse::<Coord>().unwrap();
            let by = words[9][2..(words[9].len())].parse::<Coord>().unwrap();

            ((sx, sy), (bx, by))
        })
        .collect()
}

fn coord_dist(x: Coord, y: Coord) -> Coord {
    if x > y {
        x - y
    } else {
        y - x
    }
}

fn solve_part1(data: &[(Coords, Coords)]) -> usize {
    #[cfg(test)]
    const ROW_OF_INTEREST: Coord = 10;
    #[cfg(not(test))]
    const ROW_OF_INTEREST: Coord = 2_000_000;

    let mut intervals: Vec<(Coord, Coord)> = vec![];

    for &((sx, sy), (bx, by)) in data.iter() {
        let bs_distance = coord_dist(bx, sx) + coord_dist(by, sy);

        for y in (sy - bs_distance)..=(sy + bs_distance) {
            if y == ROW_OF_INTEREST {
                let d = coord_dist(y, sy);
                let interval_len = bs_distance - d;
                let interval = (sx - interval_len, sx + interval_len);
                intervals.push(interval);
            }
        }
    }

    let mut cells = HashSet::<Coord>::new();
    for &(from, to) in &intervals {
        for y in from..=to {
            cells.insert(y);
        }
    }

    cells.len() - 1
}

fn solve_part2(data: &[(Coords, Coords)]) -> Coord {
    #[cfg(test)]
    const LIMIT: Coord = 20;
    #[cfg(not(test))]
    const LIMIT: Coord = 4_000_000;

    let mut aux: Vec<(Coord, Coord, Coord)> = vec![];
    for &((sx, sy), (bx, by)) in data.iter() {
        let distance = coord_dist(bx, sx) + coord_dist(by, sy);
        aux.push((sx, sy, distance));
    }

    for yy in 0..=LIMIT {
        let mut xx = 0;
        'next: while xx <= LIMIT {
            for &(sx, sy, bs_distance) in &aux {
                let my_distance = coord_dist(xx, sx) + coord_dist(yy, sy);
                if my_distance <= bs_distance {
                    let d = coord_dist(yy, sy);
                    let interval_len = bs_distance - d;
                    xx = sx + interval_len + 1;
                    continue 'next;
                }
            }
            return xx * 4_000_000 + yy;
        }
    }

    unreachable!();
}

generate_main!();

generate_tests!(26, 56_000_011);
