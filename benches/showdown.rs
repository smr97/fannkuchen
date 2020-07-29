#[macro_use]
extern crate criterion;
extern crate fannkuchen;

use fannkuchen::{fannkuchh_adaptive, fannkuchh_fastest, fannkuchh_rayon, fannkuchh_sequential};
use std::time::Duration;

use criterion::{Benchmark, Criterion, ParameterizedBenchmark};
const SIZE: usize = 12;

fn fannkuchh_benchmarks(c: &mut Criterion) {
    let num_threads: Vec<_> = (2..65).filter(|nt| nt % 2 == 0).collect();
    c.bench(
        "fannkuchh_redux_sequential",
        Benchmark::new("sequential fannkuchh", |b| {
            b.iter_with_setup(
                || {
                    let tp = rayon::ThreadPoolBuilder::new()
                        .num_threads(1)
                        .build()
                        .expect("Couldn't build thread pool");
                    tp
                },
                |tp| {
                    tp.install(|| fannkuchh_sequential(SIZE));
                },
            )
        }),
    );
    c.bench(
        "fannkuchh_redux_parallel",
        ParameterizedBenchmark::new(
            "adaptive fannkuchh",
            |b, nt| {
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
                            fannkuchh_adaptive(SIZE);
                        });
                    },
                )
            },
            num_threads.clone(),
        )
        .with_function("rayon fannkuchh", |b, nt| {
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
                        fannkuchh_rayon(SIZE);
                    });
                },
            )
        })
        .with_function("static fannkuchh", |b, nt| {
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
                        fannkuchh_fastest(SIZE, 10 * nt);
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
