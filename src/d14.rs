#![warn(clippy::pedantic)]
use std::collections::HashSet;

mod utils;
const DAY_ID: utils::DayIdType = 14;

type Coord = i16;
type Coords = (Coord, Coord);
type Path = Vec<Coords>;

fn parse_input(data: &str) -> Vec<Path> {
    data.lines()
        .map(|line| {
            line.split(" -> ")
                .map(|s| {
                    let (x, y) = s.split_once(',').unwrap();
                    (x.parse::<Coord>().unwrap(), y.parse::<Coord>().unwrap())
                })
                .collect()
        })
        .collect()
}

fn init_blocks(paths: &[Path]) -> HashSet<Coords> {
    let mut blocks = HashSet::new();

    for path in paths.iter() {
        for win in path.windows(2) {
            let (from_x, from_y) = win[0];
            let (to_x, to_y) = win[1];
            if from_x != to_x {
                // horizontal line (y coord constant)
                let from = from_x.min(to_x);
                let to = from_x.max(to_x);
                for x in from..=to {
                    blocks.insert((x, from_y));
                }
            } else if from_y != to_y {
                // vertical line (x coord constant)
                let from = from_y.min(to_y);
                let to = from_y.max(to_y);
                for y in from..=to {
                    blocks.insert((from_x, y));
                }
            } else {
                // single block "line"
                blocks.insert((from_x, from_y));
            }
        }
    }

    blocks
}

fn solve_part1(paths: &[Path]) -> usize {
    let mut abyss_y = 0;
    for (_x, y) in paths.iter().flatten() {
        if *y > abyss_y {
            abyss_y = *y;
        }
    }

    let mut blocks = init_blocks(paths);
    let stone_blocks = blocks.len();

    let mut falling_sand = (500, 0);
    loop {
        let (x, y) = falling_sand;

        if y > abyss_y {
            break;
        }

        if !blocks.contains(&(x, y + 1)) {
            falling_sand.1 += 1;
            continue;
        }
        if !blocks.contains(&(x - 1, y + 1)) {
            falling_sand.0 -= 1;
            falling_sand.1 += 1;
            continue;
        }
        if !blocks.contains(&(x + 1, y + 1)) {
            falling_sand.0 += 1;
            falling_sand.1 += 1;
            continue;
        }

        blocks.insert(falling_sand);

        falling_sand = (500, 0);
    }

    blocks.len() - stone_blocks
}

fn solve_part2(paths: &[Path]) -> usize {
    let mut abyss_y = 0;
    for (_x, y) in paths.iter().flatten() {
        if *y > abyss_y {
            abyss_y = *y;
        }
    }

    let floor = abyss_y + 2;

    let mut blocks = init_blocks(paths);

    // let init_blocks = blocks.clone();

    let stone_blocks = blocks.len();

    let mut sand = (500, 0);
    loop {
        let (x, y) = sand;

        if y == floor - 1 {
            if x == 500 && y == 0 {
                break;
            }
            blocks.insert(sand);
            sand = (500, 0);
            continue;
        }

        if !blocks.contains(&(x, y + 1)) {
            sand.1 += 1;
            continue;
        }
        if !blocks.contains(&(x - 1, y + 1)) {
            sand.0 -= 1;
            sand.1 += 1;
            continue;
        }
        if !blocks.contains(&(x + 1, y + 1)) {
            sand.0 += 1;
            sand.1 += 1;
            continue;
        }

        blocks.insert(sand);

        if x == 500 && y == 0 {
            break;
        }

        sand = (500, 0);
    }

    // print_blocks(paths, &init_blocks, &blocks);

    blocks.len() - stone_blocks
}

#[allow(dead_code)]
fn print_blocks(paths: &[Path], init_blocks: &HashSet<Coords>, blocks: &HashSet<Coords>) {
    const BLOCK_STR: &str = "\u{2588}";

    let mut y1 = 0;
    for &(_, y) in paths.iter().flatten() {
        if y > y1 {
            y1 = y;
        }
    }

    for y in 0..=(y1 + 2) {
        for x in (500 - y1 - 2)..=(500 + y1 + 2) {
            let char = if y == y1 + 2 {
                BLOCK_STR
            } else if blocks.contains(&(x, y)) {
                if init_blocks.contains(&(x, y)) {
                    BLOCK_STR
                } else {
                    "o"
                }
            } else {
                "."
            };
            print!("{char}");
        }
        println!();
    }
}

generate_main!();

generate_tests!(24, 93);
