#![feature(bigint_helper_methods)]

mod sect_1_3_2;

use itertools::Itertools;

fn main() {
    println!("Hello, world!");

    println!("{}",
             sect_1_3_2::ex_13::frequency_count_sync(
                 std::iter::repeat(b'A').take(10257)
                     .interleave(std::iter::repeat(b'B').take(179))
                     .interleave(std::iter::repeat(b'D').take(794301))));

    dbg!(sect_1_3_2::mmix_ex_27::search_float_divergence());
    dbg!(sect_1_3_2::mmix_ex_27::search_fixed_divergence());
}
