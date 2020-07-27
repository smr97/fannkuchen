#[macro_use]
extern crate criterion;
extern crate fannkuchen;
#[macro_use]
extern crate itertools;

use fannkuchen::{fannkuchh_adaptive, fannkuchh_fastest, fannkuchh_rayon, fannkuchh_sequential};
use std::time::Duration;

use criterion::{Criterion, ParameterizedBenchmark};

fn fannkuchh_benchmarks(c: &mut Criterion) {
    let sizes: Vec<_> = vec![11, 12, 13];
    let num_threads: Vec<_> = (1..64).filter(|nt| nt % 2 == 0).collect();
    c.bench(
        "fannkuchh_redux_sequential",
        ParameterizedBenchmark::new(
            "sequential fannkuchh",
            |b, n| {
                b.iter_with_setup(
                    || {
                        let tp = rayon::ThreadPoolBuilder::new()
                            .num_threads(1)
                            .build()
                            .expect("Couldn't build thread pool");
                        tp
                    },
                    |tp| {
                        tp.install(|| fannkuchh_sequential(*n));
                    },
                )
            },
            sizes.clone(),
        ),
    );
    c.bench(
        "fannkuchh_redux_parallel",
        ParameterizedBenchmark::new(
            "adaptive fannkuchh",
            |b, (n, nt)| {
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
                            fannkuchh_adaptive(*n);
                        });
                    },
                )
            },
            iproduct!(sizes.clone(), num_threads.clone()),
        )
        .with_function("original fannkuchh", |b, (n, nt)| {
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
                        fannkuchh_rayon(*n);
                    });
                },
            )
        })
        .with_function("original fannkuchh", |b, (n, nt)| {
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
                        fannkuchh_fastest(*n, 10 * nt);
                    });
                },
            )
        }),
    );
}

criterion_group! {
    name = benches;
            config = Criterion::default().sample_size(15).warm_up_time(Duration::from_secs(1)).nresamples(1000);
                targets = fannkuchh_benchmarks
}
criterion_main!(benches);
