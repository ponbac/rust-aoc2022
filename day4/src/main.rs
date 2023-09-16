use std::ops::RangeInclusive;

use itertools::Itertools;

trait InclusiveRangeExt {
    fn contains_range(&self, other: &Self) -> bool;

    fn contains_or_is_contained_by(&self, other: &Self) -> bool {
        self.contains_range(other) || other.contains_range(self)
    }

    fn overlaps(&self, other: &Self) -> bool;

    fn overlaps_or_is_overlapped_by(&self, other: &Self) -> bool {
        self.overlaps(other) || other.overlaps(self)
    }
}

impl<T> InclusiveRangeExt for RangeInclusive<T>
where
    T: PartialOrd,
{
    fn contains_range(&self, other: &Self) -> bool {
        self.contains(other.start()) && self.contains(other.end())
    }

    fn overlaps(&self, other: &Self) -> bool {
        self.contains(other.start()) || self.contains(other.end())
    }
}

fn main() {
    let input = include_str!("input.txt");

    part1(input);
    part2(input);
}

fn part1(input: &str) {
    let count = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(to_range_tuple)
        .filter(|(range1, range2)| range1.contains_or_is_contained_by(range2))
        .count();

    println!("Part 1: {}", count);
}

fn part2(input: &str) {
    let count = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(to_range_tuple)
        .filter(|(range1, range2)| range1.overlaps_or_is_overlapped_by(range2))
        .count();

    println!("Part 2: {}", count);
}

fn to_range_tuple(text: &str) -> (RangeInclusive<u32>, RangeInclusive<u32>) {
    text.split(',')
        .map(|range| {
            range
                .split('-')
                .map(|n| n.parse().unwrap())
                .collect_tuple()
                .map(|(start, end)| start..=end)
                .unwrap()
        })
        .collect_tuple()
        .unwrap()
}
