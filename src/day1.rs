use std::cmp::Ordering;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn parse_input_day1(input: &str) -> Vec<u32> {
    let mut nums: Vec<u32> = input.lines().map(|l| l.parse().unwrap()).collect();
    nums.sort_unstable();
    nums
}

#[aoc(day1, part1)]
fn part1(nums: &[u32]) -> u32 {
    let mut left = 0;
    let mut right = nums.len() - 1;
    while left < right {
        match (nums[left] + nums[right]).cmp(&2020) {
            Ordering::Less => left += 1,
            Ordering::Greater => right -= 1,
            Ordering::Equal => return nums[left] * nums[right]
        }
    }

    return 0;
}

#[aoc(day1, part2)]
fn part2(nums: &[u32]) -> u32 {
    for (i, elem) in nums.iter().enumerate() {
        let mut left = i + 1;
        let mut right = nums.len() - 1;
        while left < right {
            match (nums[left] + nums[right]).cmp(&(2020 - elem)) {
                Ordering::Less => left += 1,
                Ordering::Greater => right -= 1,
                Ordering::Equal => return elem * nums[left] * nums[right]
            }
        }
    }


    return 0;
}
