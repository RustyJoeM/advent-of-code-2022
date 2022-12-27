#![warn(clippy::pedantic)]
use std::collections::{HashSet, VecDeque};

mod utils;
const DAY_ID: utils::DayIdType = 19;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Material {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

impl Material {
    pub fn index(self) -> usize {
        match self {
            Material::Ore => 0,
            Material::Clay => 1,
            Material::Obsidian => 2,
            Material::Geode => 3,
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Blueprint {
    pub ore_robot_cost: usize,
    pub clay_robot_cost: usize,
    pub obsidian_robot_cost: (usize, usize),
    pub geode_robot_cost: (usize, usize),
}

impl From<&str> for Blueprint {
    // Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 2 ore and 11 clay. Each geode robot costs 4 ore and 8 obsidian.
    fn from(s: &str) -> Self {
        let words = s.split_ascii_whitespace().collect::<Vec<_>>();
        let ore_robot_cost = words[6].parse().unwrap();
        let clay_robot_cost = words[12].parse().unwrap();
        let obsidian_robot_cost = (words[18].parse().unwrap(), words[21].parse().unwrap());
        let geode_robot_cost = (words[27].parse().unwrap(), words[30].parse().unwrap());
        Self {
            ore_robot_cost,
            clay_robot_cost,
            obsidian_robot_cost,
            geode_robot_cost,
        }
    }
}

impl Blueprint {
    // number of robots needed to be able to produce 1 geode robot each turn
    pub fn robots_for_geode(&self) -> (usize, usize, usize) {
        let max_ores = [
            self.ore_robot_cost,
            self.clay_robot_cost,
            self.obsidian_robot_cost.0,
            self.geode_robot_cost.0,
        ]
        .iter()
        .max()
        .copied()
        .unwrap();
        let max_clays = self.obsidian_robot_cost.1;
        let max_obsidians = self.geode_robot_cost.1;
        (max_ores, max_clays, max_obsidians)
    }
}

fn parse_input(data: &str) -> Vec<Blueprint> {
    data.lines().map(Into::into).collect()
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Factory {
    robots: [usize; 4],
    stock: [usize; 4],
    minutes_to_go: usize,
}

impl Factory {
    pub fn new(minutes_to_go: usize) -> Self {
        Self {
            robots: [1, 0, 0, 0],
            stock: [0, 0, 0, 0],
            minutes_to_go,
        }
    }

    pub fn count_robots(&self, material: Material) -> usize {
        self.robots[material.index()]
    }

    fn update_stock(&mut self, material: Material, diff: usize, add: bool) {
        let item = &mut self.stock[material.index()];
        if add {
            *item += diff;
        } else {
            *item -= diff;
        };
    }

    pub fn mine(&mut self, material: Material) {
        let robots = self.count_robots(material);
        self.update_stock(material, robots, true);
    }

    pub fn mine_all(&mut self) {
        for material in [
            Material::Ore,
            Material::Clay,
            Material::Obsidian,
            Material::Geode,
        ] {
            self.mine(material);
        }
    }

    pub fn can_afford_robot(&self, robot_type: Material, blueprint: &Blueprint) -> bool {
        match robot_type {
            Material::Ore => self.count_stock(Material::Ore) >= blueprint.ore_robot_cost,
            Material::Clay => self.count_stock(Material::Ore) >= blueprint.clay_robot_cost,
            Material::Obsidian => {
                let (cost_ore, cost_clay) = blueprint.obsidian_robot_cost;
                self.count_stock(Material::Ore) >= cost_ore
                    && self.count_stock(Material::Clay) >= cost_clay
            }
            Material::Geode => {
                let (cost_ore, cost_obsidian) = blueprint.geode_robot_cost;
                self.count_stock(Material::Ore) >= cost_ore
                    && self.count_stock(Material::Obsidian) >= cost_obsidian
            }
        }
    }

    pub fn build_robot(&mut self, robot_type: Material, blueprint: &Blueprint) {
        match robot_type {
            Material::Ore => self.update_stock(Material::Ore, blueprint.ore_robot_cost, false),
            Material::Clay => self.update_stock(Material::Ore, blueprint.clay_robot_cost, false),
            Material::Obsidian => {
                let (cost_ore, cost_clay) = blueprint.obsidian_robot_cost;
                self.update_stock(Material::Ore, cost_ore, false);
                self.update_stock(Material::Clay, cost_clay, false);
            }
            Material::Geode => {
                let (cost_ore, cost_obsidian) = blueprint.geode_robot_cost;
                self.update_stock(Material::Ore, cost_ore, false);
                self.update_stock(Material::Obsidian, cost_obsidian, false);
            }
        }
        self.robots[robot_type.index()] += 1;
    }

    pub fn count_stock(&self, material: Material) -> usize {
        self.stock[material.index()]
    }

    pub fn max_no_cost_geodes(&self) -> usize {
        let mtg = self.minutes_to_go;
        self.count_stock(Material::Geode) // current stock
            + mtg * self.count_robots(Material::Geode)  // current geode robots
            + (mtg * (mtg - 1) / 2) // newly built geode robots
    }

    pub fn maxed_score(&self, blueprint: &Blueprint) -> Option<usize> {
        let (ores, clays, obsidians) = blueprint.robots_for_geode();
        if self.count_robots(Material::Ore) < ores
            || self.count_robots(Material::Clay) < clays
            || self.count_robots(Material::Obsidian) < obsidians
        {
            return None;
        }

        Some(self.max_no_cost_geodes())
    }

    pub fn step_and_build(&self, robot_type: Option<Material>, blueprint: &Blueprint) -> Self {
        let mut factory = self.clone();
        factory.mine_all();
        if let Some(robot_type) = robot_type {
            factory.build_robot(robot_type, blueprint);
        }
        factory.minutes_to_go -= 1;
        factory
    }
}

fn best_geode_count(blueprint: &Blueprint, minutes: usize) -> usize {
    let mut best_geodes = 0;

    let mut state_queue = VecDeque::from([Factory::new(minutes)]);

    let (max_ore_robots, max_clay_robots, max_obsidian_robots) = blueprint.robots_for_geode();

    let mut states_seen = HashSet::<([usize; 4], [usize; 4], usize)>::new();

    while let Some(mut factory) = state_queue.pop_front() {
        if factory.minutes_to_go == 0 {
            let geodes = factory.count_stock(Material::Geode);
            if geodes > best_geodes {
                best_geodes = geodes;
            }
            continue;
        }

        if !states_seen.insert((factory.robots, factory.stock, factory.minutes_to_go)) {
            continue;
        }

        if factory.max_no_cost_geodes() <= best_geodes {
            continue;
        }

        if let Some(max_geodes) = factory.maxed_score(blueprint) {
            if max_geodes > best_geodes {
                best_geodes = max_geodes;
            }
            continue;
        }

        if factory.can_afford_robot(Material::Clay, blueprint)
            && factory.count_robots(Material::Clay) < max_clay_robots
        {
            let new_factory = factory.step_and_build(Some(Material::Clay), blueprint);
            state_queue.push_front(new_factory);
        }

        if factory.can_afford_robot(Material::Ore, blueprint)
            && factory.count_robots(Material::Ore) < max_ore_robots
        {
            let new_factory = factory.step_and_build(Some(Material::Ore), blueprint);
            state_queue.push_front(new_factory);
        }

        if factory.can_afford_robot(Material::Obsidian, blueprint)
            && factory.count_robots(Material::Obsidian) < max_obsidian_robots
        {
            let new_factory = factory.step_and_build(Some(Material::Obsidian), blueprint);
            state_queue.push_front(new_factory);
        }

        if factory.can_afford_robot(Material::Geode, blueprint) {
            let new_factory = factory.step_and_build(Some(Material::Geode), blueprint);
            state_queue.push_front(new_factory);
        }

        factory.mine_all();
        factory.minutes_to_go -= 1;
        state_queue.push_front(factory);
    }

    best_geodes
}

fn solve_part1(data: &[Blueprint]) -> usize {
    const MINUTES: usize = 24;

    let mut quality_levels = Vec::with_capacity(data.len());

    for blueprint in data.iter() {
        quality_levels.push(best_geode_count(blueprint, MINUTES));
    }

    quality_levels
        .iter()
        .enumerate()
        .map(|(id, geodes)| (id + 1) * geodes)
        .sum()
}

fn solve_part2(data: &[Blueprint]) -> usize {
    const MINUTES: usize = 32;

    let mut maxes = Vec::with_capacity(data.len());

    for blueprint in data.iter().take(3) {
        maxes.push(best_geode_count(blueprint, MINUTES));
    }

    maxes.iter().product()
}

generate_main_sample!();

generate_tests!(33, 62);
