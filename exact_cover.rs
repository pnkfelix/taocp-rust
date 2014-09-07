#![allow(dead_code)]

use std::cmp::min;
use std::fmt;

trait TakeIterOn<T, A> {
    fn go<I:Iterator<T>>(&self, iter: &mut I) -> A;
}

struct Accumulate;
impl<T:Clone> TakeIterOn<T, Vec<T>> for Accumulate {
    fn go<I:Iterator<T>>(&self, iter: &mut I) -> Vec<T> {
        iter.collect()
    }
}

pub trait BitMatrix {
    fn num_cols(&self) -> uint;
    fn num_rows(&self) -> uint;
    fn at(&self, col: uint, row: uint) -> bool;
    fn without_row(&self, row: uint) -> Self;
    fn without_col(&self, col: uint) -> Self;

    fn is_covered(&self) -> bool {
        self.num_cols() == 0
    }
    fn rows_on<'a, A, K: TakeIterOn<uint, A>>(&'a self, col: uint, k: &K) -> A {
        // println!("col: {}, num_cols: {}", col, self.num_cols());
        assert!(col < self.num_cols());
        let mut i = GenericRowIter{ mat: self, col: col, cursor: 0 };
        k.go(&mut i)
    }
    fn cols_on<'a, A, K: TakeIterOn<uint, A>>(&'a self, row: uint, k: &K) -> A {
        assert!(row < self.num_rows());
        let mut i = GenericColIter{ mat: self, row: row, cursor: 0 };
        k.go(&mut i)
    }


    fn col(&self, col: uint) -> Vec<bool> {
        let mut accum = vec![];
        let r = self.num_rows();
        for row in range(0, r) {
            accum.push(self.at(col, row));
        }
        accum
    }
    fn row(&self, row: uint) -> Vec<bool> {
        let mut accum = vec![];
        let c = self.num_cols();
        for col in range(0, c) {
            accum.push(self.at(col, row));
        }
        accum
            }
        }

pub trait ColLabelled<L> { fn col_label<'a>(&'a self, col: uint) -> &'a L; }
pub trait RowLabelled<L> { fn row_label<'a>(&'a self, row: uint) -> &'a L; }

struct GenericColIter<'a, M> { mat: &'a M, row: uint, cursor: uint }
struct GenericRowIter<'a, M> { mat: &'a M, col: uint, cursor: uint }

impl<'a, M:BitMatrix> Iterator<uint> for GenericColIter<'a, M> {
    fn next(&mut self) -> Option<uint> {
        assert!(self.row < self.mat.num_rows());
        let num_cols = self.mat.num_cols();
        loop {
            if self.cursor >= num_cols { return None; }
            if self.mat.at(self.cursor, self.row) {
                let col = self.cursor;
                self.cursor += 1;
                return Some(col);
            } else {
                self.cursor += 1;
            }
        }
    }
}

impl<'a, M:BitMatrix> Iterator<uint> for GenericRowIter<'a, M> {
    fn next(&mut self) -> Option<uint> {
        assert!(self.col < self.mat.num_cols());
        let num_rows = self.mat.num_rows();
        loop {
            if self.cursor >= num_rows { return None; }
            if self.mat.at(self.col, self.cursor) {
                let row = self.cursor;
                self.cursor += 1;
                return Some(row);
            } else {
                self.cursor += 1;
            }
        }
    }
}

trait MutBitMatrix : BitMatrix {
    fn put(&mut self, col: uint, row: uint, val: bool);
}

#[deriving(Clone)]
pub struct Matrix<L, T> {
    col_indent: String,
    cols: Vec<L>,
    rows: Vec<L>,
    elems: Vec<T> }

impl<L:fmt::String, T> RowLabelled<L> for Matrix<L, T> {
    fn row_label<'a>(&'a self, row: uint) -> &'a L { self.rows.get(row) }
}

impl<L:fmt::String, T> ColLabelled<L> for Matrix<L, T> {
    fn col_label<'a>(&'a self, col: uint) -> &'a L { self.cols.get(col) }
}

impl<L:fmt::String, T:fmt::Show> fmt::Show for Matrix<L, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let obj = self;
        let mut line   = 0u;
        let mut cursor = 0;
        write!(f.buf, "\n{:s}( ", obj.col_indent);
        for l in obj.cols.iter() {
            write!(f.buf, "{:s} ", *l);
        }
        let width = obj.cols.len();
        while cursor < obj.elems.len() {
            write!(f.buf, "|\n{:s}| ", *obj.rows.get(line));
            for e in obj.elems.slice(cursor, cursor + width).iter() {
                write!(f.buf, "{} ", *e);
            }
            cursor = cursor + width;
            line = line + 1;
        }
        write!(f.buf, ")");
    }
}

impl<L, T> Matrix<L, T> {
    pub fn at<'a>(&'a self, col: uint, row: uint) -> &'a T {
        let width = self.cols.len();
        let rw = row * width;
        let len = self.elems.len();
        assert!(rw + col < len);
        self.elems.get((row * width) + col)
    }

    pub fn put(&mut self, col: uint, row: uint, t: T) {
        let width = self.cols.len();
        *self.elems.get((row * width) + col) = t;
    }
}

trait ToBool {
    fn to_bool(&self) -> bool;
}

impl ToBool for bool { fn to_bool(&self) -> bool { *self } }
impl ToBool for uint { fn to_bool(&self) -> bool { *self != 0u } }
impl ToBool for int  { fn to_bool(&self) -> bool { *self != 0 } }

impl<L:Clone, B:ToBool+Clone> BitMatrix for Matrix<L, B> {
    fn num_cols(&self) -> uint { self.cols.len() }
    fn num_rows(&self) -> uint { self.elems.len() / self.num_cols() }
    fn at(&self, col: uint, row: uint) -> bool { self.at(col, row).to_bool() }
    fn without_row(&self, row: uint) -> Matrix<L, B> {
        assert!(row < self.num_rows());
        let lft = self.elems.slice_to(row * self.num_cols());
        let rgt = self.elems.slice_from((row+1) * self.num_cols());
        let mut rows = self.rows.clone();
        rows.remove(row);
        Matrix { rows: rows,
                 col_indent: self.col_indent.clone(), cols: self.cols.clone(),
                 elems: Vec::from_slice(lft).append(rgt) }
    }
    fn without_col(&self, col: uint) -> Matrix<L, B> {
        let mut accum = vec![];
        let mut cursor = 0;
        let mut next_drop = col;
        let len = self.elems.len();
        while cursor < len {
            accum.push_all(self.elems.slice(cursor, next_drop));
            cursor = next_drop + 1;
            next_drop = min(len, next_drop + self.num_cols());
        }
        let mut cols = self.cols.clone();
        cols.remove(col);
        Matrix { rows: self.rows.clone(), 
                 col_indent: self.col_indent.clone(), cols: cols, elems: accum }
    }
}

mod x {
    use std::fmt;
    use super::BitMatrix;
    use super::RowLabelled;

    pub trait Solution<T> {
        fn include(&self, component: &T) -> Self;
    }
    impl<S:Str> Solution<S> for Vec<String> {
        fn include(&self, component: &S) -> Vec<String> {
            let mut v = self.clone();
            if !v.iter().any(|s|s.equiv(component)) {
                // let copy = component.into_owned();
                // Above does not work because `into_owned` takes self by-value
                let copy = component.as_slice().into_owned();
                v.push(copy);
            }

            return v;
        }
    }

    pub fn find_solutions<
        L:fmt::String+::std::str::Str, // work-around issue #8075
        M:Clone+BitMatrix+fmt::Show+RowLabelled<L>, S:Clone+Solution<L>>(
        a: &M,
        partial_soln: &S,
        select_col: &|&M| -> uint) -> Vec<S> {
        recur(0, a, partial_soln, select_col)
    }

    pub fn recur<
        L:fmt::String+::std::str::Str, // work-around issue #8075
        M:Clone+BitMatrix+fmt::Show+RowLabelled<L>,
        S:Clone+Solution<L>>(level: uint,
                             a: &M,
                             partial_soln: &S,
                             select_col: &|&M| -> uint) -> Vec<S> {

        let mut solns = vec![];
        let indent = "    ".repeat(level);

        if a.is_covered() { // problem is solved,
            debug!("{}a is_covered, soln: {:?}", indent, partial_soln);
            return vec![partial_soln.clone()]; // success.
        }
        let c = (*select_col)(a);
        let rows = a.rows_on(c, &super::Accumulate);
        for &r in rows.iter() {
            debug!("{}solving mat {} for (c,r) = ({},{:s})", indent, *a, c, *a.row_label(r));
            let partial = partial_soln.include(a.row_label(r));
            let cols = a.cols_on(r, &super::Accumulate);

            let mut a_new = a.clone();
            // println!("{:s}  removing row {:s}", indent, *a_new.row_label(r));
            // a_new = a_new.without_row(r);
            for &j in cols.iter().rev() {
                debug!("{:s} removing column {:u} due to it being covered by row {:s}", indent, j, *a.row_label(r));

                let rows = a_new.rows_on(j, &super::Accumulate);
                for &i in rows.iter().rev() {
                    if i == r {
                        debug!("{:s}  removing row {:s}", indent, *a_new.row_label(i));
                    } else {
                        debug!("{:s}  removing row {:s} (as it collides with row {:s} on column {:u})", indent, *a_new.row_label(i), *a.row_label(r), j);
                    }
                    a_new = a_new.without_row(i);
                }

                a_new = a_new.without_col(j);
                debug!("{:s} removing col {:u} yielded {}", indent, j, a_new);

            }
            let sub = recur(level + 1, &a_new, &partial, select_col);
            solns.push_all(sub.as_slice());
        }

        return solns;
    }
}

fn choose_nonzero_col<M:BitMatrix>(m: &M) -> Option<uint> {
    for c in range(0, m.num_cols()) {
        for r in range(0, m.num_rows()) {
            if m.at(c, r) { return Some(c); }
        }
    }
    return None;
}

pub fn simple_exact_cover_instance_1() -> Matrix<&'static str, uint> {
    Matrix {
        cols: vec!["a", "b", "c", "d", "e", "f", "g", ],
        col_indent: " ".into_owned(),
        rows: vec!["1", "2", "3", "4", "5", "6", ],
        elems: vec![0, 0, 1, 0, 1, 1, 0,
                    1, 0, 0, 1, 0, 0, 1,
                    0, 1, 1, 0, 0, 1, 0,
                    1, 0, 0, 1, 0, 0, 0,
                    0, 1, 0, 0, 0, 0, 1,
                    0, 0, 0, 1, 1, 0, 1,
                    ]
    }
}

fn solve_exact_cover_instance_1() {
    let m = simple_exact_cover_instance_1();
    let unconstrained_soln : Vec<String> = vec![];
    println!("simple_exact_cover_instance begin: {}", m);
    let solns = x::find_solutions(&m,
                                  &unconstrained_soln,
                                  &|m| { choose_nonzero_col(m).unwrap_or(0) } );
    println!("simple_exact_cover_instance solns: {:?}", solns);
}

pub fn simple_exact_cover_instance_2() -> Matrix<&'static str, uint> {
    // (This instance of the problem is not solvable; I was trying to debug
    //  why my transcription of Knuth's algorithm X goes wrong.  The bug ended
    //  up being that my defintiion of "A is empty" was based on the number of
    //  elements in the matrix, but the actual termination condition is the
    //  number of columns (which can exist independently of rows).  I would
    //  argue that this remains a deficiency in Knuth's presentation, but
    //  it is a pretty easy bug to fix once you see where it arises.)
    Matrix {
        rows:      vec!["   a", "   b"],
        col_indent: "    ".into_owned(),
        cols: vec![ "1", "2", "3"],
        elems: vec![0, 1, 1,
                    1, 1, 0, ]
    }
}

fn solve_exact_cover_instance_2() {
    let m = simple_exact_cover_instance_2();
    let unconstrained_soln : Vec<String> = vec![];
    println!("simple_exact_cover_instance begin: {}", m);
    let solns = x::find_solutions(&m,
                                  &unconstrained_soln,
                                  &|m| { choose_nonzero_col(m).unwrap_or(0) } );
    println!("simple_exact_cover_instance solns: {:?}", solns);
}

fn main() {
    simple_exact_cover_instance_1();
    simple_exact_cover_instance_2();
}
