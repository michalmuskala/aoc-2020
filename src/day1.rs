use std::{cmp::Ordering, num::ParseIntError};

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn parse(input: &str) -> Result<Vec<u64>, ParseIntError> {
    let mut array: Vec<u64> = input.lines().map(|l| l.parse()).collect::<Result<_, _>>()?;
    array.sort_unstable();
    Ok(array)
}

#[aoc(day1, part1)]
fn part1(array: &[u64]) -> Option<u64> {
    match search(array, 0, 2020) {
        Some((i, j)) => Some(array[i] * array[j]),
        None => None,
    }
}

#[aoc(day1, part2)]
fn part2(array: &[u64]) -> Option<u64> {
    for (i, elem) in array.iter().enumerate() {
        if let Some((j, k)) = search(array, i + 1, 2020 - elem) {
            return Some(elem * array[j] * array[k]);
        }
    }

    None
}

fn search(array: &[u64], mut left: usize, target: u64) -> Option<(usize, usize)> {
    let mut right = array.len() - 1;
    while left < right {
        match (array[left] + array[right]).cmp(&target) {
            Ordering::Less => left += 1,
            Ordering::Greater => right -= 1,
            Ordering::Equal => return Some((left, right)),
        }
    }
    return None;
}
