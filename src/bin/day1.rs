use itertools::Itertools;

const INPUT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/input/",
    env!("CARGO_BIN_NAME"),
    ".txt"
));

fn main() {
    let part1 = INPUT
        .lines()
        .map(str::parse::<i32>)
        .map(Result::unwrap)
        .tuple_combinations::<(_, _)>()
        .find(|(x, y)| x + y == 2020)
        .map(|(x, y)| x * y)
        .unwrap();

    println!("{:?}", part1);

    let part2 = INPUT
        .lines()
        .map(str::parse::<i32>)
        .map(Result::unwrap)
        .tuple_combinations::<(_, _, _)>()
        .find(|(x, y, z)| x + y + z == 2020)
        .map(|(x, y, z)| x * y * z)
        .unwrap();

    println!("{:?}", part2);
}
