#[derive(Debug, Clone, Copy)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}
impl Shape {
    fn from_str(s: &str) -> Self {
        match s {
            "A" | "X" => Shape::Rock,
            "B" | "Y" => Shape::Paper,
            "C" | "Z" => Shape::Scissors,
            _ => panic!("Invalid shape"),
        }
    }

    fn beats(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Shape::Rock, Shape::Scissors)
                | (Shape::Paper, Shape::Rock)
                | (Shape::Scissors, Shape::Paper)
        )
    }
}

#[derive(Debug, Clone, Copy)]
enum Outcome {
    Win = 6,
    Tie = 3,
    Loss = 0,
}

impl Outcome {
    fn from_str(s: &str) -> Self {
        match s {
            "X" => Outcome::Loss,
            "Y" => Outcome::Tie,
            "Z" => Outcome::Win,
            _ => panic!("Invalid shape"),
        }
    }
}

struct Game {
    opponent: Shape,
    response: Shape,
}

impl Game {
    fn new(opponent: Shape, response: Shape) -> Self {
        Game { opponent, response }
    }

    fn score(&self) -> i32 {
        let outcome = if self.opponent.beats(&self.response) {
            Outcome::Loss
        } else if self.response.beats(&self.opponent) {
            Outcome::Win
        } else {
            Outcome::Tie
        };

        outcome as i32 + self.response as i32
    }
}

fn main() {
    let input = include_str!("input.txt");

    part1(input);
    part2(input);
}

fn part1(input: &str) {
    let mut score: i32 = 0;
    for line in input.lines().filter(|line| !line.trim().is_empty()) {
        let mut chars = line.split_whitespace();

        let opponent = Shape::from_str(chars.next().unwrap());
        let response = Shape::from_str(chars.next().unwrap());
        let game = Game::new(opponent, response);
        score += game.score();
    }

    println!("Part 1: {}", score);
}

fn part2(input: &str) {
    let mut score: i32 = 0;
    for line in input.lines().filter(|line| !line.trim().is_empty()) {
        let mut chars = line.split_whitespace();

        let opponent = Shape::from_str(chars.next().unwrap());

        let needed_outcome = Outcome::from_str(chars.next().unwrap());
        let response = match needed_outcome {
            Outcome::Win => match opponent {
                Shape::Rock => Shape::Paper,
                Shape::Paper => Shape::Scissors,
                Shape::Scissors => Shape::Rock,
            },
            Outcome::Tie => opponent,
            Outcome::Loss => match opponent {
                Shape::Rock => Shape::Scissors,
                Shape::Paper => Shape::Rock,
                Shape::Scissors => Shape::Paper,
            },
        };

        let game = Game::new(opponent, response);
        score += game.score();
    }

    println!("Part 2: {}", score);
}
