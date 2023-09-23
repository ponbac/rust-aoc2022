use std::{
    collections::HashMap,
    fmt::{self, Debug, Formatter},
};

#[derive(Debug, Hash, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct TreeMap(Vec<Vec<u32>>);

impl Debug for TreeMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for row in &self.0 {
            for col in row {
                write!(f, "{}", col)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl TreeMap {
    const DIRECTIONS: [Direction; 4] = [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ];

    fn get(&self, x: usize, y: usize) -> Option<u32> {
        self.0.get(y).and_then(|row| row.get(x).copied())
    }

    fn walk(&self, x: usize, y: usize, direction: &Direction) -> Option<u32> {
        match direction {
            Direction::Up if y > 0 => self.get(x, y - 1),
            Direction::Down => self.get(x, y + 1),
            Direction::Left if x > 0 => self.get(x - 1, y),
            Direction::Right => self.get(x + 1, y),
            _ => None,
        }
    }

    fn walkable(&self, x: usize, y: usize, direction: &Direction) -> bool {
        let base_tree = self.get(x, y).unwrap();

        let mut x = x;
        let mut y = y;
        while let Some(tree) = self.walk(x, y, direction) {
            if tree >= base_tree {
                return false;
            }

            match direction {
                Direction::Up => y -= 1,
                Direction::Down => y += 1,
                Direction::Left => x -= 1,
                Direction::Right => x += 1,
            }
        }

        true
    }

    fn visible_from(&self, x: usize, y: usize) -> HashMap<&Direction, i32> {
        let base_tree = self.get(x, y).unwrap();

        // init with 0 for each direction
        let mut visible = Self::DIRECTIONS
            .iter()
            .map(|direction| (direction, 0))
            .collect::<HashMap<_, _>>();

        for direction in &Self::DIRECTIONS {
            let mut x = x;
            let mut y = y;

            while let Some(tree) = self.walk(x, y, direction) {
                *visible.get_mut(direction).unwrap() += 1;

                if tree >= base_tree {
                    break;
                }

                match direction {
                    Direction::Up => y -= 1,
                    Direction::Down => y += 1,
                    Direction::Left => x -= 1,
                    Direction::Right => x += 1,
                }
            }
        }

        visible
    }

    fn is_visible(&self, x: usize, y: usize) -> bool {
        self.walkable(x, y, &Direction::Up)
            || self.walkable(x, y, &Direction::Down)
            || self.walkable(x, y, &Direction::Left)
            || self.walkable(x, y, &Direction::Right)
    }

    fn scenic_score(&self, x: usize, y: usize) -> i32 {
        let visible = self.visible_from(x, y);

        visible.values().product::<i32>()
    }
}

fn main() {
    let input = include_str!("input.txt");

    let tree_map = TreeMap(
        input
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| {
                line.chars()
                    .map(|c| char::to_digit(c, 10).unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>(),
    );

    println!("TREES: \n{:?}", tree_map);

    let walkable_trees = tree_map
        .0
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(x, _)| tree_map.is_visible(*x, y))
                .count()
        })
        .sum::<usize>();

    let max_scenic_score = tree_map
        .0
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, _)| tree_map.scenic_score(x, y))
                .max()
                .unwrap()
        })
        .max()
        .unwrap();

    println!("Part 1: {}", walkable_trees);
    println!("Part 2: {}", max_scenic_score);
}
