mod sect_1_3_2 {
    const H: u8 = 9;
    const W: u8 = 8;
    const LEN: usize = (H * W) as usize;
    #[derive(PartialEq, Eq)]
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
    #[test]
    fn ex_10() {
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
        assert_eq!(NineByEight::build(|i, j| {
            match (i, j) {
                (3, 4) => 10,
                (3, _) => 20,
                _ => 0,
            }
        }).saddle_point(), Some((3, 4)));

        // Check some cases where there is no saddle point
        assert_eq!(NineByEight::build(|i, j| {
            if i == j { 10 } else { 0 }
        }).saddle_point(), None);
        let delta = NineByEight::build(|i, j| {
                (if i > j { i - j } else { j - i }) as u32
        });
        // dbg!(&delta);
        assert_eq!(delta.saddle_point(), None);

        // Interesting corner case: a matrix with all the same value has *every*
        // position as a saddle point.
        assert_eq!(NineByEight::build(|i, j| 0).saddle_point(), Some((0, 0)));
    }
}

fn main() {
    println!("Hello, world!");
}
