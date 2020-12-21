use regex::Regex;
use std::collections::{HashMap, HashSet};

use itertools::Itertools;

const INPUT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/input/",
    env!("CARGO_BIN_NAME"),
    ".txt"
));

fn find_dependants(dependencies: &HashSet<&str>) -> HashSet<&'static str> {
    INPUT
        .lines()
        .filter_map(|l| {
            let (dependant, line_dependencies) = l.split(" bags contain ").next_tuple().unwrap();
            if dependencies.iter().any(|d| line_dependencies.contains(d)) {
                Some(dependant)
            } else {
                None
            }
        })
        .collect()
}

fn num_bags_inside(bags: &HashMap<&str, HashMap<String, usize>>, name: &str) -> usize {
    return bags[name]
        .iter()
        .map(|(bag, count)| count + num_bags_inside(&bags, bag) * count)
        .sum();
}

fn main() {
    // Part 1
    let mut part1: HashSet<_> = HashSet::new();
    part1.insert("shiny gold");
    loop {
        let new_dependants = find_dependants(&part1);
        if new_dependants.iter().all(|d| part1.contains(d)) {
            break;
        }
        part1.extend(new_dependants);
    }
    println!("{:?}", part1.len() - 1);

    // Part 2
    let pattern = Regex::new(r"(\d+) (.*?) bags?").unwrap();
    let bags: HashMap<&str, HashMap<String, usize>> = INPUT
        .lines()
        .map(|l| {
            let (dependant, line_dependencies) = l.split(" bags contain ").next_tuple().unwrap();
            (
                dependant,
                pattern
                    .captures_iter(line_dependencies)
                    .map(|m| (m[2].to_string(), m[1].parse().unwrap()))
                    .collect(),
            )
        })
        .collect();
    let part2 = num_bags_inside(&bags, "shiny gold");
    println!("{:?}", part2);
}
