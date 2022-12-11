#![warn(clippy::pedantic)]
use std::collections::VecDeque;

mod utils;
const DAY_ID: utils::DayIdType = 11;

type Res = u64;

#[derive(Debug, Copy, Clone)]
enum Operation {
    Add(usize),
    Mul(usize),
    Pow,
}

impl From<&str> for Operation {
    // HACKY!
    fn from(s: &str) -> Self {
        // Operation: new = old + X
        if s.contains('+') {
            let add = s.split_ascii_whitespace().last().unwrap().parse().unwrap();
            return Self::Add(add);
        }
        // Operation: new = old * old
        if s.ends_with("old") {
            return Self::Pow;
        }
        // Operation: new = old * X
        let mul = s.split_ascii_whitespace().last().unwrap().parse().unwrap();
        Self::Mul(mul)
    }
}

#[derive(Debug, Copy, Clone)]
struct MonkeyTest {
    pub divisor: usize,
    pub true_target: usize,
    pub false_target: usize,
}

#[derive(Debug, Clone)]
struct Monkey {
    pub items: VecDeque<usize>,
    pub operation: Operation,
    pub test: MonkeyTest,
}

fn parse_input(data: &str) -> Vec<Monkey> {
    let mut monkeys = vec![];

    for group in data.split("\n\n") {
        let mut items = vec![];
        let mut operation = None;
        let mut divisor = 0;
        let mut true_target = 0;
        let mut false_target = 0;
        for line in group.lines() {
            if line.starts_with("  Starting items:") {
                items = line[18..].split(", ").map(|i| i.parse().unwrap()).collect();
            }

            if line.starts_with("  Operation:") {
                operation = Some(line.into());
            }

            if line.starts_with("  Test: divisible by ") {
                divisor = line
                    .split_ascii_whitespace()
                    .last()
                    .unwrap()
                    .parse()
                    .unwrap();
            }

            if line.starts_with("    If true") {
                true_target = line
                    .split_ascii_whitespace()
                    .last()
                    .unwrap()
                    .parse()
                    .unwrap();
            }

            if line.starts_with("    If false") {
                false_target = line
                    .split_ascii_whitespace()
                    .last()
                    .unwrap()
                    .parse()
                    .unwrap();
            }
        }
        monkeys.push(Monkey {
            items: VecDeque::from(items),
            operation: operation.unwrap(),
            test: MonkeyTest {
                divisor,
                true_target,
                false_target,
            },
        });
    }

    monkeys
}

fn solve_part1(data: &[Monkey]) -> Res {
    let mut monkeys = data.to_vec();
    let mut monkey_inspects = vec![0usize; monkeys.len()];

    for _ in 0..20 {
        for monkey_index in 0..monkeys.len() {
            let mut more_items = !monkeys[monkey_index].items.is_empty();
            while more_items {
                let mut item = monkeys[monkey_index].items.pop_front().unwrap();
                more_items = !monkeys[monkey_index].items.is_empty();

                item = match monkeys[monkey_index].operation {
                    Operation::Add(add) => item + add,
                    Operation::Mul(mul) => item * mul,
                    Operation::Pow => item * item,
                };
                item /= 3;

                let target = if (item % monkeys[monkey_index].test.divisor) == 0 {
                    monkeys[monkey_index].test.true_target
                } else {
                    monkeys[monkey_index].test.false_target
                };

                monkeys[target].items.push_back(item);

                monkey_inspects[monkey_index] += 1;
            }
        }
    }

    monkey_inspects.sort_unstable();
    let len = monkey_inspects.len();
    monkey_inspects[len - 1] as Res * monkey_inspects[len - 2] as Res
}

fn solve_part2(data: &[Monkey]) -> Res {
    let mut monkeys = data.to_vec();
    let mut monkey_inspects = vec![0usize; monkeys.len()];

    let common_divisor = monkeys.iter().map(|m| m.test.divisor).product::<usize>();

    for _ in 0..10_000 {
        for monkey_index in 0..monkeys.len() {
            let mut more_items = !monkeys[monkey_index].items.is_empty();
            while more_items {
                let mut item = monkeys[monkey_index].items.pop_front().unwrap();
                more_items = !monkeys[monkey_index].items.is_empty();

                item = match monkeys[monkey_index].operation {
                    Operation::Add(add) => item + add,
                    Operation::Mul(mul) => item * mul,
                    Operation::Pow => item * item,
                };

                let target = if (item % monkeys[monkey_index].test.divisor) == 0 {
                    monkeys[monkey_index].test.true_target
                } else {
                    monkeys[monkey_index].test.false_target
                };

                let target_item = item % common_divisor;

                monkeys[target].items.push_back(target_item);
                monkey_inspects[monkey_index] += 1;
            }
        }
    }

    monkey_inspects.sort_unstable();
    let len = monkey_inspects.len();
    monkey_inspects[len - 1] as Res * monkey_inspects[len - 2] as Res
}

generate_main!();

generate_tests!(10605, 2_713_310_158);
