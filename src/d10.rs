// #![warn(clippy::pedantic)]
mod utils;
const DAY_ID: utils::DayIdType = 10;

type Res = i32;

#[derive(Debug, Copy, Clone)]
enum Command {
    AddX(Res),
    Noop,
}

impl From<&str> for Command {
    fn from(s: &str) -> Self {
        if s == "noop" {
            Self::Noop
        } else {
            let (_, arg) = s.split_once(' ').unwrap();
            Self::AddX(arg.parse::<Res>().unwrap())
        }
    }
}

const CYCLES_TO_CHECK: [Res; 6] = [20, 60, 100, 140, 180, 220];

impl Command {
    pub fn cycles(self) -> Res {
        match self {
            Command::AddX(_) => 2,
            Command::Noop => 1,
        }
    }
}

fn parse_input(data: &str) -> Vec<Command> {
    data.lines().map(Into::into).collect()
}

fn solve_part1(data: &[Command]) -> Res {
    let mut x = 1;

    let mut signal = 0;

    let mut cycles = 0;

    let mut last_was_add = false;

    let mut checked = 0; // compensate for some bug mis-indexing last item in do_check

    for (step, command) in data.iter().enumerate() {
        let do_check = CYCLES_TO_CHECK.contains(&cycles)
            || (last_was_add && CYCLES_TO_CHECK.contains(&(cycles - 1)));

        if do_check {
            let correct_cycle = CYCLES_TO_CHECK[checked];
            let update = if let Command::AddX(fix) = &data[step - 1] {
                (x - *fix) * correct_cycle
            } else {
                x * correct_cycle
            };
            signal += update;
            checked += 1;
        }
        cycles += command.cycles();
        match command {
            Command::AddX(num) => {
                x += num;
                last_was_add = true;
            }
            Command::Noop => {
                last_was_add = false;
            }
        }
    }

    signal
}

fn solve_part2(data: &[Command]) -> Res {
    const COLS: i32 = 40;
    const ROWS: i32 = 6;
    let mut screen = [['.'; COLS as usize]; ROWS as usize];

    for cycle in 0i32..(ROWS * COLS) {
        let row = cycle / 40;
        let col = cycle % 40;

        let mut cursor = 1;
        let mut cycles_needed = 0;
        let mut prev_add;
        for command in data.iter() {
            match command {
                Command::AddX(num) => {
                    cursor += num;
                    prev_add = Some(num);
                }
                Command::Noop => {
                    prev_add = None;
                }
            }
            cycles_needed += command.cycles();
            if cycles_needed > cycle {
                if let Some(num) = prev_add {
                    cursor -= num;
                }
                break;
            }
        }

        if [cursor - 1, cursor, cursor + 1].contains(&(col as Res)) {
            screen[row as usize][col as usize] = '#';
        }
    }

    for row in screen {
        let row_str: String = row.iter().collect();
        println!("\t{}", row_str);
    }

    0
}

generate_main!();

generate_tests!(13140, 0);
