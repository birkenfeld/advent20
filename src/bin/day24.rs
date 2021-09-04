use advtools::prelude::*;
use advtools::input::iter_lines;
use advtools::grid::{Grid, Pos};

const N: usize = 140;

fn main() {
    let mut tiles = Grid::<bool>::empty(N, N);
    for line in iter_lines() {
        let mut pos = tiles.center::<i32>();
        let mut chars = line.chars();
        while let Some(ch) = chars.next() {
            match ch {
                'e' => pos.step_right(),
                'w' => pos.step_left(),
                's' => {
                    pos.step_up();
                    if chars.next() == Some('e') { pos.step_right() }
                }
                'n' => {
                    pos.step_down();
                    if chars.next() == Some('w') { pos.step_left() }
                }
                _ => unreachable!()
            }
        }
        tiles[pos] ^= true;
    }
    advtools::verify("Black tiles at start", tiles.count(|x| *x), 360);

    for _ in 0..100 {
        let flips = iproduct!(1..N as i32-1, 1..N as i32-1).filter_map(|(x, y)| {
            let black = tiles[Pos(x, y)];
            let neighbors = [(1, 0), (-1, 0), (0, 1), (0, -1), (-1, 1), (1, -1)]
                .iter().filter(|(dx, dy)| tiles[Pos(x + dx, y + dy)]).count();
            ((black && (neighbors == 0 || neighbors > 2)) || (!black && neighbors == 2))
                .then(|| Pos(x, y))
        }).collect_vec();
        flips.into_iter().for_each(|pos| tiles[pos] ^= true);
    }
    advtools::verify("Black tiles after 100 days", tiles.count(|x| *x), 3924);
}
