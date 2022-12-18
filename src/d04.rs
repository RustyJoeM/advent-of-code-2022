mod utils;
const DAY_ID: utils::DayIdType = 4;

type Res = usize;

type Number = u32;
type Assignment = [Number; 4];

fn parse_input(data: &str) -> Vec<Assignment> {
    data.lines()
        .map(|x| {
            let elves: Vec<&str> = x.split(',').collect();
            let a: Vec<Number> = elves[0]
                .split('-')
                .map(|x| x.parse::<u32>().unwrap())
                .collect();
            let b: Vec<Number> = elves[1]
                .split('-')
                .map(|x| x.parse::<u32>().unwrap())
                .collect();
            [a[0], a[1], b[0], b[1]]
        })
        .collect()
}

pub fn has_complete_overlap(assignment: &Assignment) -> bool {
    let [a1, a2, b1, b2] = assignment;
    (a1 >= b1 && a2 <= b2) || (b1 >= a1 && b2 <= a2)
}

pub fn has_some_overlap(assignment: &Assignment) -> bool {
    let [a1, a2, b1, b2] = assignment;
    (a1 >= b1 && a1 <= b2)
        || (a2 >= b1 && a2 <= b2)
        || (b1 >= a1 && b1 <= a2)
        || (b2 >= a1 && b2 <= a2)
}

fn solve_part1(data: &[Assignment]) -> Res {
    data.iter().filter(|x| has_complete_overlap(x)).count()
}

fn solve_part2(data: &[Assignment]) -> Res {
    data.iter().filter(|x| has_some_overlap(x)).count()
}

generate_main!();

generate_tests!(2, 4);
