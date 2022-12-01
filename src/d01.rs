mod utils;
const DAY_ID: utils::DayIdType = 1;

type Res = u32;

fn parse_input(data: &str) -> Vec<u32> {
    let mut res = vec![];

    for group in data.split("\n\n") {
        let s: u32 = group.lines().map(|x| x.parse::<u32>().unwrap()).sum();
        res.push(s);
    }

    res
}

fn solve_part1(data: &[u32]) -> Res {
    *data.iter().max().unwrap()
}

fn solve_part2(data: &[u32]) -> Res {
    let mut v = data.to_vec();
    v.sort();
    v.iter().rev().take(3).sum()
}

generate_main!();

generate_tests!(24000, 45000);
