use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
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

fn main() {
    // let input = include_str!("input.txt");
    let input = EXAMPLE_INPUT;

    let instructions = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| Instruction::parse(line).unwrap().1)
        .collect::<Vec<_>>();

    for instruction in instructions {
        println!("{:?}", instruction);
    }
}
