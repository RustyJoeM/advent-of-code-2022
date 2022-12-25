#![warn(clippy::pedantic)]
use std::collections::{HashMap, HashSet, VecDeque};

mod utils;
const DAY_ID: utils::DayIdType = 16;

#[derive(Debug, Clone)]
struct Valve {
    name: String,
    flow_rate: usize,
    tunnels: Vec<String>,
}

fn parse_input(data: &str) -> Vec<Valve> {
    data.lines()
        .map(|line| {
            // Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
            let words = line.split_ascii_whitespace().collect::<Vec<&str>>();
            let name = words[1].to_string();
            let flow_rate = words[4]
                .strip_prefix("rate=")
                .unwrap()
                .strip_suffix(';')
                .unwrap()
                .parse::<usize>()
                .unwrap();
            let tunnels = words
                .iter()
                .skip(9)
                .map(|w| {
                    match w.strip_suffix(',') {
                        Some(w) => w,
                        None => w,
                    }
                    .to_string()
                })
                .collect();
            Valve {
                name,
                flow_rate,
                tunnels,
            }
        })
        .collect()
}

fn find_shortest_distance(volcano_map: &HashMap<&str, &Valve>, start: &str, finish: &str) -> usize {
    let mut queue = VecDeque::from(vec![(start, start, 0usize)]);
    let mut visited = HashMap::<&str, &str>::from([(start, start)]);

    let mut distance = 999_999_999;
    while !queue.is_empty() {
        let (node, from, dist) = queue.pop_front().unwrap();

        if node == finish {
            distance = dist;
            break;
        }

        let neighbors = &volcano_map.get(node).unwrap().tunnels;

        for neighbor in neighbors.iter() {
            if neighbor == from {
                continue;
            }
            visited.insert(neighbor, node);
            queue.push_back((neighbor, node, dist + 1));
        }
    }

    distance
}

struct PipeSystem<'data> {
    valves: &'data [Valve],
    distances: HashMap<(&'data str, &'data str), usize>,
}

impl<'data> PipeSystem<'data> {
    pub fn new(valves: &'data [Valve]) -> Self {
        let valve_names = valves.iter().map(|v| v.name.as_str()).collect::<Vec<_>>();
        let volcano_map = valves.iter().map(|v| (v.name.as_str(), v)).collect();

        let mut distances = HashMap::<(&str, &str), usize>::new();
        for start in &valve_names {
            for finish in &valve_names {
                let distance = if start == finish {
                    0
                } else {
                    find_shortest_distance(&volcano_map, start, finish)
                };
                distances.insert((start, finish), distance);
            }
        }

        Self { valves, distances }
    }

    pub fn productive_valves(&self) -> Vec<(&str, usize)> {
        self.valves
            .iter()
            .filter(|v| v.flow_rate > 0)
            .map(|v| (v.name.as_str(), v.flow_rate))
            .collect()
    }

    pub fn min_distance(&self, from: &str, to: &str) -> usize {
        *self.distances.get(&(from, to)).unwrap()
    }

    fn highest_pressure(&self, t: usize, from: &str, seen: &HashSet<&str>) -> usize {
        const MINUTES: usize = 30;

        let mut best = 0;

        let productive_valves = self.productive_valves();

        for (to, to_flow) in productive_valves {
            if !seen.contains(&to) {
                let new_t = t + self.min_distance(from, to) + 1;
                if new_t <= MINUTES {
                    let mut new_seen = seen.clone();
                    new_seen.insert(to);
                    let a =
                        self.highest_pressure(new_t, to, &new_seen) + (MINUTES - new_t) * to_flow;
                    if a > best {
                        best = a;
                    }
                }
            }
        }

        best
    }

    pub fn get_max_pressure(&self) -> usize {
        let seen = HashSet::new();
        self.highest_pressure(0, "AA", &seen)
    }

    // TODO - potential bug when best solution does not come from me/helper/me/helper swaps...
    fn highest_pressure_with_helper(
        &self,
        helpers_turn: bool,
        me: (usize, &str),
        helper: (usize, &str),
        visited: &HashSet<&str>,
    ) -> usize {
        const MINUTES: usize = 26;

        let mut best = 0;

        let productive_valves = self.productive_valves();

        for (to, to_flow) in productive_valves {
            if !visited.contains(&to) {
                let (t, from) = if helpers_turn { helper } else { me };
                let new_t = t + self.min_distance(from, to) + 1;
                if new_t <= MINUTES {
                    let mut new_visited = visited.clone();
                    new_visited.insert(to);
                    let a = self.highest_pressure_with_helper(
                        !helpers_turn,
                        if helpers_turn { me } else { (new_t, to) },
                        if helpers_turn { (new_t, to) } else { helper },
                        &new_visited,
                    ) + (MINUTES - new_t) * to_flow;
                    if a > best {
                        best = a;
                    }
                }
            }
        }

        best
    }

    pub fn get_max_pressure_with_helper(&self) -> usize {
        let seen = HashSet::new();
        self.highest_pressure_with_helper(false, (0, "AA"), (0, "AA"), &seen)
    }
}

fn solve_part1(valves: &[Valve]) -> usize {
    let system = PipeSystem::new(valves);
    system.get_max_pressure()
}

fn solve_part2(valves: &[Valve]) -> usize {
    let system = PipeSystem::new(valves);
    system.get_max_pressure_with_helper()
}

generate_main!();

generate_tests!(1651, 1707);
