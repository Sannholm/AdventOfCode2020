use itertools::Itertools;

const INPUT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/input/",
    env!("CARGO_BIN_NAME"),
    ".txt"
));

fn binary_walk(steps: &[bool], range: (u32, u32)) -> u32 {
    return steps
        .iter()
        .fold(range, |r, &s| {
            let half_range = (r.1 - r.0 + 1) / 2;
            if s {
                (r.0 + half_range, r.1)
            } else {
                (r.0, r.1 - half_range)
            }
        })
        .0;
}

fn main() {
    let seat_ids = INPUT
        .lines()
        .map(|l| {
            let (left, right) = l.split_at(7);
            let row = binary_walk(&left.chars().map(|c| c == 'B').collect_vec()[..], (0, 127));
            let col = binary_walk(&right.chars().map(|c| c == 'R').collect_vec()[..], (0, 7));
            row * 8 + col
        })
        .collect_vec();

    // Part 1
    let part1 = seat_ids.iter().max().unwrap();
    println!("{:?}", part1);

    // Part 2
    let part2 = (8..(126 * 8 + 7)).find(|id| {
        !seat_ids.contains(id) && seat_ids.contains(&(id - 1)) && seat_ids.contains(&(id + 1))
    });
    println!("{:?}", part2);
}
