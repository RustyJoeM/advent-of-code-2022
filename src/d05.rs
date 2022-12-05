mod utils;
const DAY_ID: utils::DayIdType = 5;

type Res = String;

#[derive(Debug, Clone)]
pub struct Move {
    pub count: usize,
    pub from_index: usize,
    pub to_index: usize,
}

#[derive(Debug, Clone)]
struct Towers {
    towers: Vec<Vec<char>>,
}

impl Towers {
    pub fn process_move(&mut self, m: &Move) {
        for _ in 0..m.count {
            let letter = self.towers[m.from_index - 1].pop().unwrap();
            self.towers[m.to_index - 1].push(letter);
        }
    }

    pub fn process_move_new(&mut self, m: &Move) {
        let split_index = self.towers[m.from_index - 1].len() - m.count;
        let mut carry = self.towers[m.from_index - 1].split_off(split_index);
        self.towers[m.to_index - 1].append(&mut carry);
    }

    pub fn top_crates(&self) -> String {
        self.towers.iter().map(|t| t[t.len() - 1]).collect()
    }
}

fn parse_input(data: &str) -> (Towers, Vec<Move>) {
    let (tower_text, moves_text) = data.split_once("\n\n").unwrap();

    let tower_count = (tower_text.lines().next().unwrap().len() + 1) / 4;
    let mut towers = vec![Vec::new(); tower_count];

    for line in tower_text.lines() {
        // init state section stops on first line with no crate displayed [.]
        if !line.contains('[') {
            break;
        }
        let chars: Vec<char> = line.chars().collect();
        for index in 0..tower_count {
            let letter = chars[index * 4 + 1];
            if letter.is_ascii_alphabetic() {
                towers[index].push(letter);
            }
        }
    }

    for tower in &mut towers {
        tower.reverse();
    }

    let moves = moves_text
        .lines()
        .map(|line| {
            let values: Vec<usize> = line
                .split(' ')
                .skip(1)
                .step_by(2)
                .map(|x| x.parse::<usize>().unwrap())
                .collect();
            Move {
                count: values[0],
                from_index: values[1],
                to_index: values[2],
            }
        })
        .collect();

    (Towers { towers }, moves)
}

fn solve_part1((towers, moves): &(Towers, Vec<Move>)) -> Res {
    let mut my_towers = towers.clone();
    for m in moves {
        my_towers.process_move(m);
    }

    my_towers.top_crates()
}

fn solve_part2((towers, moves): &(Towers, Vec<Move>)) -> Res {
    let mut my_towers = towers.clone();
    for m in moves {
        my_towers.process_move_new(m);
    }
    my_towers.top_crates()
}

generate_main!();

generate_tests!("CMZ".to_string(), "MCD".to_string());
