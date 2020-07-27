#[macro_use]
extern crate criterion;
extern crate fannkuchen;
#[macro_use]
extern crate itertools;

use fannkuchen::fannkuchh_adaptive;
use std::time::Duration;

use criterion::{Criterion, ParameterizedBenchmark};

fn fannkuchh_benchmarks(c: &mut Criterion) {
    let blocksizes: Vec<_> = vec![10, 100, 1000, 10_000, 100_000];
    c.bench(
        "N11",
        ParameterizedBenchmark::new(
            "adaptive",
            |b, blocksize| {
                b.iter_with_setup(
                    || {
                        let tp = rayon::ThreadPoolBuilder::new()
                            .num_threads(1)
                            .build()
                            .expect("Couldn't build thread pool");
                        tp
                    },
                    |tp| {
                        tp.install(|| fannkuchh_adaptive(11, *blocksize));
                    },
                )
            },
            iproduct!(blocksizes.clone()),
        ),
    );
    c.bench(
        "N12",
        ParameterizedBenchmark::new(
            "adaptive",
            |b, blocksize| {
                b.iter_with_setup(
                    || {
                        let tp = rayon::ThreadPoolBuilder::new()
                            .num_threads(1)
                            .build()
                            .expect("Couldn't build thread pool");
                        tp
                    },
                    |tp| {
                        tp.install(|| fannkuchh_adaptive(12, *blocksize));
                    },
                )
            },
            iproduct!(blocksizes.clone()),
        ),
    );
}

criterion_group! {
    name = benches;
            config = Criterion::default().sample_size(10).warm_up_time(Duration::from_secs(1)).nresamples(1000);
                targets = fannkuchh_benchmarks
}
criterion_main!(benches);
