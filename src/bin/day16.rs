use advtools::prelude::{HashMap, HashSet, Itertools};
use advtools::input;

fn belongs([(a, b), (c, d)]: &[(u32, u32); 2], n: &u32) -> bool {
    (a <= n && n <= b) || (c <= n && n <= d)
}

fn main() {
    let mut iter = input::lines();
    let mut ranges = vec![];
    let mut tickets = vec![];

    for line in &mut iter {
        if line == "your ticket:" {
            break;
        }
        let split = line.split_whitespace().collect_vec();
        let r1 = split[split.len() - 3].split('-').map(input::to_u32).collect_tuple().unwrap();
        let r2 = split[split.len() - 1].split('-').map(input::to_u32).collect_tuple().unwrap();
        ranges.push([r1, r2]);
    }

    let my_ticket = iter.next().unwrap().split(',').map(input::to_u32).collect_vec();

    for line in iter.skip(1) {
        tickets.push(line.split(',').map(input::to_u32).collect_vec());
    }

    let mut error_rate = 0;
    tickets.retain(|ticket| {
        let errors = ticket.iter().filter(|val| !ranges.iter().any(|r| belongs(r, val)));
        if let Some(error) = errors.sum1::<u32>() {
            error_rate += error;
            false
        } else {
            true
        }
    });

    advtools::verify("Error rate", error_rate, 23122);

    let mut places_left = (0..ranges.len()).collect::<HashSet<_>>();
    let mut found = HashMap::new();

    while !places_left.is_empty() {
        for (j, r) in ranges.iter().enumerate() {
            if !found.contains_key(&j) {
                if let Ok(&x) = places_left.iter().filter(|&&i| {
                    tickets.iter().all(|t| belongs(r, &t[i]))
                }).exactly_one() {
                    places_left.remove(&x);
                    found.insert(j, x);
                }
            }
        }
    }

    let mult = (0..6).map(|j| my_ticket[found[&j]] as u64).product::<u64>();
    advtools::verify("My ticket", mult, 362974212989u64);
}
