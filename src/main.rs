#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;
mod fannkuchh_adaptive;
mod fannkuchh_original;
mod fannkuchh_rayon;
mod fannkuchh_sequential;
use fannkuchh_adaptive::fannkuchh_adaptive;
use fannkuchh_original::fannkuchh_fastest;
use fannkuchh_rayon::fannkuchh_rayon;
use fannkuchh_sequential::fannkuchh_sequential;
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
    ($tp: ident, $inp: ident, $inp2: ident, $solver: ident) => {
        mean_time(|| {
            $tp.install(|| {
                let start = Instant::now();
                $solver($inp, $inp2);
                start.elapsed()
            })
        })
    };
}
fn main() {
    let n = std::env::args()
        .nth(1)
        .expect("Enter n as the first arg.")
        .parse()
        .expect("Enter valid n.");
    let nt = std::env::args()
        .nth(2)
        .expect("Enter num_threads as the second arg.")
        .parse()
        .expect("Enter valid number of threads.");
    let thread_pool = rayon::ThreadPoolBuilder::new()
        .num_threads(nt)
        .build()
        .expect("Couldn't build thread pool");
    let (checksum, max_flip_count_0) = fannkuchh_fastest(n, 6 * nt);
    let num_blocks = 6 * nt;
    let (_, max_flip_count_1) = fannkuchh_rayon(n);
    let (checksum_adaptive, max_flip_count_2) = fannkuchh_adaptive(n);
    let (_, max_flip_count_3) = fannkuchh_sequential(n);
    let mean_time_adaptive = bench_fannkuchen!(thread_pool, n, fannkuchh_adaptive);
    let mean_time_original = bench_fannkuchen!(thread_pool, n, num_blocks, fannkuchh_fastest);
    let mean_time_rayon = bench_fannkuchen!(thread_pool, n, fannkuchh_rayon);
    let mean_time_sequential = bench_fannkuchen!(thread_pool, n, fannkuchh_sequential);
    println!("Checksum {}", checksum);
    println!("Checksum Adaptive {}", checksum_adaptive);
    println!("Adaptive time {:?}", mean_time_adaptive);
    println!("Original time {:?}", mean_time_original);
    println!("Rayon time {:?}", mean_time_rayon);
    println!("Sequential time {:?}", mean_time_sequential);
    assert_eq!(max_flip_count_0, max_flip_count_0);
    assert_eq!(max_flip_count_0, max_flip_count_1);
    assert_eq!(max_flip_count_0, max_flip_count_2);
    assert_eq!(max_flip_count_0, max_flip_count_3);
    println!("Pfannkuchen({}) = {}", n, max_flip_count_0);
}
