use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, value},
    sequence::preceded,
    IResult,
};

static EXAMPLE_INPUT: &str = include_str!("example.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Instruction {
    Noop,
    Addx(i32),
}

impl Instruction {
    fn parse(i: &str) -> IResult<&str, Self> {
        let noop = tag("noop");
        let addx = preceded(tag("addx "), nom::character::complete::i32);
        alt((value(Self::Noop, noop), map(addx, Self::Addx)))(i)
    }

    fn cycles(&self) -> u32 {
        match self {
            Self::Noop => 1,
            Self::Addx(_) => 2,
        }
    }
}

#[derive(Debug)]
struct Machine {
    instructions: Vec<Instruction>,
    current_instruction: Option<(Instruction, u32)>,
    cycle: u32,
    x_register: i32,
}

impl Machine {
    fn new(instructions: Vec<Instruction>) -> Self {
        let mut new = Machine {
            instructions: instructions.into_iter().rev().collect(),
            current_instruction: None,
            cycle: 1,
            x_register: 1,
        };
        new.decode();

        new
    }

    fn decode(&mut self) {
        self.current_instruction = self.instructions.pop().map(|i| (i, i.cycles()));
    }

    fn step(&mut self) -> bool {
        if self.current_instruction.is_none() {
            return false;
        }

        let (instruction, cycles_left) = self.current_instruction.as_mut().unwrap();
        *cycles_left -= 1;
        if *cycles_left == 0 {
            match instruction {
                Instruction::Noop => {}
                Instruction::Addx(x) => self.x_register += *x,
            }
            self.decode();
        }

        self.cycle += 1;
        true
    }
}

const DISPLAY_MASK: u64 = 0b1111111111111111111111111111111111111111;

fn sprite_value(pos: u32) -> u64 {
    (0b11100000000000000000000000000000000000000 >> pos) & DISPLAY_MASK
}

fn cycle_mask(cycle: u32) -> u64 {
    (0b1000000000000000000000000000000000000000 >> (cycle % 40)) & DISPLAY_MASK
}

fn main() {
    let input = include_str!("input.txt");
    // let input = EXAMPLE_INPUT;

    let instructions = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| Instruction::parse(line).unwrap().1)
        .collect::<Vec<_>>();

    let mut machine = Machine::new(instructions);

    let mut cycle_sum = 0;
    while machine.step() {
        match machine.cycle {
            20 | 60 | 100 | 140 | 180 | 220 => {
                println!("Cycle {}: {}", machine.cycle, machine.x_register);
                cycle_sum += machine.cycle as i32 * machine.x_register;
            }
            _ => (),
        }
    }

    println!("Part 1: {}", cycle_sum);
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_sprite_value() {
        assert_eq!(
            format!("{:040b}", sprite_value(0)),
            "1100000000000000000000000000000000000000"
        );
        assert_eq!(
            format!("{:040b}", sprite_value(1)),
            "1110000000000000000000000000000000000000"
        );
        assert_eq!(
            format!("{:040b}", sprite_value(38)),
            "0000000000000000000000000000000000000111"
        );
        assert_eq!(
            format!("{:040b}", sprite_value(39)),
            "0000000000000000000000000000000000000011"
        );
        assert_eq!(
            format!("{:040b}", sprite_value(40)),
            "0000000000000000000000000000000000000001"
        );
    }
}
