const INPUT: &[usize] = &[0, 14, 1, 3, 7, 9];

fn main() {
    let mut turns = vec![0u32; 30000000];

    for (t, &n) in INPUT.iter().enumerate() {
        turns[n] = t as u32 + 1;
    }

    let mut number = 0;
    for t in INPUT.len()+1..30000000 {
        let prev_t = turns[number] as usize;
        turns[number] = t as u32;

        number = if prev_t != 0 { t - prev_t } else { 0 };

        if t == 2019 {
            advtools::verify("2020th number", number, 763);
        }
    }

    advtools::verify("30000000th number", number, 1876406);
}
