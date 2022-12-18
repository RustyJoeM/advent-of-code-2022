#![warn(clippy::pedantic)]
use std::collections::HashSet;

mod utils;
const DAY_ID: utils::DayIdType = 17;

fn parse_input(data: &str) -> &str {
    data
}

type Coord = i128;
type Coords = (Coord, Coord);

#[derive(Debug, Copy, Clone)]
enum Block {
    Minus,
    Plus,
    L,
    I,
    Square,
}

impl Block {
    pub fn from_step(step: usize) -> Self {
        match step % 5 {
            0 => Self::Minus,
            1 => Self::Plus,
            2 => Self::L,
            3 => Self::I,
            4 => Self::Square,
            _ => unreachable!(),
        }
    }

    pub fn width(self) -> Coord {
        match self {
            Block::Minus => 4,
            Block::Plus | Block::L => 3,
            Block::I => 1,
            Block::Square => 2,
        }
    }

    pub fn height(self) -> Coord {
        match self {
            Block::Minus => 1,
            Block::Plus | Block::L => 3,
            Block::I => 4,
            Block::Square => 2,
        }
    }

    pub fn taken_coords(self, (x, y): Coords) -> Vec<Coords> {
        match self {
            Block::Minus => vec![(x, y), (x + 1, y), (x + 2, y), (x + 3, y)],
            Block::Plus => vec![
                (x + 1, y),
                (x, y + 1),
                (x + 1, y + 1),
                (x + 2, y + 1),
                (x + 1, y + 2),
            ],
            Block::L => vec![
                (x, y),
                (x + 1, y),
                (x + 2, y),
                (x + 2, y + 1),
                (x + 2, y + 2),
            ],
            Block::I => vec![(x, y), (x, y + 1), (x, y + 2), (x, y + 3)],
            Block::Square => vec![(x, y), (x + 1, y), (x, y + 1), (x + 1, y + 1)],
        }
    }

    pub fn coords_on_left_of(self, (x, y): Coords) -> Vec<Coords> {
        match self {
            Block::Minus => vec![(x - 1, y)],
            Block::Plus => vec![(x, y), (x - 1, y + 1), (x, y + 2)],
            Block::L => vec![(x - 1, y), (x + 1, y + 1), (x + 1, y + 2)],
            Block::I => {
                vec![(x - 1, y), (x - 1, y + 1), (x - 1, y + 2), (x - 1, y + 3)]
            }
            Block::Square => vec![(x - 1, y), (x - 1, y + 1)],
        }
    }

    pub fn coords_on_right_of(self, (x, y): Coords) -> Vec<Coords> {
        match self {
            Block::Minus => vec![(x + 4, y)],
            Block::Plus => vec![(x + 2, y), (x + 3, y + 1), (x + 2, y + 2)],
            Block::L => vec![(x + 3, y), (x + 3, y + 1), (x + 3, y + 2)],
            Block::I => {
                vec![(x + 1, y), (x + 1, y + 1), (x + 1, y + 2), (x + 1, y + 3)]
            }
            Block::Square => vec![(x + 2, y), (x + 2, y + 1)],
        }
    }

    pub fn coords_below_of(self, (x, y): Coords) -> Vec<Coords> {
        match self {
            Block::Minus => vec![(x, y - 1), (x + 1, y - 1), (x + 2, y - 1), (x + 3, y - 1)],
            Block::Plus => vec![(x, y), (x + 1, y - 1), (x + 2, y)],
            Block::L => vec![(x, y - 1), (x + 1, y - 1), (x + 2, y - 1)],
            Block::I => vec![(x, y - 1)],
            Block::Square => vec![(x, y - 1), (x + 1, y - 1)],
        }
    }
}

fn well_includes(well: &HashSet<Coords>, coords: &[Coords]) -> bool {
    coords.iter().any(|coords| well.contains(coords))
}

const WELL_WIDTH: Coord = 7;
const BLOCK_INIT_X: Coord = 2;
const BLOCK_INIT_DY: Coord = 3;

fn solve_part1(data: &str) -> Coord {
    const BLOCK_COUNT: usize = 2022;

    let mut blocks_fallen = 0;
    let mut highest_point: Coord = 0;

    let mut block = Block::from_step(blocks_fallen);

    let mut block_position = (BLOCK_INIT_X, highest_point + BLOCK_INIT_DY);

    let mut well = HashSet::<Coords>::new();
    for x in 0..WELL_WIDTH {
        well.insert((x, -1));
    }

    let mut char_index = 0usize;
    let char_count = data.chars().count();
    loop {
        let ch = data.chars().nth(char_index).unwrap();
        char_index += 1;
        char_index %= char_count;

        match ch {
            '>' => {
                if block_position.0 < WELL_WIDTH - block.width()
                    && !well_includes(&well, &block.coords_on_right_of(block_position))
                {
                    block_position.0 += 1;
                }
            }
            '<' => {
                if block_position.0 != 0
                    && !well_includes(&well, &block.coords_on_left_of(block_position))
                {
                    block_position.0 -= 1;
                }
            }
            _ => unreachable!(),
        }

        if well_includes(&well, &block.coords_below_of(block_position)) {
            for coords in block.taken_coords(block_position) {
                well.insert(coords);
            }
            highest_point = highest_point.max(block_position.1 + block.height());

            blocks_fallen += 1;
            block = Block::from_step(blocks_fallen);
            block_position = (BLOCK_INIT_X, highest_point + BLOCK_INIT_DY);

            if blocks_fallen == BLOCK_COUNT {
                break;
            }
        } else {
            block_position.1 -= 1;
        }
    }

    highest_point
}

fn solve_part2(data: &str) -> Coord {
    const BLOCK_COUNT: usize = 1_000_000_000_000;

    let mut blocks_fallen = 0;
    let mut highest_point: Coord = 0;

    let mut block = Block::from_step(blocks_fallen);

    let mut block_position = (BLOCK_INIT_X, highest_point + BLOCK_INIT_DY);

    let mut well = HashSet::<Coords>::new();
    for x in 0..WELL_WIDTH {
        well.insert((x, -1));
    }

    let mut cycle_blocks_heights: Vec<(usize, Coord)> = vec![];

    let mut latest_blocks = 0;
    let mut latest_height = 0;

    let mut skipped_loops = 0;

    let mut char_index = 0usize;
    let char_count = data.chars().count();
    loop {
        let ch = data.chars().nth(char_index).unwrap();
        char_index += 1;
        char_index %= char_count;

        if char_index == 0 {
            let block_diff = blocks_fallen - latest_blocks;
            let height_diff = highest_point - latest_height;
            cycle_blocks_heights.push((block_diff, height_diff));

            // println!(
            //     "After {blocks_fallen} blocks (current height: {highest_point}),\n\tcycles data: {cycle_blocks_heights:?}"
            // );

            let len = cycle_blocks_heights.len();
            if len > 1 {
                let prev = cycle_blocks_heights[len - 2];
                let current = cycle_blocks_heights[len - 1];
                if prev == current {
                    skipped_loops = (BLOCK_COUNT - blocks_fallen) / current.0;
                    // println!("stable loop -> size {current:?}, on {blocks_fallen}, skipped loops: {skipped_loops}", );
                    blocks_fallen += skipped_loops * current.0;
                    // println!("jumping to {blocks_fallen} blocks");
                }
            }

            latest_blocks = blocks_fallen;
            latest_height = highest_point;
        }

        match ch {
            '>' => {
                if block_position.0 < WELL_WIDTH - block.width()
                    && !well_includes(&well, &block.coords_on_right_of(block_position))
                {
                    block_position.0 += 1;
                }
            }
            '<' => {
                if block_position.0 != 0
                    && !well_includes(&well, &block.coords_on_left_of(block_position))
                {
                    block_position.0 -= 1;
                }
            }
            _ => unreachable!(),
        }

        if well_includes(&well, &block.coords_below_of(block_position)) {
            for coords in block.taken_coords(block_position) {
                well.insert(coords);
            }
            highest_point = highest_point.max(block_position.1 + block.height());
            blocks_fallen += 1;
            block = Block::from_step(blocks_fallen);
            block_position = (BLOCK_INIT_X, highest_point + BLOCK_INIT_DY);

            if blocks_fallen == BLOCK_COUNT {
                break;
            }
        } else {
            block_position.1 -= 1;
        }
    }

    let catch_up = if skipped_loops > 0 {
        // println!("\tFull loops skipped: {skipped_loops} with height: {highest_point}\n\tcycles data: {cycle_blocks_heights:?}");
        (skipped_loops as Coord) * cycle_blocks_heights[cycle_blocks_heights.len() - 1].1
    } else {
        0
    };

    highest_point + catch_up
}

generate_main!();

generate_tests!(3068, 1_514_285_714_288);
