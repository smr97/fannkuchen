use fannkuchen::fannkuchh_adaptive;
#[cfg(not(feature = "logs"))]
use rayon::ThreadPoolBuilder;
#[cfg(feature = "logs")]
use rayon_logs::ThreadPoolBuilder;

fn main() {
    #[cfg(feature = "logs")]
    {
        let n = std::env::args().nth(1).unwrap().parse().unwrap();
        let thread_pool = ThreadPoolBuilder::new()
            .num_threads(4)
            .build()
            .expect("Couldn't build thread pool");

        thread_pool
            .logging_install(|| {
                fannkuchh_adaptive(n);
            })
            .1
            .save_svg("adaptive_log.svg");
    }
}
