mod ex_10 {
    const H: u8 = 9;
    const W: u8 = 8;
    const LEN: usize = (H * W) as usize;
    #[derive(Copy, Clone, PartialEq, Eq)]
    struct NineByEight([u32; LEN]);
    impl std::fmt::Debug for NineByEight {
        fn fmt(&self, w: &mut std::fmt::Formatter) -> std::fmt::Result {
            let mut rows = self.0[..].chunks(W as usize);
            for row in rows {
                row.fmt(w)?;
            }
            Ok(())
        }
    }

    trait SaddlePoint {
        fn saddle_point(&self) -> Option<(u8, u8)>;
    }
    impl NineByEight {
        fn build(f: impl Fn(u8, u8) -> u32) -> Self {
            let mut m = [0; LEN];
            for i in 0..H {
                for j in 0..W {
                    m[Self::idx(i, j)] = f(i, j);
                }
            }
            NineByEight(m)
        }
        fn idx(row: u8, col: u8) -> usize {
            let res = (row * W) as usize + (col as usize);
            res
        }
        fn col(idx: usize) -> u8 {
            (idx % W as usize).try_into().unwrap()
        }
        fn row(idx: usize) -> u8 {
            (idx / W as usize).try_into().unwrap()
        }
    }
    impl SaddlePoint for NineByEight {
        fn saddle_point(&self) -> Option<(u8, u8)> {
            for i in 0..H {
                // find smallest in row i
                let start = Self::idx(i, 0);
                let limit = Self::idx(i, W);
                let min_val_in_row = *self.0[start..limit].iter().min().unwrap();
                // dbg!((i, min_val_in_row));
                'next_col: for j in 0..W {
                    if self.0[Self::idx(i, j)] == min_val_in_row {
                        // If min_val_in_row is greater than or equal to all elements of column,
                        // then we have found a saddle point.
                        let hope_max_in_col = min_val_in_row;
                        for i in 0..H {
                            let idx = Self::idx(i, j);
                            if self.0[idx] > hope_max_in_col {
                                // dbg!((i, j, idx, self.0[idx], hope_max_in_col));
                                continue 'next_col;
                            }
                        }
                        return Some((i, j));
                    }
                }
            }

            return None;
        }
    }
    struct AltSaddlePoint(NineByEight);
    impl SaddlePoint for AltSaddlePoint {
        fn saddle_point(&self) -> Option<(u8, u8)> {
            let find_col_max = |j| -> u32 {
                let mut col_max = self.0.0[NineByEight::idx(0, j)];
                for i in 1..H {
                    let val = self.0.0[NineByEight::idx(i, j)];
                    if val > col_max {
                        col_max = val;
                    }
                }
                col_max
            };
            let min_of_col_max = {
                let mut min_of_col_max = find_col_max(0);
                for j in 1..W {
                    let col_max = find_col_max(j);
                    if col_max < min_of_col_max {
                        min_of_col_max = col_max;
                    }
                }
                min_of_col_max
            };
            let find_row_min = |i| -> (u8, u32) {
                let mut row_min = (0, self.0.0[NineByEight::idx(i, 0)]);
                for j in 1..W {
                    let val = self.0.0[NineByEight::idx(i, j)];
                    if val < row_min.1 {
                        row_min = (j, val);
                    }
                }
                row_min
            };
            for i in 0..H {
                let (j, row_min) = find_row_min(i);
                if row_min == min_of_col_max {
                    return Some((i, j));
                }
            }
            return None;
        }
    }

    macro_rules! test_both {
        (($nxe: expr).saddle_point(), $expect: expr) => {
            {
                let nxe = $nxe;
                assert_eq!(nxe.saddle_point(), $expect);
                assert_eq!(AltSaddlePoint(nxe).saddle_point(), $expect);
            }
        }
    }

    #[test]
    fn simple_saddle_point_upper_left() {
        let simple = NineByEight([1, 2, 2, 2, 2, 2, 2, 2,
                                  0, 0, 0, 0, 0, 0, 0, 0,
                                  0, 0, 0, 0, 0, 0, 0, 0,
                                  0, 0, 0, 0, 0, 0, 0, 0,
                                  0, 0, 0, 0, 0, 0, 0, 0,
                                  0, 0, 0, 0, 0, 0, 0, 0,
                                  0, 0, 0, 0, 0, 0, 0, 0,
                                  0, 0, 0, 0, 0, 0, 0, 0,
                                  0, 0, 0, 0, 0, 0, 0, 0,]);
        assert_eq!(simple, NineByEight::build(|i, j| {
            match (i, j) {
                (0, 0) => 1,
                (0, _) => 2,
                _ => 0,
            }
        }));

        assert_eq!(simple.saddle_point(), Some((0, 0)));
        assert_eq!(AltSaddlePoint(simple).saddle_point(), Some((0, 0)));
        test_both!((simple).saddle_point(), Some((0, 0)));
    }

    #[test]
    fn still_simple_but_near_middle() {
        test_both!((NineByEight::build(|i, j| {
            match (i, j) {
                (3, 4) => 10,
                (3, _) => 20,
                _ => 0,
            }
        })).saddle_point(), Some((3, 4)));
    }

    #[test]
    fn cases_with_no_saddle_point() {
        // Check some cases where there is no saddle point
        test_both!((NineByEight::build(|i, j| {
            if i == j { 10 } else { 0 }
        })).saddle_point(), None);
        let delta = NineByEight::build(|i, j| {
                (if i > j { i - j } else { j - i }) as u32
        });
        // dbg!(&delta);
        test_both!((delta).saddle_point(), None);
    }

    #[test]
    fn constant_matrix_has_saddle_points_everywhere() {
        // Interesting corner case: a matrix with all the same value has *every*
        // position as a saddle point.
        test_both!((NineByEight::build(|i, j| 0)).saddle_point(), Some((0, 0)));
    }
}

pub mod ex_13 {
    use std::fmt::Write;

    // Knuth got to assume his character set was reasoanbly small for the MIX
    // version. I should look into what their plans are for the MMIX version.
    //
    // Oooooh:
    //
    // > For efficiency, “buffer” the input: While reading a block into one area
    // > of memory you can be counting characters from another area.
    //
    // My reading of that is: use async IO! But is that silly? Ha hah. Well lets
    // try a sync version first at least.

    pub fn frequency_count_sync(bytes: impl Iterator<Item=u8>) -> String {
        const LEN: usize = (u8::MAX as usize) + 1;
        let mut table: [u64; LEN] = [0; LEN];
        for b in bytes {
            // the number of blanks should not be counted.
            if b as char == ' ' { continue; }
            table[b as usize] += 1;
        }
        let mut buf = String::new();
        for i in 0..u8::MAX {
            if table[i as usize] > 0 {
                writeln!(buf, "{}\t{:07}", i as char, table[i as usize]);
            }
        }
        buf
    }
}
