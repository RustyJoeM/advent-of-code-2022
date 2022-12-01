use std::fs::read_to_string;

pub type DayIdType = u8;

#[allow(dead_code)]
pub fn string_from_input(day: DayIdType) -> String {
    let path = format!("inputs/{:02}.txt", day);
    read_to_string(path).unwrap()
}

#[allow(dead_code)]
pub fn string_from_sample(day: DayIdType) -> String {
    let path = format!("inputs/{:02}-sample.txt", day);
    read_to_string(path).unwrap()
}

#[macro_export]
macro_rules! print_timed_result {
    ( $prefix:literal, $expression:expr ) => {
        let start = std::time::Instant::now();
        let result = $expression;
        let duration = start.elapsed();
        println!("{}: {:?} ({:?})", $prefix, result, duration);
    };
}

#[macro_export]
macro_rules! generate_main_input {
    ( $is_sample:expr ) => {
        fn main() {
            let data_str = if $is_sample {
                utils::string_from_sample(DAY_ID)
            } else {
                utils::string_from_input(DAY_ID)
            };
            let data;
            print_timed_result!("parse input", data = parse_input(&data_str));
            print_timed_result!("part 1", solve_part1(&data));
            print_timed_result!("part 2", solve_part2(&data));
        }
    };
}

#[macro_export]
macro_rules! generate_main {
    () => {
        generate_main_input!(false);
    };
}

#[macro_export]
macro_rules! generate_main_sample {
    () => {
        generate_main_input!(true);
    };
}

#[macro_export]
macro_rules! generate_tests {
    ( $part1_result:expr, $part2_result:expr ) => {
        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn part1_test() {
                let data_str = utils::string_from_sample(DAY_ID);
                let data = parse_input(&data_str);
                assert_eq!(solve_part1(&data), $part1_result);
            }

            #[test]
            fn part2_test() {
                let data_str = utils::string_from_sample(DAY_ID);
                let data = parse_input(&data_str);
                assert_eq!(solve_part2(&data), $part2_result);
            }
        }
    };
}
