```rust
use std::borrow::Cow;
use std::convert::Into;
use std::mem;

// This `Bigit` definition is to allow me to swap in
// another type like `u8` or `u16` for the purpose of
// sanity-checking the logic for when the bitvec overflows
// without having to wait to visit over 2^32 cases.
use std::u32 as bigit;
pub type Bigit = u32;

pub enum Control<B> { Break(B), Yield }

impl<B> Into<Control<B>> for () {
    fn into(self) -> Control<B> { Control::Yield }
}

pub trait Generator {
    type Item: ?Sized;
    type Final;
    fn gen<F, R>(&mut self, visit: F) -> Self::Final
        where F: for <'a> FnMut(Cow<'a, Self::Item>) -> R,
              R: Into<Control<Self::Final>>;
}


#[derive(PartialEq, Eq, Debug)]
pub struct LexicoBitVecs { seek: Bigit, state: Vec<Bigit>, }

fn width() -> usize {
    mem::size_of::<Bigit>() * 8
}

fn word_len(n: usize) -> usize {
    (n + (width()-1))/width()
}

fn word_idx(n: usize) -> (usize, usize) {
    (n / width(), n % width())
}

fn zero_bigits(n: usize) -> Vec<Bigit> {
    let len = word_len(n);
    let init_state = (0..len).map(|_|0).collect();
    init_state
}

fn lexicographic(n: usize) -> LexicoBitVecs {
    let rem = n % width();
    let seek = if rem == 0 {
        bigit::MAX
    } else {
        bigit::MAX >> (width()-rem)
    };
    // println!("n: {} width: {} rem: {}", n, width, rem);
    LexicoBitVecs { seek: seek, state: zero_bigits(n) }
}

impl LexicoBitVecs {
    fn done(&self) -> bool {
        let mut i = self.state.iter().rev();
        if *i.next().unwrap() != self.seek {
            return false;
        }
        for &w in i {
            if w != bigit::MAX { return false; }
        }
        return true;
    }

    fn increment_when_not_done(&mut self) {
        for w in &mut self.state {
            if *w < bigit::MAX {
                *w += 1;
                break;
            } else {
                *w = 0;
            }
        }
    }
}

impl Generator for LexicoBitVecs {
    type Item = [Bigit];
    type Final = ();
    fn gen<F, R>(&mut self, mut visit: F)
        where F: for <'a> FnMut(Cow<'a, Self::Item>) -> R,
              R: Into<Control<()>>
    {
        loop {
            let call_result =
                visit(Cow::Borrowed(&self.state[..]));
            match call_result.into() {
                Control::Break(()) => return,
                Control::Yield => ()
            }
            if self.done() { break; }
            self.increment_when_not_done();
        }
    }
}

#[cfg(test)]
use self::bitvecs::lexicographic as bitvecs;

#[test]
fn bitvecs_n01_init() {
    assert_eq!(lexicographic(1),
               LexicoBitVecs { seek: 0b_1, state: vec![0] })
}

#[test]
fn bitvecs_n01() {
    let mut results: Vec<Vec<Bigit>> = vec![];
    bitvecs(1).gen(|v| results.push(v.into_owned()));
    assert_eq!(results, [[0], [1]]);
}

#[test]
fn bitvecs_n02_init() {
    assert_eq!(lexicographic(2),
               LexicoBitVecs { seek: 0b_11, state: vec![0] });
}

#[test]
fn bitvecs_n02() {
    let mut results: Vec<Vec<Bigit>> = vec![];
    bitvecs(2).gen(|v| results.push(v.into_owned()));
    assert_eq!(results, [[0], [1], [2], [3]]);
}

#[test]
fn bitvecs_n15() {
    if mem::size_of::<Bigit>() == 1 {
        let last_bigit = 0b_11_11111;
        assert_eq!(lexicographic(15),
                   LexicoBitVecs { seek: last_bigit,
                             state: vec![0, 0] });
        let mut count = 0;
        let mut last_result = vec![];
        bitvecs(15).gen(|v| {
            count += 1;
            last_result = v.into_owned()
        });
        assert_eq!(count, 1 << 15);
        let ones = 0b1111_1111;
        assert_eq!(last_result, vec![ones, last_bigit]);
    } else {
        let last_bigit = 0b_11111_11111_11111_u16 as Bigit;
        assert_eq!(lexicographic(15),
                   LexicoBitVecs { seek: last_bigit,
                             state: vec![0] });
        let mut count = 0;
        let mut last_result = vec![];
        bitvecs(15).gen(|v| {
            count += 1;
            last_result = v.into_owned()
        });
        assert_eq!(count, 1 << 15);
        assert_eq!(last_result, vec![last_bigit]);
    }
}

#[cfg(feature="benchmarking")]
#[bench]
fn bitvecs_n33(b: &mut ::test::Bencher) {
    const K: usize = 22;
    b.iter(|| {
        let last_bigit = !0 >> (width() - (K % width())) as Bigit;
        let mut count: u64 = 0;
        let mut last_result = 0;
        bitvecs(K).gen(|v| {
            count += 1;
            last_result = *v.last().unwrap();
        });
        assert_eq!(count, 1 << K);
        assert_eq!(last_result, last_bigit);
    })
}

#[derive(PartialEq, Eq, Debug)]
pub struct Tuples { limits: Vec<u32>, state: Vec<u32>, }

pub fn tuples(limits: Vec<u32>) -> Tuples {
    let len = limits.len();
    let init_state = (0..len).map(|_|0).collect();
    Tuples { limits: limits, state: init_state }
}

impl Tuples {
    fn done_after_increment(&self) -> bool {
        for i in (0..self.limits.len()).rev() {
            debug_assert!(self.state[i] <= self.limits[i]);
            if self.state[i] != 0 {
                return false;
            }
        }
        return true;
    }

    fn increment(&mut self) {
        for i in (0..self.limits.len()) {
            let mut v = self.state[i];
            v += 1;
            if v == self.limits[i] {
                self.state[i] = 0;
            } else {
                self.state[i] = v;
                break;
            }
        }
    }
}

impl Generator for Tuples {
    type Item = [u32];
    type Final = ();
    fn gen<F, R>(&mut self, mut visit: F)
        where F: for <'a> FnMut(Cow<'a, Self::Item>) -> R,
              R: Into<Control<()>>
    {
        loop {
            let call_result =
                visit(Cow::Borrowed(&self.state[..]));
            match call_result.into() {
                Control::Break(()) => return,
                Control::Yield => ()
            }
            self.increment();
            if self.done_after_increment() { break; }
        }
    }
}

#[test]
fn tuples_t234() {
    let mut results: Vec<Vec<u32>> = vec![];
    tuples(vec![2,3,4]).gen(|v| results.push(v.into_owned()));
    assert_eq!(results, [[0, 0, 0], [1, 0, 0],
                         [0, 1, 0], [1, 1, 0],
                         [0, 2, 0], [1, 2, 0],

                         [0, 0, 1], [1, 0, 1],
                         [0, 1, 1], [1, 1, 1],
                         [0, 2, 1], [1, 2, 1],

                         [0, 0, 2], [1, 0, 2],
                         [0, 1, 2], [1, 1, 2],
                         [0, 2, 2], [1, 2, 2],

                         [0, 0, 3], [1, 0, 3],
                         [0, 1, 3], [1, 1, 3],
                         [0, 2, 3], [1, 2, 3],

                         ]);

}

#[test]
fn tuples_t432() {
    let mut results: Vec<Vec<u32>> = vec![];
    tuples(vec![4,3,2]).gen(|v| results.push(v.into_owned()));
    assert_eq!(results,
               [[0, 0, 0], [1, 0, 0], [2, 0, 0], [3, 0, 0],
                [0, 1, 0], [1, 1, 0], [2, 1, 0], [3, 1, 0],
                [0, 2, 0], [1, 2, 0], [2, 2, 0], [3, 2, 0],

                [0, 0, 1], [1, 0, 1], [2, 0, 1], [3, 0, 1],
                [0, 1, 1], [1, 1, 1], [2, 1, 1], [3, 1, 1],
                [0, 2, 1], [1, 2, 1], [2, 2, 1], [3, 2, 1],

                ]);
}

#[derive(PartialEq, Eq, Debug)]
pub struct GrayBitVecs {
    n: usize, parity_bit: u8, state: Vec<Bigit>
}

fn gray(n: usize) -> GrayBitVecs {
    GrayBitVecs { n: n, parity_bit: 0, state: zero_bigits(n) }
}

impl Generator for GrayBitVecs {
    type Item = [Bigit];
    type Final = ();
    fn gen<F, R>(&mut self, mut visit: F)
        where F: for <'a> FnMut(Cow<'a, Self::Item>) -> R,
              R: Into<Control<()>>
    {
        loop {
            let call_result =
                visit(Cow::Borrowed(&self.state[..]));
            match call_result.into() {
                Control::Break(()) => return,
                Control::Yield => ()
            }
            self.parity_bit = 1 - self.parity_bit;
            let j;
            if self.parity_bit == 1 {
                j = 0;
            } else {
                'a: loop {
                    for (i, &a) in self.state.iter().enumerate() {
                        for b in 0..width() {
                            if 0 != (a & (1 << b)) {
                                j = i * width() + b + 1;
                                break 'a;
                            }
                        }
                    }
                    panic!("parity bit ensures cannot happen");
                }
            }
            if j == self.n {
                break;
            } else {
                let (jw, ji) = word_idx(j);
                let mut w = self.state[jw];
                if 0 != (w & (1 << ji)) {
                    // a_j is 1; set to 0
                    w &= !(1 << ji)
                } else {
                    // a_j is 0; set to 1
                    w |=   1 << ji;
                }
                self.state[jw] = w;
            }
        }
    }
}

#[test]
fn gray_n4() {
    let mut results: Vec<Vec<Bigit>> = vec![];
    gray(4).gen(|v| results.push(v.into_owned()));
    assert_eq!(results, [[0b0000],
                         [0b0001],
                         [0b0011],
                         [0b0010],
                         [0b0110],
                         [0b0111],
                         [0b0101],
                         [0b0100],
                         [0b1100],
                         [0b1101],
                         [0b1111],
                         [0b1110],
                         [0b1010],
                         [0b1011],
                         [0b1001],
                         [0b1000]]);
}

pub mod bitvecs {
    pub enum BitVecs {
        Lexico(super::LexicoBitVecs)
    }
    #[inline]
    pub fn lexicographic(n: usize) -> BitVecs {
        BitVecs::Lexico(super::lexicographic(n))
    }
}

use self::bitvecs::BitVecs;

impl Generator for BitVecs {
    type Item = [Bigit];
    type Final = ();
    fn gen<F, R>(&mut self, visit: F) -> Self::Final
        where F: for <'a> FnMut(Cow<'a, Self::Item>) -> R,
              R: Into<Control<Self::Final>>
    {
        match *self {
            BitVecs::Lexico(ref mut b) => b.gen(visit),
        }
    }
}
```
