use advtools::grid::{Pos, Dir};
use advtools::input::{iter_input, to_i32};

fn main() {
    let mut pos = Pos(0i32, 0);
    let mut dir = Dir::R;

    for line in iter_input::<String>() {
        match (&line[..1], to_i32(&line[1..])) {
            ("F", n) => for _ in 0..n { pos.step(dir); }
            ("N", n) => for _ in 0..n { pos.step_up(); }
            ("S", n) => for _ in 0..n { pos.step_down(); }
            ("E", n) => for _ in 0..n { pos.step_right(); }
            ("W", n) => for _ in 0..n { pos.step_left(); }
            ("L", 90) | ("R", 270) => dir = dir.left(),
            ("R", 90) | ("L", 270) => dir = dir.right(),
            ("R", 180) | ("L", 180) => dir = dir.flip(),
            _ => unreachable!()
        }
    }

    advtools::verify("End position", pos.manhattan(), 1457);

    let mut wpos = Pos(10i32, -1);
    let mut pos = Pos(0i32, 0);

    for line in iter_input::<String>() {
        match (&line[..1], to_i32(&line[1..])) {
            ("F", n) => pos += wpos * n,
            ("N", n) => for _ in 0..n { wpos.step_up(); }
            ("S", n) => for _ in 0..n { wpos.step_down(); }
            ("E", n) => for _ in 0..n { wpos.step_right(); }
            ("W", n) => for _ in 0..n { wpos.step_left(); }
            ("L", 90) | ("R", 270) => wpos = Pos(wpos.y, -wpos.x),
            ("R", 90) | ("L", 270) => wpos = Pos(-wpos.y, wpos.x),
            ("R", 180) | ("L", 180) => wpos = Pos(-wpos.x, -wpos.y),
            _ => unreachable!()
        }
    }

    advtools::verify("With waypoint", pos.manhattan(), 106860);
}
