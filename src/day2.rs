use aoc_runner_derive::{aoc, aoc_generator};
use lazy_static::lazy_static;
use regex::Regex;

struct Candidate{
    pos: (usize, usize),
    letter: char,
    string: String,
}

#[aoc_generator(day2)]
fn parse(input: &str) -> Vec<Candidate> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(\d+)-(\d+) (.): (.*)$").unwrap();
    }
    fn parse_candidate(line: &str) -> Candidate {
        let caps = RE.captures(line).unwrap();
        Candidate {
            pos: (caps.get(1).unwrap().as_str().parse().unwrap(), caps.get(2).unwrap().as_str().parse().unwrap()),
            letter: caps.get(3).unwrap().as_str().chars().next().unwrap(),
            string: caps.get(4).unwrap().as_str().to_owned(),
        }
    }
    input.lines().map(parse_candidate).collect()
}

#[aoc(day2, part1)]
fn part1(array: &[Candidate]) -> usize {
    fn is_valid(candidate: &Candidate) -> bool {
        let count = candidate.string.chars().filter(|&char| char == candidate.letter).count();
        (candidate.pos.0..=candidate.pos.1).contains(&count)
    }
    array.iter().filter(|&candidate| is_valid(candidate)).count()
}

#[aoc(day2, part2)]
fn part2(array: &[Candidate]) -> usize {
    fn is_valid(candidate: &Candidate) -> bool {
        (candidate.string.chars().nth(candidate.pos.0 - 1).unwrap() == candidate.letter) ^
            (candidate.string.chars().nth(candidate.pos.1 - 1).unwrap() == candidate.letter)
    }
    array.iter().filter(|&candidate| is_valid(candidate)).count()
}
