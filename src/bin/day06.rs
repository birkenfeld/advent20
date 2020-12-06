use advtools::prelude::*;
use advtools::input::input_string;

fn main() {
    let mut any_count = 0;
    let mut all_count = 0;

    for group in input_string().split("\n\n") {
        let any_yes = group.chars().filter(|&ch| ch != '\n').collect::<HashSet<_>>();
        any_count += any_yes.len();

        let all_yes = group.lines().map(|l| l.chars().collect::<HashSet<_>>())
            .fold(any_yes, |a, b| &a & &b);
        all_count += all_yes.len();
    }

    advtools::verify("Number of \"any said yes\"", any_count, 6947);
    advtools::verify("Number of \"all said yes\"", all_count, 3398);
}
