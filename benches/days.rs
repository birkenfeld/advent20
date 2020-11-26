use criterion::*;

macro_rules! make_benches {
    ($($mod:tt),+) => {
        $(
            mod $mod {
                include!(concat!("../src/bin/", stringify!($mod), ".rs"));
                use criterion::{Criterion, Benchmark};
                pub fn criterion_benchmark(c: &mut Criterion) {
                    use std::time::Duration;
                    advtools::bench_mode(
                        concat!("input/", stringify!($mod), ".txt"));
                    let b = Benchmark::new(stringify!($mod), |b| b.iter(main))
                        .sample_size(10)
                        .warm_up_time(Duration::from_millis(500))
                        .measurement_time(Duration::from_millis(1000));
                    c.bench("aoc", b);
                }
            }
        )+
        criterion_group!(benches, $($mod::criterion_benchmark),+);
    };
}

make_benches!(day01);
criterion_main!(benches);
