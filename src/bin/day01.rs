use advtools::itertools::Itertools;
use advtools::input;

fn main() {
    let items: Vec<i32> = input::parse_lines().sorted().collect();

    let find_product = |n| items.iter().combinations(n)
        .find(|c| c.iter().copied().sum::<i32>() == 2020)
        .map_or(0, |c| c.into_iter().product::<i32>());

    advtools::verify("Product of 2", find_product(2), 41979);
    advtools::verify("Product of 3", find_product(3), 193416912);
}
