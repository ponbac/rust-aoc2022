use std::collections::HashSet;

use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");

    part1(input);
    part2(input);
}

fn part1(input: &str) {
    let capital_start: usize = b'A'.into();
    let small_start: usize = b'a'.into();

    let mut sum: usize = 0;
    for line in input.lines().filter(|l| !l.is_empty()) {
        let first_half = &line[..line.len() / 2];
        for char in line[line.len() / 2..].chars() {
            if first_half.contains(char) {
                match char {
                    'a'..='z' => {
                        sum += char as usize - small_start + 1;
                    }
                    'A'..='Z' => {
                        sum += char as usize - capital_start + 27;
                    }
                    _ => {}
                }
                break;
            }
        }
    }

    println!("Part 1: {}", sum);
}

fn part2(input: &str) {
    let capital_start: usize = b'A'.into();
    let small_start: usize = b'a'.into();

    let mut sum: usize = 0;
    for mut chunk in input
        .lines()
        .filter(|l| !l.is_empty())
        .chunks(3)
        .into_iter()
    {
        let first_chars: HashSet<_> = chunk.next().unwrap().chars().collect();
        let second_chars: HashSet<_> = chunk.next().unwrap().chars().collect();

        for char in chunk.next().unwrap().chars() {
            if first_chars.contains(&char) && second_chars.contains(&char) {
                match char {
                    'a'..='z' => {
                        sum += char as usize - small_start + 1;
                    }
                    'A'..='Z' => {
                        sum += char as usize - capital_start + 27;
                    }
                    _ => {}
                }
                break;
            }
        }
    }

    println!("Part 2: {}", sum);
}
