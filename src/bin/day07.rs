use advtools::prelude::HashMap;
use advtools::input;

const MINE: &str = "shiny gold";
type Map = HashMap<&'static str, Vec<(u32, &'static str)>>;

fn contains_mine(bag: &'static str, map: &Map) -> bool {
    map[&bag].iter().any(|&(_, inner)| inner == MINE || contains_mine(inner, map))
}

fn count_bags(bag: &'static str, map: &Map) -> u32 {
    1 + map[&bag].iter().map(|(n, inner)| n * count_bags(inner, map)).sum::<u32>()
}

fn main() {
    let map: Map = input::rx_lines::<(&str, &str)>("(.+) bags contain (.+)")
        .map(|(outer, inners)| (outer, inners.split(", ").filter_map(|part| {
            // Parse inner bags, which might be empty ("contain no other bags").
            let num = part[..1].parse().ok()?;
            Some((num, part[2..].split(" bag").next().unwrap()))
        }).collect())).collect();

    let count = map.keys().filter(|k| contains_mine(k, &map)).count();
    advtools::verify("Bag types that contain mine", count, 261);
    advtools::verify("Bags inside my bag", count_bags(MINE, &map) - 1, 3765);
}
