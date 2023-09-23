use itertools::Itertools;
use nom::{combinator::all_consuming, Finish};
use parse::{GridPos, Instruction};

static EXAMPLE_INPUT: &str = r#"
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
"#;

mod parse;

fn main() {
    // let input = include_str!("input.txt");
    let input = EXAMPLE_INPUT;

    let instructions = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| all_consuming(Instruction::parse)(l).finish().unwrap().1);

    let mut head = GridPos { x: 0, y: 0 };
    let mut tail = GridPos { x: 0, y: 0 };

    let mut visited_by_tail = std::collections::HashSet::new();
    for ins in instructions.clone() {
        for _ in 0..ins.dist {
            head += ins.dir.delta();

            let diff = head - tail;
            match (diff.x, diff.y) {
                (2, 0) => tail += GridPos { x: 1, y: 0 },
                (-2, 0) => tail += GridPos { x: -1, y: 0 },
                (0, 2) => tail += GridPos { x: 0, y: 1 },
                (0, -2) => tail += GridPos { x: 0, y: -1 },
                (2, 1) => tail += GridPos { x: 1, y: 1 },
                (2, -1) => tail += GridPos { x: 1, y: -1 },
                (-2, 1) => tail += GridPos { x: -1, y: 1 },
                (-2, -1) => tail += GridPos { x: -1, y: -1 },
                (1, 2) => tail += GridPos { x: 1, y: 1 },
                (-1, 2) => tail += GridPos { x: -1, y: 1 },
                (1, -2) => tail += GridPos { x: 1, y: -1 },
                (-1, -2) => tail += GridPos { x: -1, y: -1 },
                _ => (),
            }

            visited_by_tail.insert(tail);
        }
    }

    println!("Part 1: {:?}", visited_by_tail.len());

    let mut knots = [GridPos { x: 0, y: 0 }; 10];
    let mut visited_by_tail = std::collections::HashSet::new();
    for ins in instructions {
        for _ in 0..ins.dist {
            knots[0] += ins.dir.delta();
            // follow the previous knot
            let mut prev_knot = knots[0];
            for (i, knot) in knots.iter_mut().skip(1).enumerate() {
                let diff = prev_knot - *knot;
                match (diff.x, diff.y) {
                    (2, 0) => *knot += GridPos { x: 1, y: 0 },
                    (-2, 0) => *knot += GridPos { x: -1, y: 0 },
                    (0, 2) => *knot += GridPos { x: 0, y: 1 },
                    (0, -2) => *knot += GridPos { x: 0, y: -1 },
                    (2, 1) => *knot += GridPos { x: 1, y: 1 },
                    (2, -1) => *knot += GridPos { x: 1, y: -1 },
                    (-2, 1) => *knot += GridPos { x: -1, y: 1 },
                    (-2, -1) => *knot += GridPos { x: -1, y: -1 },
                    (1, 2) => *knot += GridPos { x: 1, y: 1 },
                    (-1, 2) => *knot += GridPos { x: -1, y: 1 },
                    (1, -2) => *knot += GridPos { x: 1, y: -1 },
                    (-1, -2) => *knot += GridPos { x: -1, y: -1 },
                    _ => (),
                }

                prev_knot = *knot;

                if i == 9 {
                    visited_by_tail.insert(*knot);
                }
            }
        }

        println!("{:?}", knots);
    }

    println!("Part 2: {:?}", visited_by_tail.len());
}
