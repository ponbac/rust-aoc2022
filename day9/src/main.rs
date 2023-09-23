use nom::{combinator::all_consuming, Finish};
use parse::Instruction;

static EXAMPLE_INPUT: &str = r#"
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
"#;

mod parse;

fn main() {
    let input = include_str!("input.txt");
    let input = EXAMPLE_INPUT;

    let instructions = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| all_consuming(Instruction::parse)(l).finish().unwrap().1);

    for ins in instructions {
        println!("{ins:?}");
    }
}
