#![feature(generators)]

use std::collections::HashMap;

use iterator_item::iterator_item;

fn main() {
    println!("Hello, world!");
    let mut seen = HashMap::new();
    for (j, cand) in latin_squares(3).enumerate() {
        let mut entries = seen.entry(cand.clone()).or_insert(Vec::new());
        entries.push(j);
        dbg!((j, cand, entries));
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
        let mut iter = cand.iter()
            .map(|(f,s)| {
                format!("{}{}",
                        char::from_u32('a' as u32 + *f as u32).unwrap(),
                        char::from_u32('α' as u32 + *s as u32).unwrap())
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
    fn* latin_squares(dim: u16) yields Vec<String> {
        dbg!(dim);
        assert!(dim <= 16); // assume for now we can fit one card in a u32
        let m = MatrixCxn { dim };
        let mut cand: Vec<(u16, u16)> = std::iter::repeat((0, 0)).take((dim * dim) as usize).collect();

        loop {
            if m.check_cand(&cand[..]) {
                yield m.to_cards(&cand);
            }
            // increment
            let mut i = cand.len() - 1;
            loop {
                if let Some(c2) = m.next(cand[i]) {
                    cand[i] = c2;
                    if !m.check_cand_at(&cand[..], i) {
                        // if we cannot locally pass with this, then just keep searching.
                        continue;
                    }
                    for j in i+1..cand.len() {
                        cand[j] = (0, 0);
                    }
                    break;
                } else if i == 0 {
                    return;
                } else {
                    i -= 1;
                }
            }
        }
    }
}
