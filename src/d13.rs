#![warn(clippy::pedantic)]
use std::cmp::Ordering;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, one_of},
    combinator::recognize,
    multi::{many0, many1, separated_list0},
    sequence::{preceded, terminated},
    IResult,
};

mod utils;
const DAY_ID: utils::DayIdType = 13;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Item {
    Num(usize),
    List(Vec<Item>),
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match items_ordered(self, other) {
            Some(is_ordered) => {
                if is_ordered {
                    Some(Ordering::Less)
                } else {
                    Some(Ordering::Greater)
                }
            }
            None => None,
        }
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Item::Num(e1), Item::Num(e2)) => e1.cmp(e2),
            (Item::List(l1), Item::List(l2)) => l1.cmp(l2),
            (Item::List(l1), Item::Num(e2)) => l1.as_slice().cmp(&[Item::Num(*e2)]),
            (Item::Num(e1), Item::List(l2)) => [Item::Num(*e1)].as_slice().cmp(l2.as_slice()),
        }
    }
}

type ItemPair = (Item, Item);

fn integer_parser(input: &str) -> IResult<&str, Item> {
    let (rest, num_str) =
        recognize(many1(terminated(one_of("0123456789"), many0(char('_')))))(input)?;
    let num = num_str.parse::<usize>().unwrap();
    IResult::Ok((rest, Item::Num(num)))
}

fn list_parser(input: &str) -> IResult<&str, Item> {
    let (rest, v) = separated_list0(tag(","), item_parser)(input)?;
    IResult::Ok((rest, Item::List(v)))
}

fn item_parser(input: &str) -> IResult<&str, Item> {
    alt((
        integer_parser,
        terminated(preceded(char('['), list_parser), char(']')),
    ))(input)
}

fn items_ordered(left: &Item, right: &Item) -> Option<bool> {
    match (left, right) {
        (Item::Num(l), Item::Num(r)) => {
            if l == r {
                None
            } else {
                Some(l < r)
            }
        }
        (Item::List(l), Item::List(r)) => {
            let left_len = l.len();
            let right_len = r.len();
            // If both values are lists, compare the first value of each list,
            // then the second value, and so on.
            let count = left_len.max(right_len);
            for i in 0..count {
                // If the left list runs out of items first,
                // the inputs are in the right order.
                if i >= left_len && right_len > left_len {
                    // println!("\tleft ran out");
                    return Some(true);
                }

                // If the right list runs out of items first,
                // the inputs are not in the right order.
                if i >= right_len && left_len > right_len {
                    // println!("\tright ran out");
                    return Some(false);
                }

                // If the lists are the same length and
                // no comparison makes a decision about the order,
                // continue checking the next part of the input.
                let cmp = items_ordered(&l[i], &r[i]);
                if cmp.is_some() {
                    return cmp;
                }
            }

            None
        }
        (Item::Num(_), Item::List(_)) => items_ordered(&Item::List(vec![left.clone()]), right),
        (Item::List(_), Item::Num(_)) => items_ordered(left, &Item::List(vec![right.clone()])),
    }
}

fn parse_input(data: &str) -> Vec<ItemPair> {
    data.split("\n\n")
        .map(|x| {
            let (a, b) = x.split_once('\n').unwrap();
            let (_, a) = item_parser(a).unwrap();
            let (_, b) = item_parser(b).unwrap();
            (a, b)
        })
        .collect()
}

fn solve_part1(data: &[ItemPair]) -> usize {
    let ordered = data
        .iter()
        .enumerate()
        .filter(|(_, (left, right))| {
            if let Some(is_ordered) = items_ordered(left, right) {
                is_ordered
            } else {
                false
            }
        })
        .collect::<Vec<(usize, &ItemPair)>>();

    ordered.iter().map(|(num, _)| num + 1).sum()
}

fn solve_part2(data: &[ItemPair]) -> usize {
    let divider_2: Item = Item::List(vec![Item::List(vec![Item::Num(2)])]);
    let divider_6: Item = Item::List(vec![Item::List(vec![Item::Num(6)])]);

    let mut items = vec![divider_2.clone(), divider_6.clone()];
    for (left, right) in data.iter() {
        items.push(left.clone());
        items.push(right.clone());
    }
    items.sort();

    let index2 = items.iter().position(|i| *i == divider_2).unwrap() + 1;
    let index6 = items.iter().position(|i| *i == divider_6).unwrap() + 1;

    index2 * index6
}

generate_main!();

generate_tests!(13, 140);
