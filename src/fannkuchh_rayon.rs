use rayon::prelude::*;
use std::mem::replace;
// One greater than the maximum `n` value. Used to size stack arrays.
const MAX_N: usize = 16;

//Currently seems slow because of counts based optimisation
pub fn fannkuchh_rayon(n: usize) -> (usize, usize) {
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

    let number_of_permutations = factorial_lookup_table[n];

    // Iterate over each block.
    (0..number_of_permutations)
        .into_par_iter()
        .map(|current_permutation_index| {
            let mut current_permutation: [u8; MAX_N] =
                [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];

            // Make the permutation needed for this run
            let mut temp_permutation: [u8; MAX_N] = [0; MAX_N];
            let mut permutation_index = current_permutation_index;
            for i in (1..n).rev() {
                let f = factorial_lookup_table[i];
                let d = permutation_index / f;

                // Rotate the permutation left by d places. This is faster
                // than using slice::rotate_left.
                temp_permutation[0..=i - d].copy_from_slice(&current_permutation[d..=i]);
                temp_permutation[i - d + 1..=i].copy_from_slice(&current_permutation[..d]);
                current_permutation = temp_permutation;

                permutation_index = permutation_index % f;
            }

            let mut max_flip_count = 0;
            let mut checksum = 0;

            // If the first value in the current_permutation is not 1 (0)
            // then we will need to do at least one flip for the
            // current_permutation.
            if current_permutation[0] > 0 {
                // Make a copy of current_permutation[] to work on.
                let mut temp_permutation = current_permutation;

                let mut flip_count: usize = 1;

                // Flip temp_permutation until the element at the
                // first_value index is 1 (0).
                let mut first_value = current_permutation[0] as usize & 0xF;
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
                        for (low_index, high_index) in (1..first_value).zip((1..first_value).rev())
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
                if current_permutation_index % 2 == 0 {
                    checksum += flip_count;
                } else {
                    checksum -= flip_count;
                }

                // Update max_flip_count if necessary.
                max_flip_count = max_flip_count.max(flip_count);
            }

            (checksum, max_flip_count)
        })
        .reduce(
            || (0, 0),
            |(cs1, mf1), (cs2, mf2)| (cs1 + cs2, mf1.max(mf2)),
        )
}
