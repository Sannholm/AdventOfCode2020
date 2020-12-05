use itertools::Itertools;
use regex::Regex;
use std::{collections::HashMap, unreachable};

const INPUT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/input/",
    env!("CARGO_BIN_NAME"),
    ".txt"
));

fn parse_input() -> impl Iterator<Item = HashMap<&'static str, &'static str>> {
    INPUT.split_terminator("\r\n\r\n").map(|pp| {
        pp.split_whitespace()
            .map(|f| f.split(':').collect_tuple().unwrap())
            .collect()
    })
}

fn main() {
    // Part 1
    let passport_has_required_fields = |pp: &HashMap<_, _>| {
        ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
            .iter()
            .all(|f| pp.contains_key(f))
    };

    let part1 = parse_input().filter(passport_has_required_fields).count();
    println!("{:?}", part1);

    // Part 2
    let passport_has_valid_values = |pp: &HashMap<&str, &str>| {
        pp.iter().all(|(&f, &v)| {
            Regex::new(match f {
                "byr" => "^(19[2-8][0-9]|199[0-9]|200[0-2])$",
                "iyr" => "^(201[0-9]|2020)$",
                "eyr" => "^(202[0-9]|2030)$",
                "hgt" => "^(1[5-8][0-9]|19[0-3])cm|(59|6[0-9]|7[0-6])in$",
                "hcl" => "^#[0-9a-f]{6}$",
                "ecl" => "^amb|blu|brn|gry|grn|hzl|oth$",
                "pid" => "^[0-9]{9}$",
                "cid" => ".*",
                _ => unreachable!(),
            })
            .unwrap()
            .is_match(v)
        })
    };
    let part2 = parse_input()
        .filter(passport_has_required_fields)
        .filter(passport_has_valid_values)
        .count();
    println!("{:?}", part2);
}
