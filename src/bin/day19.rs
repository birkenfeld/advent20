use advtools::prelude::{HashMap, Itertools};
use advtools::input;
use advtools::itertools::repeat_n;

fn expand(rules: &HashMap<i16, Vec<Vec<i16>>>, rule: &[Vec<i16>], text: &str) -> Option<usize> {
    'alts: for alt in rule {
        let mut i = 0;
        for &subrule in alt {
            if subrule == -1 && text[i..].starts_with('a') {
                i += 1;
            } else if subrule == -2 && text[i..].starts_with('b') {
                i += 1;
            } else if let Some(j) = expand(rules, &rules[&subrule], &text[i..]) {
                i += j;
            } else {
                continue 'alts;
            }
        }
        return Some(i);
    }
    None
}

fn main() {
    let mut iter = input::lines();
    let mut msgs = vec![];
    let mut rules = HashMap::new();
    rules.insert(-1, vec![]);
    rules.insert(-2, vec![]);

    while let Some(rule) = iter.next() {
        if rule.starts_with(&['a', 'b'][..]) {
            msgs.push(rule);
            msgs.extend(iter);
            break;
        }
        let parts = rule.split_whitespace().collect_vec();
        let index = parts[0].trim_matches(':').parse().unwrap();
        let mut substs = vec![vec![]];
        for &part in &parts[1..] {
            if part == "|" {
                substs.push(vec![]);
            } else if part == "\"a\"" {
                substs.last_mut().unwrap().push(-1);
            } else if part == "\"b\"" {
                substs.last_mut().unwrap().push(-2);
            } else {
                substs.last_mut().unwrap().push(part.parse().unwrap());
            }
        }
        rules.insert(index, substs);
    }

    let n = msgs.iter()
                .filter(|msg| expand(&rules, &rules[&0], msg) == Some(msg.len()))
                .count();
    advtools::verify("Simple rules", n, 142);

    let n = msgs.iter()
                .filter_map(|msg| (2..10).cartesian_product(1..5).find(|&(n, m)| {
                    let rule = repeat_n(42, n).chain(repeat_n(31, m)).collect();
                    n > m && expand(&rules, &[rule], msg) == Some(msg.len())
                }))
                .count();
    advtools::verify("Complex rules", n, 294);
}
