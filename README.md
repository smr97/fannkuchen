# What
This repository hosts several parallel implementations for the fannkuch_redux benchmark.
The problem (`pancake flipping`) and its current state-of-the-art benchmark results are documented [here](https://benchmarksgame-team.pages.debian.net/benchmarksgame/performance/fannkuchredux.html).
This repository uses the Rust #5 implementation originally contributed by Cliff Biffle. This code can be found (nearly) as-is in the file `fannkuchh_original.rs`.

# Why
My aim is to compare the effects of a new task splitting scheduler on the scalability of this algorithm. It is a particularly good candidate becuase it is almost completely processor bound.
Furthermore, the amount of work to be done in each iteration is not very well defined (not equal) and hence there could be some tiny load imbalances.
`fannkuchh_adaptive.rs` contains a stateful iterator that splits itself adadptively depending on the amount of parallelism in the machine.

# How
Just run cargo bench for comparing all implementations. The `main.rs` file has a quick bench for comparing the implementations without the `Criterion` crate.
Furthermore, the release profile has been modified to build the fastest binary code possible. This is not done for the benchmark, since the bench profile can not be changed.
