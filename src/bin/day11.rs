use advtools::grid::{Grid, Pos};
use advtools::input;

#[derive(Clone, PartialEq)]
enum Tile {
    Seat,
    Person,
    Floor,
}

const DIRS: &[(i32, i32)] = &[(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

fn run(mut grid: Grid<Tile>, dist_limit: i32, occ_limit: usize) -> usize {
    loop {
        let prev_grid = grid.clone();

        for pos in grid.positions::<i32>() {
            let count = || DIRS.iter().fold(0, |count, (dx, dy)| {
                for i in 1..=dist_limit {
                    match prev_grid.get(Pos(pos.x + i*dx, pos.y + i*dy)) {
                        None | Some(&Tile::Seat) => break,
                        Some(&Tile::Person) => return count + 1,
                        Some(&Tile::Floor) => {}
                    }
                }
                count
            });
            match prev_grid[pos] {
                Tile::Seat   if count() == 0         => grid[pos] = Tile::Person,
                Tile::Person if count() >= occ_limit => grid[pos] = Tile::Seat,
                _ => ()
            }
        }

        if grid == prev_grid {
            return grid.count(|t| t == &Tile::Person);
        }
    }
}

fn main() {
    let grid = Grid::new(input::lines().map(|line| {
        line.chars().map(|ch| if ch == 'L' { Tile::Seat } else { Tile::Floor }).collect()
    }));

    advtools::verify("Finally occupied, rule 1", run(grid.clone(), 1, 4), 2344);
    advtools::verify("Finally occupied, rule 2", run(grid, 1000, 5), 2076);
}
