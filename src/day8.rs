use std::fmt;

use aoc_runner_derive::{aoc, aoc_generator};
use bit_set::BitSet;

#[derive(Debug, Clone)]
enum Instr {
    Nop(isize),
    Acc(i64),
    Jmp(isize),
}

impl Instr {
    fn flip(&mut self) {
        match self {
            Instr::Nop(jmp) => *self = Instr::Jmp(*jmp),
            Instr::Jmp(jmp) => *self = Instr::Nop(*jmp),
            Instr::Acc(_) => (),
        }
    }
}

#[aoc_generator(day8)]
fn parse(input: &str) -> Vec<Instr> {
    fn parse_line(line: &str) -> Option<Instr> {
        let (instr, num) = split_once(line, " ")?;
        match instr {
            "nop" => Some(Instr::Nop(num.parse().ok()?)),
            "acc" => Some(Instr::Acc(num.parse().ok()?)),
            "jmp" => Some(Instr::Jmp(num.parse().ok()?)),
            _ => None,
        }
    }
    input.lines().map(parse_line).flatten().collect()
}

#[derive(Debug, Eq, PartialEq)]
enum Outcome {
    Loops(i64),
    Terminates(i64),
}

impl fmt::Display for Outcome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Outcome::Loops(result) => write!(f, "{}", result),
            Outcome::Terminates(result) => write!(f, "{}", result),
        }
    }
}

#[aoc(day8, part1)]
fn part1(instrs: &[Instr]) -> Outcome {
    let mut ic = 0;
    let mut acc = 0;
    let mut visited = BitSet::new();

    loop {
        visited.insert(ic);
        match instrs[ic] {
            Instr::Nop(_) => ic += 1,
            Instr::Acc(value) => {
                acc += value;
                ic += 1;
            }
            Instr::Jmp(jmp) => {
                ic = (ic as isize + jmp) as usize;
            }
        }

        if visited.contains(ic) {
            return Outcome::Loops(acc)
        }
        if ic == instrs.len() {
            return Outcome::Terminates(acc)
        }
    }
}

#[aoc(day8, part2)]
fn part2(instrs: &[Instr]) -> i64 {
    let mut candidate = instrs.to_vec();

    for i in 0..candidate.len() {
        candidate[i].flip();
        match part1(&candidate) {
            Outcome::Loops(_) => candidate[i].flip(),
            Outcome::Terminates(result) => return result,
        }
    }

    0
}


fn split_once<'a>(str: &'a str, pat: &str) -> Option<(&'a str, &'a str)> {
    let mut iter = str.split(pat);
    Some((iter.next()?, iter.next()?))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

        let instrs = parse(input);

        assert_eq!(part1(&instrs), Outcome::Loops(5));

        assert_eq!(part2(&instrs), 8)
    }
}
