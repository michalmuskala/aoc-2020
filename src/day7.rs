use std::collections::{HashMap, HashSet, VecDeque};

use aoc_runner_derive::{aoc, aoc_generator};
use lazy_static::lazy_static;
use regex::Regex;

#[aoc_generator(day7)]
fn parse(input: &str) -> HashMap<String, HashMap<String, u64>> {
    fn parse_rule(line: &str) -> (String, HashMap<String, u64>) {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(?P<num>\d+) (?P<colour>[\w ]+) bags?").unwrap();
        }
        let (parent, contains) = split_once(line, " bags contain ").unwrap();
        let contained = match contains {
            "no other bags." => HashMap::new(),
            list => RE
                .captures_iter(list)
                .map(|cap| (cap["colour"].to_string(), cap["num"].parse().unwrap()))
                .collect(),
        };
        (parent.to_string(), contained)
    }
    input.lines().map(parse_rule).collect()
}

#[aoc(day7, part1)]
fn part1(rules: &HashMap<String, HashMap<String, u64>>) -> usize {
    let transposed: HashMap<String, HashSet<String>> = rules
        .into_iter()
        .flat_map(|(parent, contained)| {
            contained
                .keys()
                .map(move |contained| (contained.clone(), parent.clone()))
        })
        .fold(HashMap::new(), |mut map, (contained, parent)| {
            map.entry(contained)
                .or_insert(HashSet::new())
                .insert(parent);
            map
        });

    let mut todo = VecDeque::new();
    let mut seen = HashSet::new();
    todo.extend(transposed.get("shiny gold").unwrap());

    while todo.len() != 0 {
        if let Some(next) = todo.pop_front() {
            if !seen.contains(next) {
                todo.extend(transposed.get(next).into_iter().flatten());
                seen.insert(next.to_string());
            }
        }
    }

    seen.len()
}

#[aoc(day7, part2)]
fn part2(rules: &HashMap<String, HashMap<String, u64>>) -> u64 {
    fn count(rule: &HashMap<String, u64>, rules: &HashMap<String, HashMap<String, u64>>) -> u64 {
        let nested_count: u64 = rule
            .iter()
            .map(|(bag, times)| times * count(rules.get(bag).unwrap(), rules))
            .sum();
        nested_count + 1
    }
    count(rules.get("shiny gold").unwrap(), rules) - 1
}

fn split_once<'a>(str: &'a str, pat: &str) -> Option<(&'a str, &'a str)> {
    let mut iter = str.split(pat);
    Some((iter.next()?, iter.next()?))
}
