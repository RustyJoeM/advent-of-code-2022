#![warn(clippy::pedantic)]
use std::collections::HashSet;

mod utils;
const DAY_ID: utils::DayIdType = 6;

type Res = usize;

fn parse_input(data: &str) -> Vec<char> {
    data.chars().collect()
}

fn index_of_unique_chain(data: &[char], unique_length: usize) -> usize {
    let mut uniq = data
        .iter()
        .take(unique_length)
        .copied()
        .collect::<HashSet<_>>();

    let mut i = unique_length;
    for w in data.windows(unique_length + 1) {
        if uniq.len() == unique_length {
            break;
        }
        if !w[1..].contains(&w[0]) {
            uniq.remove(&w[0]);
        }
        uniq.insert(*w.last().unwrap());
        i += 1;
    }
    i
}

fn solve_part1(data: &[char]) -> Res {
    index_of_unique_chain(data, 4)
}

fn solve_part2(data: &[char]) -> Res {
    index_of_unique_chain(data, 14)
}

generate_main!();

generate_tests!(7, 19);
