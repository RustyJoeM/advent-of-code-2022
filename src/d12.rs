// #![warn(clippy::pedantic)]
mod utils;
const DAY_ID: utils::DayIdType = 12;

use std::collections::{HashMap, VecDeque};

type Index = i64;

#[derive(Debug)]
struct Maze {
    map: String,
    max_index: Index,
    cols: Index,
}

impl Maze {
    pub fn reachable_neighbors(&self, index: Index) -> Vec<Index> {
        const INDICES: [(Index, Index); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

        let mut neighbors = vec![];
        let from = self.map.chars().nth(index as usize).unwrap();

        for (dx, dy) in INDICES {
            let neighbor_index = index + self.cols * dx.signum() + dy.signum();
            if neighbor_index < 0 || neighbor_index >= self.max_index {
                continue;
            }
            let ch = self.map.chars().nth(neighbor_index as usize).unwrap();
            // don't go back to start
            if ch == 'S' {
                continue;
            }
            // hacky fixes for start/stop form/to position
            let from = if from == 'S' { 'a' } else { from };
            let ch = if ch == 'E' { 'z' } else { ch };

            if ch as i8 - from as i8 <= 1 {
                neighbors.push(neighbor_index);
            }
        }

        neighbors
    }

    pub fn start(&self) -> Index {
        self.map.find('S').unwrap() as Index
    }

    pub fn finish(&self) -> Index {
        self.map.find('E').unwrap() as Index
    }
}

fn parse_input(data: &str) -> Maze {
    let map = data.lines().collect::<String>();
    Maze {
        max_index: map.len() as Index,
        map,
        cols: data.lines().next().unwrap().len() as Index,
    }
}

fn find_bfs(maze: &Maze, start: Index) -> usize {
    let mut queue = VecDeque::<Index>::from(vec![start]);
    let mut visited = HashMap::<Index, Index>::from([(start, start)]);

    let finish = maze.finish();

    while !queue.is_empty() {
        let node = queue.pop_front().unwrap();

        if node == finish {
            break;
        }

        let unvisited_neighbors = maze
            .reachable_neighbors(node)
            .iter()
            .filter(|index| !visited.contains_key(index))
            .copied()
            .collect::<Vec<Index>>();

        for neighbor in unvisited_neighbors {
            visited.insert(neighbor, node);
            queue.push_back(neighbor);
        }
    }

    let mut path = VecDeque::new();

    let mut parent_key = finish;
    while let Some((coords, next_parent)) = visited.get_key_value(&parent_key) {
        if coords == next_parent {
            break;
        }
        path.push_back(*coords);
        parent_key = *next_parent;
    }

    path.len()
}

fn solve_part1(maze: &Maze) -> usize {
    find_bfs(maze, maze.start())
}

fn solve_part2(maze: &Maze) -> usize {
    let mut all_a = vec![];
    for (index, ch) in maze.map.chars().enumerate() {
        if ch == 'a' {
            all_a.push(index as Index);
        }
    }

    all_a
        .iter()
        .map(|start| find_bfs(maze, *start))
        .filter(|len| *len != 0)
        .min()
        .unwrap()
}

generate_main!();

generate_tests!(31, 29);
