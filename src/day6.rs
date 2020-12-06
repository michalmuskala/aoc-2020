use aoc_runner_derive::{aoc, aoc_generator};
use bit_set::BitSet;

#[aoc_generator(day6)]
fn parse(input: &str) -> Vec<Vec<BitSet>> {
    input
        .split("\n\n")
        .map(|str| {
            str.lines()
                .map(|line| line.chars().map(|c| c as usize - 'a' as usize).collect())
                .collect()
        })
        .collect()
}

#[aoc(day6, part1)]
fn part1(sets: &[Vec<BitSet>]) -> usize {
    sets.into_iter()
        .map(|sets| {
            sets.into_iter()
                .fold(BitSet::new(), |mut set, other| {
                    set.union_with(other);
                    set
                })
                .len()
        })
        .sum()
}

#[aoc(day6, part2)]
fn part2(sets: &[Vec<BitSet>]) -> usize {
    sets.into_iter()
        .map(|sets| {
            sets.into_iter()
                .enumerate()
                .fold(BitSet::new(), |mut set, (idx, other)| {
                    if idx == 0 {
                        // for first iteration, otherwise result would be empty
                        set.union_with(other);
                    } else {
                        set.intersect_with(other);
                    }
                    set
                })
                .len()
        })
        .sum()
}
