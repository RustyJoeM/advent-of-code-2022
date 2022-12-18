#![warn(clippy::pedantic)]
use std::collections::HashMap;

mod utils;
const DAY_ID: utils::DayIdType = 7;

type Res = usize;

type Data = HashMap<String, Res>;

fn parse_input(data: &str) -> Data {
    let mut map = HashMap::new();

    let mut pwd = vec![String::new()];
    for line in data.lines() {
        if line.starts_with("$ ls") || line.starts_with("dir ") {
            continue;
        }
        if line.starts_with("$ cd") {
            let dir_name = line.split_ascii_whitespace().last().unwrap();
            if dir_name == ".." {
                pwd.pop();
            } else if dir_name == "/" {
                pwd = vec![String::new()];
            } else {
                pwd.push(dir_name.to_string());
            }
            continue;
        }

        let (size_str, _) = line.split_once(' ').unwrap();
        let file_size = size_str.parse::<usize>().unwrap();

        for i in 0..pwd.len() {
            let mut key = pwd[0..=i].join("/");
            if key.is_empty() {
                key = "/".to_string();
            }
            let entry: &mut usize = map.entry(key).or_default();
            *entry += file_size;
        }
    }

    map
}

fn solve_part1(data: &Data) -> Res {
    data.values().filter(|size| **size < 100_000).sum()
}

fn solve_part2(data: &Data) -> Res {
    const DISK_SPACE: usize = 70_000_000;
    const UNUSED_SPACE: usize = 30_000_000;
    const MAX_TAKEN: usize = DISK_SPACE - UNUSED_SPACE;

    let root_dir_size = data.values().max().unwrap();
    let need_to_delete = root_dir_size - MAX_TAKEN;

    data.values()
        .filter(|v| **v >= need_to_delete)
        .min()
        .copied()
        .unwrap()
}

generate_main!();

generate_tests!(95437, 24_933_642);
