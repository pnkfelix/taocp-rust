mod sect_1_3_2;

use itertools::Itertools;

fn main() {
    println!("Hello, world!");

    println!("{}",
             sect_1_3_2::ex_13::frequency_count_sync(
                 std::iter::repeat(b'A').take(10257)
                     .interleave(std::iter::repeat(b'B').take(179))
                     .interleave(std::iter::repeat(b'D').take(794301))));
}
