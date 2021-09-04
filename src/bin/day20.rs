use advtools::prelude::*;
use advtools::input::{iter_lines, to_u64};
use advtools::grid::{Grid, Dir::*};

const N: usize = 12;
const M: usize = 8;
const W: usize = N*M - 1;

const MONSTER: &[(usize, usize)] = &[
    (0, 18),
    (1, 0), (1, 5), (1, 6), (1, 11), (1, 12), (1, 17), (1, 18), (1, 19),
    (2, 1), (2, 4), (2, 7), (2, 10), (2, 13), (2, 16)
];

const COORDINATES: &[fn(usize, usize) -> (usize, usize)] = &[
    |x, y| (x, y),
    |x, y| (W-x, y),
    |x, y| (x, W-y),
    |x, y| (W-x, W-y),
    |x, y| (y, x),
    |x, y| (y, W-x),
    |x, y| (W-y, x),
    |x, y| (W-y, W-x),
];

fn edge_to_int(it: impl Iterator<Item=bool>) -> (u16, u16) {
    let mut fwd = 0;
    let mut rev = 0;
    for item in it {
        if item {
            fwd |= 1;
            rev |= 0x8000;
        }
        fwd <<= 1;
        rev >>= 1;
    }
    (fwd, rev >> 4)
}

fn edges(pattern: &[Vec<bool>]) -> (u16, u16, u16, u16, u16, u16, u16, u16) {
    let (t, rt) = edge_to_int(pattern[0].iter().cloned());
    let (r, rr) = edge_to_int(pattern.iter().map(|x| *x.last().unwrap()));
    let (b, rb) = edge_to_int(pattern.last().unwrap().iter().cloned());
    let (l, rl) = edge_to_int(pattern.iter().map(|x| x[0]));
    (t, rt, r, rr, b, rb, l, rl)
}

fn main() {
    let mut unoriented_tiles = HashMap::new();
    let mut tiles = HashMap::new();
    let mut tile_edges = HashMap::new();
    let mut by_edge: HashMap<_, Vec<_>> = HashMap::new();
    let mut tile = vec![];
    let mut last_index = 0;
    for line in iter_lines() {
        if line.starts_with("Tile"){
            last_index = to_u64(line[5..].trim_matches(':'));
        } else {
            tile.push(line.chars().map(|ch| ch == '#').collect_vec());
            if tile.len() == M+2 {
                unoriented_tiles.insert(last_index, std::mem::take(&mut tile));
            }
        }
    }

    let (t, _, r, _, b, _, l, _) = edges(&unoriented_tiles[&last_index]);
    tile_edges.insert(last_index, edges(&unoriented_tiles[&last_index]));
    by_edge.entry(t).or_default().push((last_index, U));
    by_edge.entry(r).or_default().push((last_index, R));
    by_edge.entry(b).or_default().push((last_index, D));
    by_edge.entry(l).or_default().push((last_index, L));
    tiles.insert(last_index, unoriented_tiles.remove(&last_index).unwrap());
    while !unoriented_tiles.is_empty() {
        for &index in unoriented_tiles.keys() {
            let i_edges = edges(&unoriented_tiles[&index]);
            let entry = if let Some(l) = by_edge.get(&i_edges.0) {
                (l[0].1, U, false)
            } else if let Some(l) = by_edge.get(&i_edges.1) {
                (l[0].1, U, true)
            } else if let Some(l) = by_edge.get(&i_edges.2) {
                (l[0].1, R, false)
            } else if let Some(l) = by_edge.get(&i_edges.3) {
                (l[0].1, R, true)
            } else if let Some(l) = by_edge.get(&i_edges.4) {
                (l[0].1, D, false)
            } else if let Some(l) = by_edge.get(&i_edges.5) {
                (l[0].1, D, true)
            } else if let Some(l) = by_edge.get(&i_edges.6) {
                (l[0].1, L, false)
            } else if let Some(l) = by_edge.get(&i_edges.7) {
                (l[0].1, L, true)
            } else {
                continue;
            };
            let mut pattern = unoriented_tiles.remove(&index).unwrap();
            pattern = match entry {
                (R, U, false) | (U, R, false) | (L, U, false) | (R, D, false) |
                (L, D, false) | (D, L, false) | (U, R, true) | (D, L, true) =>
                    (0..10).map(|y| (0..10).map(|x| pattern[9-x][y]).collect()).collect(),
                (U, U, false) | (D, D, false) | (U, U, true) | (R, R, true) |
                (D, D, true) | (L, L, true) | (R, L, true) | (L, R, true) =>
                    (0..10).map(|y| (0..10).map(|x| pattern[9-y][9-x]).collect()).collect(),
                (D, R, false) | (U, L, false) | (R, U, true) | (L, U, true) |
                (D, R, true) | (L, D, true) | (R, D, true) | (U, L, true) =>
                    (0..10).map(|y| (0..10).map(|x| pattern[x][9-y]).collect()).collect(),
                _ => pattern,
            };
            pattern = match entry {
                (U|R, U|R, false) | (D|L, D|L, false) | (D|L, U|R, true) | (U|R, D|L, true) =>
                    pattern.into_iter().map(|mut v| { v.reverse(); v }).collect(),
                _ => pattern,
            };
            let (t, _, r, _, b, _, l, _) = edges(&pattern);
            tile_edges.insert(index, edges(&pattern));
            tiles.insert(index, pattern);
            by_edge.entry(t).or_default().push((index, U));
            by_edge.entry(r).or_default().push((index, R));
            by_edge.entry(b).or_default().push((index, D));
            by_edge.entry(l).or_default().push((index, L));
            break;
        }
    }

    let mut corner_indices = vec![];
    let mut painted = 0;
    for &index in tiles.keys() {
        let (t, _, r, _, b, _, l, _) = tile_edges[&index];
        if by_edge[&t].len() == 1 && by_edge[&l].len() == 1 {
            painted = index;
        }
        if by_edge[&t].len() + by_edge[&r].len() + by_edge[&b].len() + by_edge[&l].len() == 6 {
            corner_indices.push(index);
        }
    }

    let corner_prod: u64 = corner_indices.iter().product();
    advtools::verify("Corner product", corner_prod, 18449208814679u64);

    let mut grid = Grid::from_iter(N*M, std::iter::repeat(false).take(N*N*M*M));
    let (mut y, mut x) = (0, 0);
    loop {
        let pattern = tiles.remove(&painted).unwrap();
        for px in 0..8 {
            for py in 0..8 {
                grid[(M*x + px, M*y + py)] = pattern[py + 1][px + 1];
            }
        }
        if tiles.is_empty() {
            break;
        }
        let edge = if y % 2 == 0 {
            if x == N-1 {
                y += 1;
                tile_edges[&painted].4
            } else {
                x += 1;
                tile_edges[&painted].2
            }
        } else {
            if x == 0 {
                y += 1;
                tile_edges[&painted].4
            } else {
                x -= 1;
                tile_edges[&painted].6
            }
        };
        painted = by_edge[&edge].iter().find(|x| tiles.contains_key(&x.0)).unwrap().0;
    }

    // Search for monsters
    for c in COORDINATES {
        for y in 0..=W-3 {
            for x in 0..=W-20 {
                if MONSTER.iter().all(|(dy, dx)| grid[c(x+dx, y+dy)]) {
                    MONSTER.iter().for_each(|(dy, dx)| grid[c(x+dx, y+dy)] = false);
                }
            }
        }
    }

    advtools::verify("Roughness", grid.count(|&x| x), 1559);
}
