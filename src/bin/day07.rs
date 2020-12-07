use advtools::prelude::*;
use advtools::input::input_string;

const MINE: &str = "shiny gold";
type Map<'s> = HashMap<&'s str, Vec<(u32, &'s str)>>;

fn contains_mine<'s>(bag: &'s str, map: &Map<'s>) -> bool {
    map[&bag].iter().any(|(_, inner)| inner == &MINE || contains_mine(inner, map))
}

fn count_bags<'s>(bag: &'s str, map: &Map<'s>) -> u32 {
    1 + map[&bag].iter().map(|(n, inner)| n * count_bags(inner, map)).sum::<u32>()
}

fn main() {
    let input = input_string();
    let map: Map = input.lines().map(|line| {
        let (outer, inners) = line.split(" bags contain ").collect_tuple().unwrap();
        (outer, inners.split(", ").filter_map(|part| {
            part[..1].parse::<u32>().ok().map(|n| {
                (n, part[2..].split(" bag").next().unwrap())
            })
        }).collect_vec())
    }).collect();

    let count = map.keys().filter(|k| contains_mine(k, &map)).count();
    advtools::verify("Bag types that contain mine", count, 261);
    advtools::verify("Bags inside my bag", count_bags(MINE, &map) - 1, 3765);
}
