use itertools::Itertools;
use regex::*;

const INPUT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/input/",
    env!("CARGO_BIN_NAME"),
    ".txt"
));

fn main() {
    let pattern = Regex::new(r"(\d+)-(\d+) ([a-z]): ([a-z]+)").unwrap();

    let parse_input = || {
        INPUT.lines().map(|row| {
            pattern
                .captures(row)
                .unwrap()
                .iter()
                .skip(1)
                .map(|x| x.unwrap().as_str())
                .collect_tuple::<(_, _, _, _)>()
                .unwrap()
        })
    };

    let count_valid_passwords = |is_valid_pass: &dyn Fn(&str, usize, usize, char) -> bool| {
        parse_input()
            .filter(|&(num1, num2, letter, pass)| {
                is_valid_pass(
                    pass,
                    num1.parse().unwrap(),
                    num2.parse().unwrap(),
                    letter.chars().next().unwrap(),
                )
            })
            .count()
    };

    let part1 = count_valid_passwords(&|pass, min, max, letter| {
        let occ = pass.chars().filter(|&c| c == letter).count();
        occ >= min && occ <= max
    });
    println!("{:?}", part1);

    let part2 = count_valid_passwords(&|pass, pos1, pos2, letter| {
        (pass.chars().nth(pos1 - 1).unwrap() == letter)
            ^ (pass.chars().nth(pos2 - 1).unwrap() == letter)
    });
    println!("{:?}", part2);
}
