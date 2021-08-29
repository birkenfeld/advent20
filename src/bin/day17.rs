use advtools::prelude::*;
use advtools::input::iter_lines;
use advtools::itertools::iproduct;
use std::ops::Range;

const N: isize = 18;

fn index(mut w: isize, mut z: isize, mut y: isize, mut x: isize) -> Option<usize> {
    w += N/2; z += N/2; y += N/2; x += N/2;
    (w >= 0 && w < N && z >= 0 && z < N && y >= 0 && y <= N && x >= 0 && x <= N)
        .then(|| (w * N*N*N + z * N*N + y * N + x) as usize)
}

fn process(grid: &mut [bool], diffs: &[(isize, isize, isize, isize)], wrange: Option<Range<isize>>) -> usize {
    for round in 0..6 {
        let mut flips = vec![];
        for w in wrange.clone().unwrap_or(-round-2..round+2) {
            for z in -N/2..N/2 {
                for y in -N/2..N/2 {
                    for x in -N/2..N/2 {
                        let nbs = diffs.iter()
                                       .filter_map(|&(dw, dz, dy, dx)| index(w+dw, z+dz, y+dy, x+dx))
                                       .filter(|&j| grid[j]).count();
                        let i = index(w, z, y, x).unwrap();
                        if (grid[i] && !(2..=3).contains(&nbs)) || (!grid[i] && nbs == 3) {
                            flips.push(i);
                        }
                    }
                }
            }
        }
        flips.into_iter().for_each(|i| grid[i] = !grid[i]);
    }
    grid.iter().filter(|&&x| x).count()
}

fn main() {
    let mut grid = vec![false; (N*N*N*N) as usize];
    for (y, line) in iter_lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '#' { grid[index(0, 0, y as isize-4, x as isize-4).unwrap()] = true; }
        }
    }
    let mut grid3 = grid.clone();

    let mut diffs = iproduct!(-1..=1, -1..=1, -1..=1, -1..=1).collect_vec();
    diffs.retain(|&x| x != (0, 0, 0, 0));

    advtools::verify("3-d grid", process(&mut grid3, &diffs, Some(0..1)), 301);
    advtools::verify("4-d grid", process(&mut grid, &diffs, None), 2424);
}
