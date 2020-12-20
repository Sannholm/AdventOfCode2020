use std::collections::HashSet;

use itertools::Itertools;

const INPUT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/input/",
    env!("CARGO_BIN_NAME"),
    ".txt"
));

fn main() {
    // Part 1
    let part1: usize = INPUT
        .split_terminator("\r\n\r\n")
        .map(|g| {
            g.chars()
                .filter(|&c| c != '\n' && c != '\r')
                .collect::<HashSet<_>>()
                .iter()
                .count()
        })
        .sum();
    println!("{:?}", part1);

    // Part 2
    let part2: usize = INPUT
        .split_terminator("\r\n\r\n")
        .map(|g| {
            let sets = g
                .lines()
                .map(|l| l.chars().collect::<HashSet<_>>())
                .collect_vec();
            sets[0]
                .iter()
                .filter(|c| sets[1..].iter().all(|s| s.contains(c)))
                .count()
        })
        .sum();
    println!("{:?}", part2);
}
