use rayon_try_fold::prelude::*;
use std::{cmp::min, mem::replace, ops::Range};

// One greater than the maximum `n` value. Used to size stack arrays.
const MAX_N: usize = 16;
const MIN_BLOCKSIZE: usize = 10;

struct PfannkuchhZustand {
    max_flip_count: i32,
    checksum: i32,
    perm_range: Range<usize>,
    current_permutation: [u8; MAX_N],
    count: [usize; MAX_N],
    new: bool,
}

impl Divisible for PfannkuchhZustand {
    type Controlled = True;
    fn should_be_divided(&self) -> bool {
        self.perm_range.len() > MIN_BLOCKSIZE
    }
    fn divide(self) -> (Self, Self) {
        let (leftr, rightr) = self.perm_range.divide();
        (
            PfannkuchhZustand {
                max_flip_count: self.max_flip_count,
                checksum: self.checksum,
                perm_range: leftr,
                current_permutation: self.current_permutation,
                count: self.count,
                new: self.new,
            },
            PfannkuchhZustand {
                max_flip_count: 0,
                checksum: 0,
                perm_range: rightr,
                current_permutation: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
                count: [0; MAX_N],
                new: true,
            },
        )
    }
    fn divide_at(self, index: usize) -> (Self, Self) {
        let (leftr, rightr) = self.perm_range.divide_at(index);
        (
            PfannkuchhZustand {
                max_flip_count: self.max_flip_count,
                checksum: self.checksum,
                perm_range: leftr,
                current_permutation: self.current_permutation,
                count: self.count,
                new: self.new,
            },
            PfannkuchhZustand {
                max_flip_count: 0,
                checksum: 0,
                perm_range: rightr,
                current_permutation: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
                count: [0; MAX_N],
                new: true,
            },
        )
    }
}

//Adaptively selects the block size
pub fn fannkuchh_adaptive(n: usize) -> (i32, i32) {
    // This assert eliminates several bounds checks.
    assert!(n < MAX_N);

    // Create and initialize factorial_lookup_table.
    let factorial_lookup_table = {
        let mut table: [usize; MAX_N] = [0; MAX_N];
        table[0] = 1;
        for i in 1..MAX_N {
            table[i] = i * table[i - 1];
        }
        table
    };

    // Iterate over each block.
    PfannkuchhZustand {
        max_flip_count: 0,
        checksum: 0,
        perm_range: (0..factorial_lookup_table[n]),
        current_permutation: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
        new: true,
        count: [0; MAX_N],
    }
    .work(
        |state| state.perm_range.len() == 0,
        |state, limit| {
            let right_end = min(state.perm_range.end, state.perm_range.start + limit);
            let dieser_range = state.perm_range.start..right_end;
            let initial_permutation_index = dieser_range.start;

            // Initialize count and current_permutation only if this state is fresh.
            if state.new == true {
                let mut temp_permutation: [u8; MAX_N] = [0; MAX_N];
                let mut permutation_index = initial_permutation_index;
                for i in (1..n).rev() {
                    let f = factorial_lookup_table[i];
                    let d = permutation_index / f;

                    state.count[i] = d;

                    // Rotate the permutation left by d places. This is faster
                    // than using slice::rotate_left.
                    temp_permutation[0..=i - d].copy_from_slice(&state.current_permutation[d..=i]);
                    temp_permutation[i - d + 1..=i]
                        .copy_from_slice(&state.current_permutation[..d]);
                    state.current_permutation = temp_permutation;

                    permutation_index = permutation_index % f;
                }
            }

            // Iterate over each permutation in the block.
            for permutation_index in dieser_range {
                // If the first value in the current_permutation is not 1 (0)
                // then we will need to do at least one flip for the
                // current_permutation.
                if state.current_permutation[0] > 0 {
                    // Make a copy of current_permutation[] to work on.
                    let mut temp_permutation = state.current_permutation;

                    let mut flip_count = 1;

                    // Flip temp_permutation until the element at the
                    // first_value index is 1 (0).
                    let mut first_value = state.current_permutation[0] as usize & 0xF;
                    while temp_permutation[first_value] > 0 {
                        // Record the new_first_value and restore the old
                        // first_value at its new flipped position.
                        let new_first_value =
                            replace(&mut temp_permutation[first_value], first_value as u8);

                        // If first_value is greater than 3 (2) then we are
                        // flipping a series of four or more values so we will
                        // also need to flip additional elements in the middle
                        // of the temp_permutation.
                        if first_value > 2 {
                            for (low_index, high_index) in
                                (1..first_value).zip((1..first_value).rev())
                            {
                                temp_permutation.swap(high_index, low_index);

                                if low_index + 3 > high_index {
                                    break;
                                }
                            }
                        }

                        // Update first_value to new_first_value that we
                        // recorded earlier.
                        first_value = new_first_value as usize & 0xF;
                        flip_count += 1;
                    }

                    // Update the checksum.
                    if permutation_index % 2 == 0 {
                        state.checksum += flip_count;
                    } else {
                        state.checksum -= flip_count;
                    }

                    // Update max_flip_count if necessary.
                    state.max_flip_count = state.max_flip_count.max(flip_count);
                }

                // Generate the next permutation.
                state.current_permutation.swap(0, 1);
                let mut first_value = state.current_permutation[0];
                for i in 1..MAX_N - 2 {
                    state.count[i] += 1;
                    if state.count[i] <= i {
                        break;
                    }
                    state.count[i] = 0;

                    let new_first_value = state.current_permutation[1];

                    for j in 0..i + 1 {
                        state.current_permutation[j] = state.current_permutation[j + 1];
                    }

                    state.current_permutation[i + 1] = first_value;
                    first_value = new_first_value;
                }
            }
            // "Consume" the part of range that has been worked upon.
            state.perm_range.start = right_end;
            state.new = false;
        },
    )
    .micro_block_sizes(10, 1_000)
    .map(|zustand| (zustand.checksum, zustand.max_flip_count))
    .reduce(|| (0, 0), |l, r| (l.0 + r.0, l.1.max(r.1)))
}
