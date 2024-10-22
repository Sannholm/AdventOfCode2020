const INPUT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/input/",
    env!("CARGO_BIN_NAME"),
    ".txt"
));

fn is_tree(x: usize, y: usize) -> Option<bool> {
    let width = INPUT.lines().next().unwrap().chars().count();
    INPUT
        .lines()
        .nth(y)? // Return None if out of bounds on y-axis
        .chars()
        .nth(x % width) // Repeat horizontally
        .map(|c| c == '#')
}

fn count_trees(slope: (usize, usize)) -> usize {
    let mut x = 0;
    let mut y = 0;
    let mut num_trees = 0;

    while let Some(is_tree) = is_tree(x, y) {
        if is_tree {
            num_trees += 1
        }
        x += slope.0;
        y += slope.1;
    }

    num_trees
}

fn main() {
    let part1 = count_trees((3, 1));
    println!("{:?}", part1);

    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let part2 = slopes.iter().map(|&s| count_trees(s)).product::<usize>();
    println!("{:?}", part2);
}
