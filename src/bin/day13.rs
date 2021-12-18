use advtools::prelude::Itertools;
use advtools::input;
use num::Integer;

fn main() {
    let (earliest, buses) = input::rx_parse::<(i64, &str)>(r"(\d+)\n(.+)");
    let all_buses = buses.split(',')
        .enumerate()
        .filter_map(|(i, x)| x.parse::<i64>().ok().map(|n| (n, i as i64)))
        .sorted().rev().collect_vec();

    let first_bus = (earliest..).cartesian_product(&all_buses)
        .find(|(time, (bus, _))| time % bus == 0)
        .map(|(time, (bus, _))| (time - earliest) * bus)
        .unwrap();

    advtools::verify("Bus * Wait", first_bus, 115);

    let mut groups: Vec<(i64, i64)> = vec![];
    for (n, dt) in all_buses {
        if let Some((i, (n1, dt1))) = groups.iter().copied()
            .enumerate()
            .find(|(_, (_, dt1))| (dt1 - dt) % n == 0)
        {
            groups[i] = (n1.lcm(&n), dt1);
        } else {
            groups.push((n, dt));
        }
    }

    advtools::verify("Contest timestamp", chinese_remainder(&groups), 756261495958122i64);
}

fn chinese_remainder(groups: &[(i64, i64)]) -> i64 {
    let prod = groups.iter().map(|(n, _)| n).product::<i64>();
    let mut sum = 0;
    for &(n, dt) in groups {
        let p = prod / n;
        sum += (n - dt) as i128 * (mod_inv(p, n) as i128) * (p as i128);
    }
    (sum % prod as i128) as i64
}

fn mod_inv(x: i64, n: i64) -> i64 {
    fn egcd(a: i64, b: i64) -> (i64, i64) {
        if a == 0 {
            (0, 1)
        } else {
            let (x, y) = egcd(b % a, a);
            (y - (b / a) * x, x)
        }
    }
    (egcd(x, n).0 % n + n) % n
}
