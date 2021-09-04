use advtools::prelude::*;
use advtools::input::{iter_lines, to_u64};
use advtools::grid::{Grid, Pos, Dir, Dir::*};
use advtools::itertools::iproduct;

const MONSTER: &[(usize, usize)] = &[
    (18, 0),
    (0, 1), (5, 1), (6, 1), (11, 1), (12, 1), (17, 1), (18, 1), (19, 1),
    (1, 2), (4, 2), (7, 2), (10, 2), (13, 2), (16, 2),
];

// Coordinate transformations for all possible rotations/flips
type Trans = fn(usize, usize, usize) -> (usize, usize);
const TRANS: &[Trans] = &[
    |_, x, y| (x, y),
    |w, x, y| (w-x, y),
    |w, x, y| (x, w-y),
    |w, x, y| (w-x, w-y),
    |_, x, y| (y, x),
    |w, x, y| (y, w-x),
    |w, x, y| (w-y, x),
    |w, x, y| (w-y, w-x),
];

struct Tile {
    index: u64,
    grid: Grid<bool>,
    trans: Trans,
}

fn edges_match(tile1: &Tile, dir: Dir, tile2: &Tile, t2: Trans) -> bool {
    let w = tile1.grid.width() - 1;
    let t1 = tile1.trans;
    match dir {
        U => (0..=w).all(|x| tile1.grid[t1(w, x, 0)] == tile2.grid[t2(w, x, w)]),
        R => (0..=w).all(|y| tile1.grid[t1(w, w, y)] == tile2.grid[t2(w, 0, y)]),
        D => (0..=w).all(|x| tile1.grid[t1(w, x, w)] == tile2.grid[t2(w, x, 0)]),
        L => (0..=w).all(|y| tile1.grid[t1(w, 0, y)] == tile2.grid[t2(w, w, y)]),
    }
}

fn place_neighbors(tiles: &mut Vec<Tile>, grid: &mut Grid<Option<Tile>>, pos: Pos) {
    'directions:
    for dir in Dir::iter() {
        let new_pos = pos.to(dir);
        // If the grid in the given orientation is still empty, try to find some tile
        // which in some orientation fits the respective edge of cur_tile
        if let Some(None) = grid.get(new_pos) {
            let cur_tile = grid[pos].as_ref().unwrap();
            for (i, new_tile) in tiles.iter().enumerate() {
                for &trans in TRANS {
                    if edges_match(cur_tile, dir, new_tile, trans) {
                        let mut tile = tiles.remove(i);
                        tile.trans = trans;
                        grid[new_pos] = Some(tile);
                        place_neighbors(tiles, grid, new_pos);
                        continue 'directions;
                    }
                }
            }
        }
    }
}

fn main() {
    let mut tiles = Vec::new();
    let mut tile = vec![];
    let mut last_index = 0;
    for line in iter_lines() {
        if line.starts_with("Tile"){
            last_index = to_u64(line[5..].trim_matches(':'));
        } else {
            tile.push(line.chars().map(|ch| ch == '#').collect_vec());
            if tile.len() == tile[0].len() {
                tiles.push(Tile {
                    index: last_index,
                    grid: Grid::new(std::mem::take(&mut tile)),
                    trans: TRANS[0]  // this could be any of them
                });
            }
        }
    }
    let nt = (tiles.len() as f64 + 0.5).sqrt() as usize;
    let np = tiles[0].grid.width() - 2;
    let nf = nt*np;

    // Create a grid of tiles and place an arbitrary tile in the middle; the grid is
    // large enough so that the starting tile can be anywhere
    let mut grid = Grid::<Option<Tile>>::empty(2*nt+1, 2*nt+1);
    let middle = Pos(nt as i32, nt as i32);
    grid[middle] = Some(tiles.pop().unwrap());

    // Recursively place all neighbors with proper orientation
    place_neighbors(&mut tiles, &mut grid, middle);

    // Find edges (min x/y coordinates filled)
    let xmin = (0..=nt).find(|&x| grid[(x, nt)].is_some()).unwrap();
    let ymin = (0..=nt).find(|&y| grid[(nt, y)].is_some()).unwrap();
    let (xmax, ymax) = (xmin + nt-1, ymin + nt-1);

    // Calculate product of corner indices
    let corner_prod = [(xmin, ymin), (xmax, ymin), (xmin, ymax), (xmax, ymax)]
        .iter()
        .map(|&pos| grid[pos].as_ref().unwrap().index)
        .product::<u64>();
    advtools::verify("Corner product", corner_prod, 18449208814679u64);

    // Helper to get a pixel in the final stitched image
    let pixel = |(x, y)| {
        let tile = grid[(xmin + x/np, ymin + y/np)].as_ref().unwrap();
        tile.grid[(tile.trans)(np+1, 1 + x%np, 1 + y%np)]
    };

    // Search for monsters, we can reuse the same transformations
    let monsters = iproduct!(TRANS, 0..nf-20, 0..nf-3).filter(
        |(&t, x, y)| MONSTER.iter().all(|(dx, dy)| pixel(t(nf-1, x+dx, y+dy)))
    ).count();
    let total = iproduct!(0..nf, 0..nf).filter(|&pos| pixel(pos)).count();

    advtools::verify("Roughness", total - monsters*MONSTER.len(), 1559);
}
