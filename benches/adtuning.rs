#[macro_use]
extern crate criterion;
extern crate fannkuchen;
#[macro_use]
extern crate itertools;

use fannkuchen::fannkuchh_adaptive;
use std::time::Duration;

use criterion::{Criterion, ParameterizedBenchmark};

fn fannkuchh_benchmarks(c: &mut Criterion) {
    let sizes: Vec<_> = vec![12, 13];
    let num_threads: Vec<_> = vec![4, 11, 32, 43, 64];
    let blocksizes: Vec<_> = vec![10, 100, 1000, 10_000, 100_000];
    c.bench(
        "adaptive block size tuning",
        ParameterizedBenchmark::new(
            "sequential fannkuchh",
            |b, (n, nt, blocksize)| {
                b.iter_with_setup(
                    || {
                        let tp = rayon::ThreadPoolBuilder::new()
                            .num_threads(*nt)
                            .build()
                            .expect("Couldn't build thread pool");
                        tp
                    },
                    |tp| {
                        tp.install(|| fannkuchh_adaptive(*n, *blocksize));
                    },
                )
            },
            iproduct!(sizes.clone(), num_threads.clone(), blocksizes.clone()),
        ),
    );
}

criterion_group! {
    name = benches;
            config = Criterion::default().sample_size(15).warm_up_time(Duration::from_secs(1)).nresamples(1000);
                targets = fannkuchh_benchmarks
}
criterion_main!(benches);
