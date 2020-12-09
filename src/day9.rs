use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day9)]
fn parse(input: &str) -> Vec<u64> {
    input
        .lines()
        .map(|line| line.parse().ok())
        .flatten()
        .collect()
}

#[aoc(day9, part1)]
fn part1(instrs: &[u64]) -> Option<u64> {
    let mut set = HashSet::new();

    for i in 0..25 {
        set.insert(instrs[i]);
    }

    for i in 25..instrs.len() {
        if !set.iter().any(|num| set.contains(&(instrs[i] - num))) {
            return Some(instrs[i]);
        } else {
            set.remove(&instrs[i - 25]);
            set.insert(instrs[i]);
        }
    }

    None
}

#[aoc(day9, part2)]
fn part2(instrs: &[u64]) -> Option<u64> {
    let target = part1(instrs)?;

    let mut low = 0;
    let mut current = 0;

    for high in 0..instrs.len() {
        current += instrs[high];
        while low < high && current > target {
            current -= instrs[low];
            low += 1;
        }

        if current == target {
            let slice = &instrs[low..high];
            return Some(slice.iter().min().unwrap() + slice.iter().max().unwrap());
        }
    }

    None
}
