use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::space1,
    combinator::{map, value},
    sequence::{preceded, tuple},
    IResult,
};
use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct GridPos {
    pub x: i32,
    pub y: i32,
}

impl fmt::Debug for GridPos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl std::ops::Add for GridPos {
    type Output = GridPos;

    fn add(self, other: GridPos) -> GridPos {
        GridPos {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::AddAssign for GridPos {
    fn add_assign(&mut self, other: GridPos) {
        *self = GridPos {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl std::ops::Sub for GridPos {
    type Output = GridPos;

    fn sub(self, other: GridPos) -> GridPos {
        GridPos {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn parse(i: &str) -> IResult<&str, Self> {
        alt((
            value(Direction::Up, tag("U")),
            value(Direction::Down, tag("D")),
            value(Direction::Left, tag("L")),
            value(Direction::Right, tag("R")),
        ))(i)
    }

    pub fn delta(self) -> GridPos {
        match self {
            Direction::Up => GridPos { x: 0, y: -1 },
            Direction::Down => GridPos { x: 0, y: 1 },
            Direction::Left => GridPos { x: -1, y: 0 },
            Direction::Right => GridPos { x: 1, y: 0 },
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Instruction {
    pub dir: Direction,
    pub dist: u32,
}

impl Instruction {
    pub fn parse(i: &str) -> IResult<&str, Self> {
        map(
            tuple((
                Direction::parse,
                preceded(space1, nom::character::complete::u32),
            )),
            |(dir, dist)| Self { dir, dist },
        )(i)
    }
}
