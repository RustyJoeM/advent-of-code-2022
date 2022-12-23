#![warn(clippy::pedantic)]
use std::collections::HashMap;

mod utils;
const DAY_ID: utils::DayIdType = 21;

type Number = i64;

#[derive(Debug, Copy, Clone)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

impl From<&str> for Operation {
    fn from(s: &str) -> Self {
        match s {
            "+" => Self::Add,
            "-" => Self::Sub,
            "*" => Self::Mul,
            "/" => Self::Div,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone)]
enum Job {
    Number(Number),
    Operation(Operation, String, String),
}

#[derive(Debug, Clone)]
struct Monkey {
    pub name: String,
    pub job: Job,
}

impl From<&str> for Monkey {
    fn from(s: &str) -> Self {
        // bit hacky but...
        let (name, rest) = s.split_once(": ").unwrap();
        let name = name.to_string();
        let job: Job = if rest.contains(' ') {
            let words = rest.split_ascii_whitespace().collect::<Vec<_>>();
            Job::Operation(words[1].into(), words[0].to_string(), words[2].to_string())
        } else {
            let num = rest.parse().unwrap();
            Job::Number(num)
        };
        Self { name, job }
    }
}

struct MonkeyPack {
    monkeys: HashMap<String, Monkey>,
}

const ROOT_NAME: &str = "root";
const HUMAN_NAME: &str = "humn";

impl MonkeyPack {
    pub fn from_monkeys(input: &[Monkey]) -> Self {
        Self {
            monkeys: input.iter().map(|m| (m.name.clone(), m.clone())).collect(),
        }
    }

    pub fn monkey_shout(&self, name: &str) -> Number {
        let monkey = self.monkeys.get(name).unwrap();
        match &monkey.job {
            Job::Number(num) => *num,
            Job::Operation(op, left, right) => {
                let m1 = self.monkey_shout(left);
                let m2 = self.monkey_shout(right);
                match op {
                    Operation::Add => m1 + m2,
                    Operation::Sub => m1 - m2,
                    Operation::Mul => m1 * m2,
                    Operation::Div => m1 / m2,
                }
            }
        }
    }

    fn to_buffer(&self, buffer: &mut String, name: &str, is_part2: bool) {
        let monkey = self.monkeys.get(name).unwrap();
        match &monkey.job {
            Job::Number(num) => {
                if is_part2 && name == HUMAN_NAME {
                    buffer.push('x');
                } else {
                    buffer.push_str(&format!("{num}"));
                }
            }
            Job::Operation(op, left, right) => {
                buffer.push('(');
                self.to_buffer(buffer, left, is_part2);
                if name == ROOT_NAME && is_part2 {
                    buffer.push('=');
                } else {
                    buffer.push(match op {
                        Operation::Add => '+',
                        Operation::Sub => '-',
                        Operation::Mul => '*',
                        Operation::Div => '/',
                    });
                }
                self.to_buffer(buffer, right, is_part2);
                buffer.push(')');
            }
        }
    }

    #[allow(dead_code)]
    pub fn to_string(&self, is_part2: bool) -> String {
        let mut buffer = String::new();
        self.to_buffer(&mut buffer, ROOT_NAME, is_part2);
        buffer
    }

    pub fn is_human_affected(&self, name: &str) -> bool {
        let monkey = self.monkeys.get(name).unwrap();
        match &monkey.job {
            Job::Number(_) => monkey.name == HUMAN_NAME,
            Job::Operation(_, left, right) => {
                self.is_human_affected(left) || self.is_human_affected(right)
            }
        }
    }

    fn reduce_once(&mut self, name: &str) -> bool {
        let job = self.monkeys.get(name).unwrap().job.clone();
        if let Job::Operation(_, left, right) = job {
            let left_affected = self.is_human_affected(&left);
            let right_affected = self.is_human_affected(&right);
            match (left_affected, right_affected) {
                (true, true) => false,
                (true, false) | (false, true) => {
                    self.reduce_once(&left) || self.reduce_once(&right)
                }
                (false, false) => {
                    let new_value = self.monkey_shout(name);
                    let mut monkey = self.monkeys.get_mut(name).unwrap();
                    monkey.job = Job::Number(new_value);
                    true
                }
            }
        } else {
            false
        }
    }

    pub fn normalize(&mut self) {
        while self.reduce_once(ROOT_NAME) {}
    }

    pub fn root_children_names(&self) -> (&str, &str) {
        let root_monkey = self.monkeys.get(ROOT_NAME).unwrap();
        if let Job::Operation(_, left, right) = &root_monkey.job {
            (left, right)
        } else {
            unreachable!();
        }
    }
}

fn parse_input(data: &str) -> Vec<Monkey> {
    data.lines().map(Into::into).collect()
}

fn solve_part1(data: &[Monkey]) -> Number {
    let pack = MonkeyPack::from_monkeys(data);
    // println!("{}", pack.to_string(false));
    pack.monkey_shout(ROOT_NAME)
}

fn solve_part2(data: &[Monkey]) -> Number {
    let mut pack = MonkeyPack::from_monkeys(data);

    pack.normalize();

    let (left, right) = pack.root_children_names();
    let (mut human_branch, result_branch) = if pack.is_human_affected(left) {
        (left, right)
    } else {
        (right, left)
    };

    let mut result = pack.monkey_shout(result_branch);

    while human_branch != HUMAN_NAME {
        let m = pack.monkeys.get(human_branch).unwrap();

        let mut buffer = String::new();
        pack.to_buffer(&mut buffer, human_branch, true);
        // println!("\t{buffer} == {result}");

        if let Job::Operation(op, left, right) = &m.job {
            let left_monkey = pack.monkeys.get(left).unwrap();
            let right_monkey = pack.monkeys.get(right).unwrap();

            match (&left_monkey.job, &right_monkey.job) {
                (Job::Number(num), Job::Operation(_, _, _)) => {
                    match op {
                        Operation::Add => result -= num,
                        Operation::Sub => result = -result + num,
                        Operation::Mul => result /= num,
                        Operation::Div => result = num / result,
                    }
                    human_branch = right;
                }
                (Job::Operation(_, _, _), Job::Number(num)) => {
                    match op {
                        Operation::Add => result -= num,
                        Operation::Sub => result += num,
                        Operation::Mul => result /= num,
                        Operation::Div => result *= num,
                    }
                    human_branch = left;
                }
                (Job::Number(num1), Job::Number(num2)) => {
                    if left_monkey.name == HUMAN_NAME {
                        match op {
                            Operation::Add => result -= num2,
                            Operation::Sub => result += num2,
                            Operation::Mul => result /= num2,
                            Operation::Div => result *= num2,
                        }
                        human_branch = left;
                    } else if right_monkey.name == HUMAN_NAME {
                        match op {
                            Operation::Add => result -= num1,
                            Operation::Sub => result = -result + num1,
                            Operation::Mul => result /= num1,
                            Operation::Div => result = num1 / result,
                        }
                        human_branch = right;
                    } else {
                        panic!("Non-normalized pack cannot be solved!")
                    }
                }
                (Job::Operation(_, _, _), Job::Operation(_, _, _)) => {
                    // println!(
                    //     "left: {:?}\nright: {:?}",
                    //     &left_monkey.job, &right_monkey.job
                    // );
                    panic!("Non-normalized pack cannot be solved!")
                }
            }
        } else {
            unreachable!();
        }
    }

    result
}

generate_main!();

generate_tests!(152, 301);
