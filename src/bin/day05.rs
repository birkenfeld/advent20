use advtools::prelude::{HashSet, Itertools};
use advtools::input;

fn main() {
    let all_ids: HashSet<u32> = input::lines()
        .map(|line| line.chars().rev().enumerate()
             .map(|(i, c)| (matches!(c, 'B' | 'R') as u32) << i)
             .sum())
        .collect();

    let (min, max) = all_ids.iter().minmax().into_option().unwrap();
    advtools::verify("Highest row", max, 806);

    let my_seat = (*min..*max).find(|id| !all_ids.contains(id)).unwrap();
    advtools::verify("My seat", my_seat, 562);
}
