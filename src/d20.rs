// #![warn(clippy::pedantic)]
mod utils;
const DAY_ID: utils::DayIdType = 20;

type Number = i64;

fn parse_input(data: &str) -> Vec<Number> {
    data.lines().map(|line| line.parse().unwrap()).collect()
}

#[derive(Debug, Copy, Clone)]
struct Value {
    start_index: usize,
    value: Number,
}

fn numbers_to_values(data: &[Number]) -> Vec<Value> {
    data.iter()
        .enumerate()
        .map(|(index, value)| Value {
            start_index: index,
            value: *value,
        })
        .collect::<Vec<_>>()
}

fn solve_part1(data: &[Number]) -> Number {
    let mut buffer = numbers_to_values(data);
    let buffer_len = buffer.len();

    for i in 0..buffer.len() {
        let (current_index, &value) = buffer
            .iter()
            .enumerate()
            .find(|(_position, v)| v.start_index == i)
            .unwrap();

        let new_index =
            (current_index as Number + value.value).rem_euclid(buffer_len as Number - 1) as usize;
        // println!(
        //     "[{}] <- moving {} from {current_index} to {new_index}",
        //     values_to_string(&buffer),
        //     value.value
        // );

        buffer.remove(current_index);
        buffer.insert(new_index, value);
    }

    let (zero_index, _) = buffer
        .iter()
        .enumerate()
        .find(|(_pos, v)| v.value == 0)
        .unwrap();

    [1000, 2000, 3000]
        .iter()
        .map(|x| buffer[(zero_index + x) % buffer_len].value)
        .sum()
}

fn solve_part2(data: &[Number]) -> Number {
    const DECRYPTION_KEY: Number = 811_589_153;

    let mut buffer = numbers_to_values(data);
    for val in &mut buffer {
        val.value *= DECRYPTION_KEY;
    }

    let buffer_len = buffer.len();

    for _ in 0..10 {
        for i in 0..data.len() {
            let (current_index, &value) = buffer
                .iter()
                .enumerate()
                .find(|(_position, v)| v.start_index == i)
                .unwrap();

            let new_index = (current_index as Number + value.value)
                .rem_euclid(buffer_len as Number - 1) as usize;
            // println!(
            //     "[{}] <- moving {} from {current_index} to {new_index}",
            //     values_to_string(&buffer),
            //     value.value
            // );

            buffer.remove(current_index);
            buffer.insert(new_index, value);
        }
    }

    let (zero_index, _) = buffer
        .iter()
        .enumerate()
        .find(|(_pos, v)| v.value == 0)
        .unwrap();

    [1000, 2000, 3000]
        .iter()
        .map(|x| buffer[(zero_index + x) % buffer_len].value)
        .sum()
}

generate_main!();

generate_tests!(3, 1_623_178_306);
