use advtools::input;
use advtools::grid::Grid;

const DIRECTIONS: &[(usize, usize)] = &[(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

fn main() {
    let grid = Grid::new(input::lines().map(|line| line.chars().collect()));

    let count_trees = |&(right, down)| (0..).step_by(right)
        .zip((0..grid.height()).step_by(down))
        .filter(|&(x, y)| grid[(x % grid.width(), y)] == '#')
        .count();

    advtools::verify("Trees with 1/3", count_trees(&(3, 1)), 257);

    let prod: usize = DIRECTIONS.iter().map(count_trees).product();
    advtools::verify("Product of trees", prod, 1744787392);
}
