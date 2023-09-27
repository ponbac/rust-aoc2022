use std::{collections::VecDeque, fmt};

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

struct Machine {
    instructions: VecDeque<Instruction>,
    current_instruction: Option<(Instruction, u32)>,
    cycle: u32,
    x_register: i32,
    display_lines: Vec<u64>,
}

impl fmt::Debug for Machine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "cycle={} x={} current={:?} ({} instructions left)",
            self.cycle,
            self.x_register,
            self.current_instruction,
            self.instructions.len()
        )?;
        for line in &self.display_lines {
            for i in 0..40 {
                let c = if line & cycle_mask(i) > 0 { '#' } else { '.' };
                write!(f, "{c}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Machine {
    fn new(instructions: VecDeque<Instruction>) -> Self {
        let mut new = Machine {
            instructions,
            current_instruction: None,
            cycle: 0,
            x_register: 1,
            display_lines: Vec::new(),
        };
        new.decode();

        new
    }

    fn decode(&mut self) {
        self.current_instruction = self.instructions.pop_front().map(|ins| (ins, ins.cycles()));
    }

    fn draw(&mut self) {
        let crt_line = (self.cycle / 40) as usize;
        if crt_line + 1 > self.display_lines.len() {
            self.display_lines.push(0);
        }
        let crt_line = self.display_lines.get_mut(crt_line).unwrap();
        let cycle_mask = cycle_mask(self.cycle);
        let sprite = sprite_value(self.x_register as _);
        *crt_line |= cycle_mask & sprite;
    }

    fn step(&mut self) -> bool {
        if self.current_instruction.is_none() {
            return false;
        }

        let (ins, cycles_left) = self.current_instruction.as_mut().unwrap();
        *cycles_left -= 1;
        if *cycles_left == 0 {
            match ins {
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

fn sprite_value(pos: i32) -> u64 {
    let model = 0b11100000000000000000000000000000000000000_u64;
    let shifted;
    if pos < 0 {
        (shifted, _) = model.overflowing_shl((-pos).try_into().unwrap());
    } else {
        (shifted, _) = model.overflowing_shr(pos.try_into().unwrap());
    }
    shifted & DISPLAY_MASK
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
        .collect::<VecDeque<_>>();

    let mut machine = Machine::new(instructions);

    loop {
        machine.draw();
        println!("{:?}", machine);
        if !machine.step() {
            break;
        }
    }
    println!("Part 2: {:?}", machine);
}
