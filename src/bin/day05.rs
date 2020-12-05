use advtools::prelude::*;
use advtools::input::iter_input;

fn main() {
    let all_ids: HashSet<u32> = iter_input::<String>()
        .map(|line| line.chars().take(10).enumerate()
             .map(|(i, c)| ((c == 'B' || c == 'R') as u32) << (9 - i))
             .sum())
        .collect();

    let (min, max) = all_ids.iter().minmax().into_option().unwrap();
    advtools::verify("Highest row", max, 806);

    let my_seat = (*min..*max).find(|id| !all_ids.contains(id)).unwrap();
    advtools::verify("My seat", my_seat, 562);
}
