// #![warn(clippy::pedantic)]
mod utils;
const DAY_ID: utils::DayIdType = 25;

fn parse_input(data: &str) -> Vec<String> {
    data.lines().map(Into::into).collect()
}

type Number = i64;

fn snafu_to_decimal(snafu: &str) -> Number {
    let mut number = 0;
    for (index, digit) in snafu.chars().rev().enumerate() {
        number += (5 as Number).pow(index as u32)
            * match digit {
                '0' => 0,
                '1' => 1,
                '2' => 2,
                '-' => -1,
                '=' => -2,
                _ => panic!("Unexpected digit! {digit}"),
            }
    }
    number
}

fn decimal_to_snafu(num: Number) -> String {
    let mut snafu_digits = String::new();

    let mut leftover = num;
    while leftover > 0 {
        let remainder = leftover % 5;
        snafu_digits += match remainder {
            0 => "0",
            1 => "1",
            2 => "2",
            3 => {
                leftover += 2;
                "="
            }
            4 => {
                leftover += 1;
                "-"
            }
            _ => panic!("Unexpected digit {remainder}!"),
        };

        leftover /= 5;
    }

    snafu_digits.chars().rev().collect()
}

fn solve_part1(data: &[String]) -> String {
    let decimal: Number = data.iter().map(|line| snafu_to_decimal(line)).sum();
    decimal_to_snafu(decimal)
}

fn solve_part2(_data: &[String]) -> String {
    // no p2 on day 25 - personal pun...
    "Merry X-MAS!".to_string()
}

generate_main!();

generate_tests!("2=-1=0", "Merry X-MAS!");
