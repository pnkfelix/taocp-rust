#![allow(dead_code, private_bounds, unused)]

trait SubU32: Sized + Ord + Eq + Copy {
    fn as_u32(&self) -> u32;
}

/// An impl of Domain represents a family of domains D_i of integers,
/// where each D_i knows what integers belong to its domain.
pub trait Domain: Sized {
    type D: SubU32;
    // Returns the number of domains `n` in the family.
    fn len(&self) -> usize;
    /// Returns the least integer for D_i.
    fn min_value(&self, i: usize) -> Self::D;
    /// Returns the next larger element of D_i (i.e. returns least y in D_i such that y > x).
    /// Requires `!is_max(x)`; otherwise returns D that may no longer be in valid range.
    ///
    /// Q: would it be simpler just to make this return Option<Self::D> ?
    fn next_larger(&self, x: Self::D, i: usize) -> Self::D;
    /// Returns the maximal integer for D_i.
    fn max_value(&self, i: usize) -> Self::D;
    /// Returns true iff x is least element for D_i
    fn is_min(&self, x: Self::D, i: usize) -> bool { x == self.min_value(i) }
    /// Returns true iff x is maximal element for D_i
    fn is_max(&self, x: Self::D, i: usize) -> bool { x == self.max_value(i) }
    /// Returns element of D_i corresponding to `x` if `x` is valid integer for D_i.
    /// Otherwise returns None.
    fn inject(&self, x: u32, i: usize) -> Option<Self::D>;
}
/// A Cutoff represents a family of properties P_k on sequences x_1 x_2 ... x_n,
/// for 1 <= k <= n and where x_k belongs to a domain D_k of integers, such that:
///
/// * P_n(x_1, x_2, ..., x_n) is goal property we seek to satisfy over the sequence x_1
/// * P_l(x_1, x_2, ..., x_l) for 1 <= l < n.
pub trait Cutoff {
    type Doms: Domain;
    fn doms(&self) -> &Self::Doms;
    fn test(&self, xs: &[<Self::Doms as Domain>::D]) -> bool;
}

pub fn basic_backtrack<C: Cutoff>(pred: &C, visit: impl Fn(&[<C::Doms as Domain>::D])) {
    let doms = pred.doms();
    let n = doms.len();
    let mut l = 1;
    let mut xs: Vec<<C::Doms as Domain>::D> = Vec::new();
    // B1 -> B2 -> B3 -> B4 -> B5 -> B4
    //       B2 -> B5
    //             B3 -> B2
    //                   B4 -> B3
    //                         B5 -> stop
    'b2: loop {
        if l > n {
            visit(&xs[..]);
        } else {
            xs[l-1] = doms.min_value(l);
        }
        'b3: loop {
            if l <= n && pred.test(&xs[0..(l-1)]) {
                l = l + 1;
                continue 'b2;
            }
            'b4: loop {
                if l <= n && !doms.is_max(xs[l-1], l)  {
                    xs[l-1] = doms.next_larger(xs[l-1], l);
                    continue 'b3;
                }
                // B5
                {
                    l = l - 1;
                    if l > 0 {
                        continue 'b4;
                    } else {
                        break 'b2;
                    }
                }
            }
        }
    }
}

mod n_queens {
    use super::{Cutoff, Domain, SubU32};
    pub struct NQueens { pub n: u32 }
    #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
    pub struct Col(u32);
    impl Col {
        fn diff(self, rhs: Col) -> u32 {
            self.0.abs_diff(rhs.0)
        }
    }
    impl std::ops::Sub<Col> for Col {
        type Output = u32;
        fn sub(self, rhs: Col) -> u32 {
            self.0 - rhs.0
        }
    }
    impl SubU32 for Col {
        fn as_u32(&self) -> u32 { self.0 }
    }
    impl Domain for NQueens {
        type D = Col;
        fn len(&self) -> usize { self.n as usize }
        fn inject(&self, x: u32, i: usize) -> Option<Self::D> {
            if x >= 1 && x <= self.n {
                Some(Col(x))
            } else {
                None
            }
        }
        fn min_value(&self, _: usize) -> Col { Col(1) }
        fn max_value(&self, _: usize) -> Col { Col(self.n) }
        fn next_larger(&self, x: Col, _: usize) -> Col {
            Col(x.0 + 1)
        }
    }

    impl Cutoff for NQueens {
        type Doms = NQueens;
        fn doms(&self) -> &NQueens { self }
        fn test(&self, xs: &[<Self::Doms as Domain>::D]) -> bool {
            for j in 0..xs.len() {
                for k in (j+1)..xs.len() {
                    let x_j = xs[j];
                    let x_k = xs[k];
                    if x_j == x_k { return false; }
                    if x_k.diff(x_j) != (k - j) as u32 { return false; }
                }
            }
            return true;
        }
    }
}

fn main() {
    let nq = n_queens::NQueens { n: 4 };
    basic_backtrack(&nq, |xs| {});
    println!("Hello, world!");
}
