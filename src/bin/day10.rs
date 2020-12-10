use advtools::prelude::*;
use advtools::input::iter_input;
use std::iter::once;

// number of arrangements for each run of 1-jolt differences
const SINGLE_ARRANGE: [u64; 5] = [1, 1, 2, 4, 7];

fn main() {
    let mut jolts: Vec<u32> = once(0).chain(iter_input().sorted()).collect();
    jolts.push(jolts[jolts.len() - 1] + 3);

    let mut diffs = (0, 0);
    let mut run_1jolt = 0;
    let mut arranges = 1;
    for pair in jolts.windows(2) {
        if pair[1] - pair[0] == 1 {
            diffs.0 += 1;
            run_1jolt += 1;
        } else {
            diffs.1 += 1;
            arranges *= SINGLE_ARRANGE[run_1jolt];
            run_1jolt = 0;
        }
    }
    advtools::verify("Difference product", diffs.0 * diffs.1, 2312);
    advtools::verify("Number of arrangements", arranges, 12089663946752u64);
}
