mod utils;
const DAY_ID: utils::DayIdType = 1;

type Res = u32;

fn parse_input(data: &str) -> Vec<Vec<u32>> {
    let mut res = vec![];

    for group in data.split("\n\n") {
        let s = group.lines().map(|x| x.parse::<u32>().unwrap()).collect();
        res.push(s);
    }

    res
}

fn solve_part1(data: &[Vec<u32>]) -> Res {
    data.iter().map(|v| v.iter().sum()).max().unwrap()
}

fn solve_part2(data: &[Vec<u32>]) -> Res {
    let mut v: Vec<u32> = data.iter().map(|v| v.iter().sum()).collect();
    v.sort_unstable();
    v.iter().rev().take(3).sum()
}

generate_main!();

generate_tests!(24000, 45000);
