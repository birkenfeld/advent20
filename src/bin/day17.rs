use advtools::prelude::*;
use advtools::input::iter_lines;
use advtools::itertools::iproduct;

const NXY: usize = 21;
const NZW: usize = 13;

fn process(grid: &mut Vec<Vec<Vec<Vec<bool>>>>, diffs: &[(isize, isize, isize, isize)]) -> usize {
    for _ in 0..6 {
        let mut flips = vec![];
        for (w, dim) in grid.iter().enumerate() {
            for (z, plane) in dim.iter().enumerate() {
                for (y, row) in plane.iter().enumerate() {
                    for (x, cell) in row.iter().enumerate() {
                        let mut count = 0;
                        for &(dw, dz, dy, dx) in diffs {
                            let nw = (w as isize) + dw;
                            if nw < 0 || nw >= grid.len() as isize { continue; }
                            let nz = (z as isize) + dz;
                            if nz < 0 || nz >= NZW as isize { continue; }
                            let ny = (y as isize) + dy;
                            if ny < 0 || ny >= NXY as isize { continue; }
                            let nx = (x as isize) + dx;
                            if nx < 0 || nx >= NXY as isize { continue; }
                            count += grid[nw as usize][nz as usize][ny as usize][nx as usize] as u32;
                        }
                        if *cell {
                            if count < 2 || count > 3 { flips.push((w, z, y, x)); }
                        } else {
                            if count == 3 { flips.push((w, z, y, x)); }
                        }
                    }
                }
            }
        }
        for (w, z, y, x) in flips {
            grid[w][z][y][x] = !grid[w][z][y][x];
        }
    }
    grid.iter().flatten().flatten().flatten().filter(|x| **x).count()
}

fn main() {
    let mut grid = vec![vec![vec![vec![false; NXY]; NXY]; NZW]; NZW];
    for (y, line) in iter_lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '#' { grid[NZW/2][NZW/2][y+NXY/2-4][x+NXY/2-4] = true; }
        }
    }
    let mut diffs = iproduct!(-1..=1, -1..=1, -1..=1, -1..=1).collect_vec();
    diffs.retain(|&x| x != (0, 0, 0, 0));

    let mut grid3 = vec![grid[NZW/2].clone()];

    advtools::verify("3-d grid", process(&mut grid3, &diffs), 301);
    advtools::verify("4-d grid", process(&mut grid, &diffs), 2424);
}
