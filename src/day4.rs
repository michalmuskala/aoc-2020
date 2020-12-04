use aoc_runner_derive::{aoc, aoc_generator};

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

#[aoc_generator(day4)]
fn parse(input: &str) -> Vec<Passport> {
    fn parse_passport(input: &str) -> Passport {
        let mut byr = None;
        let mut iyr = None;
        let mut eyr = None;
        let mut hgt = None;
        let mut hcl = None;
        let mut ecl = None;
        let mut pid = None;
        let mut cid = None;

        for (key, value) in input
            .split_ascii_whitespace()
            .filter_map(|kv| split_once(kv, ":"))
        {
            match key {
                "byr" => byr = Some(value.to_string()),
                "iyr" => iyr = Some(value.to_string()),
                "eyr" => eyr = Some(value.to_string()),
                "hgt" => hgt = Some(value.to_string()),
                "hcl" => hcl = Some(value.to_string()),
                "ecl" => ecl = Some(value.to_string()),
                "pid" => pid = Some(value.to_string()),
                "cid" => cid = Some(value.to_string()),
                _ => continue,
            }
        }

        Passport {
            byr,
            iyr,
            eyr,
            hgt,
            hcl,
            ecl,
            pid,
            cid,
        }
    }
    input.split("\n\n").map(parse_passport).collect()
}

#[aoc(day4, part1)]
fn part1(passports: &[Passport]) -> usize {
    fn is_valid(passport: &Passport) -> bool {
        passport.byr.is_some()
            && passport.iyr.is_some()
            && passport.eyr.is_some()
            && passport.hgt.is_some()
            && passport.hcl.is_some()
            && passport.ecl.is_some()
            && passport.pid.is_some()
    }
    passports
        .iter()
        .filter(|&passport| is_valid(passport))
        .count()
}

#[aoc(day4, part2)]
fn part2(passports: &[Passport]) -> usize {
    fn validate(passport: &Passport) -> Option<()> {
        lazy_static! {
            static ref HCL: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
            static ref ECL: Regex = Regex::new(r"^amb|blu|brn|gry|grn|hzl|oth$").unwrap();
            static ref PID: Regex = Regex::new(r"^\d{9}$").unwrap();
        }
        let byr: u64 = passport.byr.as_ref()?.parse().ok()?;
        if byr < 1920 || byr > 2002 {
            return None;
        }

        let iyr: u64 = passport.iyr.as_ref()?.parse().ok()?;
        if iyr < 2010 || iyr > 2020 {
            return None;
        }

        let eyr: u64 = passport.eyr.as_ref()?.parse().ok()?;
        if eyr < 2020 || eyr > 2030 {
            return None;
        }

        let hgt = passport.hgt.as_ref()?;
        let (value, unit) = hgt.split_at(hgt.len() - 2);
        let value: u64 = value.parse().ok()?;
        let range = match unit {
            "cm" => 150..=193,
            "in" => 59..=76,
            _ => return None,
        };
        if !range.contains(&value) {
            return None;
        }

        if !HCL.is_match(passport.hcl.as_ref()?) {
            return None;
        }

        if !ECL.is_match(passport.ecl.as_ref()?) {
            return None;
        }

        if !PID.is_match(passport.pid.as_ref()?) {
            return None;
        }

        Some(())
    }
    passports
        .iter()
        .filter_map(|passport| validate(passport))
        .count()
}

fn split_once<'a>(str: &'a str, pat: &str) -> Option<(&'a str, &'a str)> {
    let mut iter = str.split(pat);
    Some((iter.next()?, iter.next()?))
}
