mod utils;
const DAY_ID: utils::DayIdType = 2;

type Res = u32;

#[derive(Debug, Copy, Clone)]
enum HandResult {
    Win,
    Draw,
    Loss,
}

impl HandResult {
    pub fn score(&self) -> Res {
        match self {
            HandResult::Win => 6,
            HandResult::Draw => 3,
            HandResult::Loss => 0,
        }
    }

    pub fn from_hand(hand: Hand) -> Self {
        match hand {
            Hand::Rock => HandResult::Loss,
            Hand::Paper => HandResult::Draw,
            Hand::Scissors => HandResult::Win,
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl From<&str> for Hand {
    fn from(s: &str) -> Self {
        match s {
            "A" | "X" => Hand::Rock,
            "B" | "Y" => Hand::Paper,
            "C" | "Z" => Hand::Scissors,
            _ => unreachable!(),
        }
    }
}

impl Hand {
    pub fn score(&self) -> Res {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }

    pub fn against(&self, other: Hand) -> HandResult {
        match self {
            Hand::Rock => match other {
                Hand::Rock => HandResult::Draw,
                Hand::Paper => HandResult::Loss,
                Hand::Scissors => HandResult::Win,
            },
            Hand::Paper => match other {
                Hand::Rock => HandResult::Win,
                Hand::Paper => HandResult::Draw,
                Hand::Scissors => HandResult::Loss,
            },
            Hand::Scissors => match other {
                Hand::Rock => HandResult::Loss,
                Hand::Paper => HandResult::Win,
                Hand::Scissors => HandResult::Draw,
            },
        }
    }

    pub fn opponents_hand(&self, opponents_result: HandResult) -> Hand {
        match opponents_result {
            HandResult::Loss => match self {
                Hand::Rock => Hand::Scissors,
                Hand::Paper => Hand::Rock,
                Hand::Scissors => Hand::Paper,
            },
            HandResult::Draw => *self,
            HandResult::Win => match self {
                Hand::Rock => Hand::Paper,
                Hand::Paper => Hand::Scissors,
                Hand::Scissors => Hand::Rock,
            },
        }
    }
}

fn parse_input(data: &str) -> Vec<(Hand, Hand)> {
    data.lines()
        .map(|x| {
            let draws: Vec<&str> = x.split_whitespace().collect();
            (draws[0].into(), draws[1].into())
        })
        .collect()
}

fn solve_part1(data: &[(Hand, Hand)]) -> Res {
    data.iter()
        .map(|(enemy, me)| me.score() + me.against(*enemy).score())
        .sum()
}

fn solve_part2(data: &[(Hand, Hand)]) -> Res {
    data.iter()
        .map(|(enemy, me)| (enemy, HandResult::from_hand(*me)))
        .map(|(enemy, needed_result)| {
            // "fix" parsing of data for new elf instructions...
            let me = enemy.opponents_hand(needed_result);
            me.score() + me.against(*enemy).score()
        })
        .sum()
}

generate_main!();

generate_tests!(15, 12);
