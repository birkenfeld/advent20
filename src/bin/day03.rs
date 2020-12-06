use advtools::input::iter_input;

fn main() {
    let directions = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let grid: Vec<Vec<char>> = iter_input()
        .map(|line: String| line.chars().collect())
        .collect();
    let nx = grid[0].len();

    let count_trees = |&(right, down)| (0..grid.len()).step_by(down)
        .zip((0..).step_by(right))
        .filter(|&(y, x)| grid[y][x % nx] == '#')
        .count();

    advtools::verify("Trees with 1/3", count_trees(&(3, 1)), 257);

    let prod: usize = directions.iter().map(count_trees).product();
    advtools::verify("Product of trees", prod, 1744787392);
}
