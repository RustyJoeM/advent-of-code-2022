use std::collections::HashSet;

mod utils;
const DAY_ID: utils::DayIdType = 3;

type Res = u32;

#[derive(Debug, Clone)]
pub struct Rucksack {
    pub contents: String,
    comp_a: HashSet<char>,
    comp_b: HashSet<char>,
}

impl From<&str> for Rucksack {
    fn from(s: &str) -> Self {
        let contents = s.to_string();
        let len = s.len();
        let comp_a = s.chars().take(len / 2).collect();
        let comp_b = s.chars().skip(len / 2).collect();
        Rucksack {
            contents,
            comp_a,
            comp_b,
        }
    }
}

impl Rucksack {
    pub fn intersection(&self) -> Vec<char> {
        self.comp_a.intersection(&self.comp_b).copied().collect()
    }

    pub fn common_char(&self) -> char {
        *self.intersection().first().unwrap()
    }
}

fn char_priority(ch: char) -> Res {
    let distance = ch.to_ascii_lowercase() as Res - 'a' as Res;
    let offset = if ch.is_ascii_uppercase() { 26 } else { 0 };
    distance + 1 + offset
}

fn parse_input(data: &str) -> Vec<Rucksack> {
    data.lines().map(Into::into).collect()
}

fn solve_part1(data: &[Rucksack]) -> Res {
    data.iter().map(|r| char_priority(r.common_char())).sum()
}

fn solve_part2(data: &[Rucksack]) -> Res {
    let mut priority = 0;

    for index in (0..data.len()).step_by(3) {
        let a = &data[index];
        let b = &data[index + 1];
        let c = &data[index + 2];

        for ch in a.contents.chars() {
            if b.contents.contains(ch) && c.contents.contains(ch) {
                priority += char_priority(ch);
                break;
            }
        }
    }
    priority
}

generate_main!();

generate_tests!(157, 70);
