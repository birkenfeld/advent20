use advtools::itertools::Itertools;
use advtools::input::iter_input;

fn main() {
    let items: Vec<i32> = iter_input().sorted().collect();

    for comb in items.iter().combinations(2) {
        if comb[0] + comb[1] == 2020 {
            let product = comb[0] * comb[1];
            advtools::verify("Product of 2", product, 41979);
            break;
        }
    }

    for comb in items.iter().combinations(3) {
        if comb[0] + comb[1] + comb[2] == 2020 {
            let product = comb[0] * comb[1] * comb[2];
            advtools::verify("Product of 3", product, 193416912);
            break;
        }
    }
}
