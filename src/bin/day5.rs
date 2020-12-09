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

// Original
fn find_seat_ids() -> Vec<u32> {
    INPUT
        .lines()
        .map(|l| {
            let row = binary_walk(
                l[..7].chars().map(|c| c == 'B').collect_vec().as_slice(),
                (0, 127),
            );
            let col = binary_walk(
                l[7..].chars().map(|c| c == 'R').collect_vec().as_slice(),
                (0, 7),
            );
            row * 8 + col
        })
        .collect_vec()
}

// Simpler alternative (inspired by @guspihl and @Caagr98)
fn find_seat_ids_alternative() -> Vec<u32> {
    INPUT
        .replace("F", "0")
        .replace("B", "1")
        .replace("L", "0")
        .replace("R", "1")
        .lines()
        .map(|l| {
            let row = u32::from_str_radix(&l[..7], 2).unwrap();
            let col = u32::from_str_radix(&l[7..], 2).unwrap();
            row * 8 + col
        })
        .collect_vec()
}

fn main() {
    assert_eq!(find_seat_ids(), find_seat_ids_alternative());

    let seat_ids = find_seat_ids_alternative();

    // Part 1
    let part1 = seat_ids.iter().max();
    println!("{:?}", part1);

    // Part 2
    let part2 = (8..(126 * 8 + 7)).find(|id| {
        !seat_ids.contains(id) && seat_ids.contains(&(id - 1)) && seat_ids.contains(&(id + 1))
    });
    println!("{:?}", part2);
}
