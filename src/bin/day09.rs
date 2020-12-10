use advtools::prelude::*;
use advtools::input::iter_input;

fn main() {
    let numbers: Vec<u64> = iter_input().collect();

    let not_sum = numbers.windows(26)
        .find(|win| win[..25].iter().combinations(2).all(|x| x[0] + x[1] != win[25]))
        .map(|win| win[25])
        .unwrap();
    advtools::verify("Number that does not sum", not_sum, 1504371145);

    let weakness = (2..numbers.len())
        .flat_map(|len| numbers.windows(len))
        .find(|win| win.iter().sum::<u64>() == not_sum)
        .unwrap();
    let (min, max) = weakness.iter().minmax().into_option().unwrap();
    advtools::verify("Encryption weakness", min + max, 183278487);
}
