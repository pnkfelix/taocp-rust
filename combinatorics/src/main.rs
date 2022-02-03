#![feature(generators)]

use iterator_item::iterator_item;

macro_rules! time_it {
    ($context:expr, $e:expr) => {
        {
            let timer = std::time::Instant::now();
            let e = $e;
            println!("{}: {:?}", $context, timer.elapsed());
            e
        }
    }
}


fn main() {
    for dim in 1..=5 {
        let count = time_it!(format!("LS({})", dim),
                             latin_squares(dim, true, true).count());
        dbg!((dim, count));
    }
}

struct MatrixCxn {
    dim: u16,
}

impl MatrixCxn {
    fn next(&self, c: (u16, u16)) -> Option<(u16, u16)> {
        let dim = self.dim;
        if c.1 < dim-1 {
            Some((c.0, c.1+1))
        } else if c.0 < dim-1 {
            Some((c.0+1, 0))
        } else {
            None
        }
    }
    fn row_col(&self, i:usize) -> (u16, u16) {
        let dim = self.dim;
        ((i / (dim as usize)) as u16, (i % (dim as usize)) as u16)
    }
    fn index(&self, r: u16, c: u16) -> usize {
        let dim = self.dim;
        (r * dim + c) as usize
    }

    fn check_cand_at(&self, cand: &[(u16, u16)], i: usize) -> bool {
        let (row, col) = self.row_col(i);
        // dbg!((i, row, col, cand[i]));
        if row > 0 {
            for r in 0..=row-1 {
                let j = self.index(r, col);
                // dbg!(("row", i, j, cand[i], cand[j]));
                if cand[i].0 == cand[j].0 ||
                    cand[i].1 == cand[j].1 {
                        return false;
                    }
            }
        }
        if col > 0 {
            for c in 0..=col-1 {
                let j = self.index(row, c);
                // dbg!(("col", i, j, cand[i], cand[j]));
                if cand[i].0 == cand[j].0 ||
                    cand[i].1 == cand[j].1 {
                        return false;
                    }
            }
        }

        return true;
    }

    fn check_cand(&self, cand: &[(u16, u16)]) -> bool {
        // dbg!(cand);
        for i in 0..cand.len() {
            for j in 0..i {
                if cand[i] == cand[j] {
                    return false;
                }
            }
            if !self.check_cand_at(cand, i) {
                return false;
            }
        }
        return true;
    }

    fn to_cards(&self, cand: &[(u16, u16)]) -> Vec<String> {
        let rank = ['A', 'K', 'Q', 'J'];
        let suit = ['s', 'h', 'd', 'c'];
        let mut iter = cand.iter()
            .map(|(f,s)| {
                format!("{}{}",
                        rank[*f as usize],
                        suit[*s as usize])
                        // char::from_u32('a' as u32 + *f as u32).unwrap(),
                        // char::from_u32('α' as u32 + *s as u32).unwrap())
                        // char::from_u32('0' as u32 + *s as u32).unwrap())
            });
        let mut rows = Vec::with_capacity(self.dim as usize);
        for _ in 0..self.dim {
            let mut row = Vec::with_capacity(self.dim as usize);
            for _ in 0..self.dim {
                row.push(iter.next().unwrap());
            }
            rows.push(row.join(","));
        }
        rows
    }
}

iterator_item! { *
    fn* latin_squares(dim: u16, fixed_top: bool, fixed_left: bool) yields Vec<String> {
        dbg!(dim);
        assert!(dim <= 16); // assume for now we can fit one card in a u32
        let m = MatrixCxn { dim };

        let mut cand: Vec<(u16, u16)> = std::iter::repeat((0,0)).take((dim * dim) as usize).collect();

        let start;
        if fixed_top {
            // w.l.o.g. we can assume the first row is (j,j). (You can transform it
            // to the other solutions via renamings.)
            for j in 0..dim { cand[j as usize] = (j,j); }
            start = dim as usize;
        } else {
            start = 0;
        }

        let mut i;

        // every time we continue in this loop, cand[i] has just been
        // incremented.
        'cand_i_incr: loop {
            if m.check_cand(&cand[..]) {

                let mut left_col_sorted = true;
                for r in 0..dim {
                    if cand[m.index(r, 0)].0 != r {
                        left_col_sorted = false;
                    }
                }
                if !fixed_left || left_col_sorted {
                    yield m.to_cards(&cand);
                }
            }

            if start == cand.len() { return; }

            assert!(cand.len() > 0);
            i = cand.len() - 1;
            'try_incr: loop {

                if let Some(c2) = m.next(cand[i]) {
                    cand[i] = c2;

                    // if this candidate doesn't work locally,
                    // there's no reason to waste time searching
                    // through all the deeper nodes.
                    if !m.check_cand_at(&cand[..], i) {
                        continue 'try_incr;
                    }

                    for j in i+1..cand.len() {
                        cand[j] = (0,0);
                    }
                    continue 'cand_i_incr;
                }

                // if `m.next` returns None, then we've exhausted the search
                // space for this item. If its not the root, then reset it to
                // zero and move one step backwards (effectively backtracking in
                // graph)
                if i == start {
                    return;
                }

                i -= 1;
            }
        }
    }
}
