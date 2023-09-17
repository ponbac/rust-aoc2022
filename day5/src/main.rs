use std::fmt::{self, Debug, Display};

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_while1},
    combinator::{all_consuming, map, map_res, opt},
    sequence::{delimited, preceded, tuple},
    Finish, IResult,
};

#[derive(Clone, Copy)]
struct Crate(char);

impl Debug for Crate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Display for Crate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Instruction {
    quantity: usize,
    src: usize,
    dst: usize,
}

#[derive(Clone)]
struct Piles(Vec<Vec<Crate>>);

impl Debug for Piles {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, pile) in self.0.iter().enumerate() {
            writeln!(f, "Pile {}: {:?}", i, pile)?;
        }
        Ok(())
    }
}

impl Piles {
    fn apply(&mut self, ins: &Instruction) {
        for _ in 0..ins.quantity {
            let crate_ = self.0[ins.src].pop().unwrap();
            self.0[ins.dst].push(crate_);
        }
    }

    fn apply2(&mut self, ins: &Instruction) {
        let mut crates_to_move = Vec::new();
        for _ in 0..ins.quantity {
            let crate_ = self.0[ins.src].pop().unwrap();
            crates_to_move.push(crate_);
        }

        for crate_ in crates_to_move.into_iter().rev() {
            self.0[ins.dst].push(crate_);
        }
    }
}

fn main() {
    let input = include_str!("input.txt");

    part1(input);
}

fn part1(input: &str) {
    let mut lines = input.lines().skip_while(|line| line.is_empty());

    let crate_lines: Vec<_> = (&mut lines)
        .map_while(|line| {
            all_consuming(parse_crate_line)(line)
                .finish()
                .ok()
                .map(|(_, line)| line)
        })
        .collect();
    let mut piles1 = Piles(transpose_rev(crate_lines));
    let mut piles2 = piles1.clone();
    println!("{piles1:?}");

    // we've consumed the "numbers line" but not the separating line
    assert!(lines.next().unwrap().is_empty());

    for i in lines.map(|line| all_consuming(parse_instruction)(line).finish().unwrap().1) {
        piles1.apply(&i);
        piles2.apply2(&i);
    }

    println!(
        "Part 1: {:?}",
        piles1.0.iter().map(|p| p.last().unwrap()).join("")
    );
    println!(
        "Part 2: {:?}",
        piles2.0.iter().map(|p| p.last().unwrap()).join("")
    );
}

fn transpose_rev<T>(v: Vec<Vec<Option<T>>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .rev()
                .filter_map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn parse_crate(i: &str) -> IResult<&str, Crate> {
    let first_char = |s: &str| Crate(s.chars().next().unwrap());
    let f = delimited(tag("["), take(1_usize), tag("]"));
    map(f, first_char)(i)
}

fn parse_hole(i: &str) -> IResult<&str, ()> {
    map(tag("   "), drop)(i)
}

fn parse_crate_or_hole(i: &str) -> IResult<&str, Option<Crate>> {
    alt((map(parse_crate, Some), map(parse_hole, |_| None)))(i)
}

fn parse_crate_line(i: &str) -> IResult<&str, Vec<Option<Crate>>> {
    let (mut i, c) = parse_crate_or_hole(i)?;
    let mut v = vec![c];

    loop {
        let (next_i, maybe_c) = opt(preceded(tag(" "), parse_crate_or_hole))(i)?;
        match maybe_c {
            Some(c) => v.push(c),
            None => break,
        }
        i = next_i;
    }

    Ok((i, v))
}

fn parse_number(i: &str) -> IResult<&str, usize> {
    map_res(take_while1(|c: char| c.is_ascii_digit()), |s: &str| {
        s.parse::<usize>()
    })(i)
}

fn parse_pile_number(i: &str) -> IResult<&str, usize> {
    map(parse_number, |i| i - 1)(i)
}

fn parse_instruction(i: &str) -> IResult<&str, Instruction> {
    map(
        tuple((
            preceded(tag("move "), parse_number),
            preceded(tag(" from "), parse_pile_number),
            preceded(tag(" to "), parse_pile_number),
        )),
        |(quantity, src, dst)| Instruction { quantity, src, dst },
    )(i)
}
