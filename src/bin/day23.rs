use advtools::prelude::Itertools;
use advtools::input;

const N: usize = 1_000_000;

fn play(next: &mut [usize], mut head: usize, rounds: usize) {
    let max = next.len() - 1;
    for _ in 0..rounds {
        let out1 = next[head];
        let out2 = next[out1];
        let out3 = next[out2];
        let new_head = next[out3];

        let mut insert_at = head - 1;
        while insert_at == 0 || insert_at == out1 || insert_at == out2 || insert_at == out3 {
            if insert_at == 0 {
                insert_at = max;
            } else {
                insert_at -= 1;
            }
        }
        let after_insert = next[insert_at];

        next[head] = new_head;
        next[insert_at] = out1;
        next[out3] = after_insert;

        head = new_head;
    }
}

fn main() {
    let input = input::chars().map(|ch| (ch as u8 - b'0') as usize).collect_vec();
    let head = input[0];
    let mut next1 = vec![0; input.len() + 1];
    for (i, j) in (0..input.len()-1).zip(1..input.len()) {
        next1[input[i]] = input[j];
    }
    let mut next2 = next1.clone();

    next1[input[input.len()-1]] = head;
    play(&mut next1, head, 100);
    let rest = (0..input.len()-1).scan(1, |at, _| { *at = next1[*at]; Some(*at) }).format("");
    advtools::verify("100 times", rest, "58427369");

    next2.resize(N+1, 0);
    next2[input[input.len()-1]] = input.len()+1;
    (input.len()+1..=N).for_each(|i| next2[i] = i+1);
    next2[N] = head;

    play(&mut next2, head, 10*N);
    advtools::verify("10M times", next2[1] * next2[next2[1]], 111057672960_u64);
}
