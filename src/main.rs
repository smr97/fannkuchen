mod fannkuchh_adaptive;
mod fannkuchh_original;
mod fannkuchh_rayon;
use crate::fannkuchh_adaptive::fannkuch_adaptive;
use crate::fannkuchh_original::fannkuch_fastest;
use std::time::Instant;

const NUM_RUNS: usize = 5;

fn mean_time<F: FnMut() -> std::time::Duration>(mut bench_function: F) -> std::time::Duration {
    (0..NUM_RUNS)
        .map(|_| bench_function())
        .sum::<std::time::Duration>()
        .div_f64(NUM_RUNS as f64)
}
macro_rules! bench_fannkuchen {
    ($inp: ident, $solver: ident) => {
        mean_time(|| {
            let start = Instant::now();
            $solver($inp);
            start.elapsed()
        })
    };
    ($tp: ident, $inp: ident, $solver: ident) => {
        mean_time(|| {
            $tp.install(|| {
                let start = Instant::now();
                $solver($inp);
                start.elapsed()
            })
        })
    };
}
fn main() {
    let n = std::env::args().nth(1).unwrap().parse().unwrap();
    let thread_pool = rayon::ThreadPoolBuilder::new()
        .num_threads(4)
        .build()
        .expect("Couldn't build thread pool");
    let (checksum_0, max_flip_count_0) = fannkuch_fastest(n);
    let (checksum_2, max_flip_count_2) = fannkuch_adaptive(n);
    let mean_time_fastest = bench_fannkuchen!(thread_pool, n, fannkuch_fastest);
    let mean_time_rayon = bench_fannkuchen!(thread_pool, n, fannkuch_adaptive);
    println!("Fastest time {:?}", mean_time_fastest);
    println!("Adaptive time {:?}", mean_time_rayon);
    //assert_eq!(checksum_2, checksum_0);
    assert_eq!(max_flip_count_0, max_flip_count_2);
    // Output the results to stdout.
    println!("Pfannkuchen({}) = {}", n, max_flip_count_0);
}
