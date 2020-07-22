#[macro_use]
extern crate criterion;
extern crate fannkuchen;
#[macro_use]
extern crate itertools;

use fannkuchen::fannkuchh_fastest;
use std::time::Duration;

use criterion::{Criterion, ParameterizedBenchmark};

fn fannkuchh_benchmarks(c: &mut Criterion) {
    let sizes: Vec<_> = vec![11, 12, 13];
    let num_threads: Vec<_> = vec![4, 32, 64];
    let multipliers: Vec<_> = vec![2, 4, 6, 8, 10];
    c.bench(
        "blocksize tuning",
        ParameterizedBenchmark::new(
            "adaptive fannkuchh",
            |b, (n, nt, factor)| {
                b.iter_with_setup(
                    || {
                        let tp = rayon::ThreadPoolBuilder::new()
                            .num_threads(*nt)
                            .build()
                            .expect("Couldn't build thread pool");
                        tp
                    },
                    |tp| {
                        tp.install(|| {
                            fannkuchh_fastest(*n, factor * nt);
                        });
                    },
                )
            },
            iproduct!(sizes.clone(), num_threads.clone(), multipliers.clone()),
        ),
    );
}

criterion_group! {
    name = benches;
            config = Criterion::default().sample_size(15).warm_up_time(Duration::from_secs(1)).nresamples(1000);
                targets = fannkuchh_benchmarks
}
criterion_main!(benches);
