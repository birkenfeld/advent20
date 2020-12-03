use advtools::input::iter_input;

fn main() {
    let directions = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let grid: Vec<Vec<char>> = iter_input()
        .map(|line: String| line.chars().collect())
        .collect();
    let nx = grid[0].len();

    let count = |&(right, down)| (0..grid.len())
        .step_by(down)
        .enumerate()
        .filter(|(ix, y)| grid[*y][(ix * right) % nx] == '#')
        .count();

    advtools::verify("Trees with 1/3", count(&(3, 1)), 257);

    let mult: usize = directions.iter().map(count).product();
    advtools::verify("Product of trees", mult, 1744787392);
}
