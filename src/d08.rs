#![warn(clippy::pedantic)]
mod utils;
const DAY_ID: utils::DayIdType = 8;

type Res = usize;

// unsafe (might panic) flattened 2D square array
struct SquareVec<T: Copy> {
    pub size: usize,
    pub vec: Vec<T>,
}

impl<T: Copy> SquareVec<T> {
    pub fn new(size: usize, init_value: T) -> Self {
        Self {
            size,
            vec: vec![init_value; size * size],
        }
    }

    fn index_of(&self, row: usize, col: usize) -> usize {
        row * self.size + col
    }

    pub fn get(&self, row: usize, col: usize) -> T {
        self.vec[self.index_of(row, col)]
    }

    pub fn set(&mut self, row: usize, col: usize, value: T) {
        let index = self.index_of(row, col);
        self.vec[index] = value;
    }
}

type Trees = SquareVec<usize>;

fn parse_input(data: &str) -> Trees {
    let size = data.lines().next().unwrap().len();

    let mut trees = SquareVec::new(size, 0);

    for (row, line) in data.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            let num = (ch as usize) - ('0' as usize);
            trees.set(row, col, num);
        }
    }

    trees
}

fn solve_part1(trees: &Trees) -> Res {
    let size = trees.size;

    let mut visible: SquareVec<bool> = SquareVec::new(size, false);

    for row in 1..(size - 1) {
        for col in 1..(size - 1) {
            // left to right
            for i in 0..col {
                if trees.get(row, i) >= trees.get(row, col) {
                    break;
                }
                if i == col - 1 {
                    visible.set(row, col, true);
                }
            }

            // right to left
            for i in (col + 1)..size {
                if trees.get(row, i) >= trees.get(row, col) {
                    break;
                }
                if i == size - 1 {
                    visible.set(row, col, true);
                }
            }

            // top to down
            for i in 0..row {
                if trees.get(i, col) >= trees.get(row, col) {
                    break;
                }
                if i == row - 1 {
                    visible.set(row, col, true);
                }
            }

            // down to top
            for i in (row + 1)..size {
                if trees.get(i, col) >= trees.get(row, col) {
                    break;
                }
                if i == size - 1 {
                    visible.set(row, col, true);
                }
            }
        }
    }

    for i in 0..size {
        visible.set(0, i, true);
        visible.set(i, 0, true);
        visible.set(size - 1, i, true);
        visible.set(i, size - 1, true);
    }

    visible.vec.iter().filter(|x| **x).count()
}

fn solve_part2(trees: &Trees) -> Res {
    let size = trees.size;

    let mut scenic_score = 0;

    for row in 1..(size - 1) {
        for col in 1..(size - 1) {
            let mut left = 0;
            for i in (0..col).rev() {
                if trees.get(row, i) >= trees.get(row, col) {
                    if i != 0 {
                        left += 1;
                    }
                    break;
                }
                left += 1;
            }

            let mut right = 0;
            for i in (col + 1)..size {
                if trees.get(row, i) >= trees.get(row, col) {
                    if i != size - 2 {
                        right += 1;
                    }
                    break;
                }
                right += 1;
            }

            let mut top = 0;
            for i in (0..row).rev() {
                if trees.get(i, col) >= trees.get(row, col) {
                    if i != 0 {
                        top += 1;
                    }
                    break;
                }
                top += 1;
            }

            let mut down = 0;
            for i in (row + 1)..size {
                if trees.get(i, col) >= trees.get(row, col) {
                    if i != size - 2 {
                        down += 1;
                    }
                    break;
                }
                down += 1;
            }

            // println!("[{row};{col}]: {left}, {right}, {top}, {down}");
            let product = left * right * top * down;
            if product > scenic_score {
                scenic_score = product;
            }
        }
    }

    scenic_score
}

generate_main!();

generate_tests!(21, 8);
