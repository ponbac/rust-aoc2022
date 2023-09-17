use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", packet_start(input, 4));
    println!("Part 2: {}", packet_start(input, 14));
}

fn packet_start(signal: &str, n: usize) -> usize {
    signal
        .as_bytes()
        .windows(n)
        .position(|w| HashSet::<_>::from_iter(w).len() == n)
        .map(|i| i + n)
        .unwrap()
}
