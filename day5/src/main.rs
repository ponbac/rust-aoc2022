use std::fmt::Debug;

use itertools::Itertools;

static EXAMPLE_INPUT: &str = r#"
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
"#;

#[derive(Clone)]
struct Shipyard(Vec<Vec<Option<char>>>);

impl Debug for Shipyard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut shipyard = "\n".to_owned();
        for row in &self.0 {
            shipyard.push('[');
            shipyard.push_str(&row.iter().map(|c| c.unwrap_or(' ')).collect::<String>());
            shipyard.push_str("]\n");
        }

        write!(f, "{}", shipyard)
    }
}

impl Shipyard {
    fn from_str(input: &str) -> Self {
        let mut shipyard = Vec::new();
        for line in input.lines().filter(|l| !l.is_empty()) {
            if line.starts_with("move") || line.starts_with(" 1") {
                break;
            }

            let crates = (" ".to_owned() + line)
                .chars()
                .chunks(4)
                .into_iter()
                .map(|mut chunk| parse_crate(&chunk.join("")))
                .collect::<Vec<Option<char>>>();

            shipyard.push(crates);
        }

        let transposed = transpose(shipyard);
        Self(
            transposed
                .iter()
                .map(|row| row.iter().filter(|c| c.is_some()).cloned().collect())
                .collect(),
        )
    }

    fn apply(&mut self, instruction: Instruction) {
        let mut from = self.0[instruction.from - 1].clone();
        let mut to = self.0[instruction.to - 1].clone();

        let mut n_to_take = instruction.num;
        while n_to_take > 0 {
            if from.last().is_some() {
                if to.last().is_none() {
                    to.pop();
                }

                to.push(from.pop().unwrap());
                n_to_take -= 1;
            } else {
                from.pop();
            }
        }

        self.0[instruction.from - 1] = from;
        self.0[instruction.to - 1] = to;
    }
}

#[derive(Debug)]
struct Instruction {
    num: usize,
    from: usize,
    to: usize,
}

impl Instruction {
    fn from_str(input: &str) -> Self {
        // grab only the numbers
        let nums = input
            .chars()
            .filter(|c| c.is_ascii_digit())
            .collect::<String>()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect::<Vec<usize>>();

        let num = nums[0];
        let from = nums[1];
        let to = nums[2];

        Self { num, from, to }
    }
}

fn main() {
    let _input = include_str!("input.txt");

    part1(EXAMPLE_INPUT);
}

fn part1(input: &str) {
    let mut shipyard = Shipyard::from_str(input);
    println!("{:?}", shipyard);

    let instructions = input
        .lines()
        .filter(|l| l.starts_with("move"))
        .map(Instruction::from_str)
        .collect::<Vec<Instruction>>();
    println!("{:?}", instructions);

    for instruction in instructions {
        shipyard.apply(instruction);
        println!("{:?}", shipyard);
    }
}

fn parse_crate(i: &str) -> Option<char> {
    i.chars().nth(2)
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .rev()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}
