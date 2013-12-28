use std::cmp::min;
use std::fmt;

trait TakeIterOn<T, A> {
    fn go<I:Iterator<T>>(&self, iter: &mut I) -> A;
}

struct Accumulate;
impl<T:Clone> TakeIterOn<T, ~[T]> for Accumulate {
    fn go<I:Iterator<T>>(&self, iter: &mut I) -> ~[T] {
        iter.collect()
    }
}

trait BitMatrix {
    fn num_cols(&self) -> uint;
    fn num_rows(&self) -> uint;
    fn at(&self, col: uint, row: uint) -> bool;
    fn without_row(&self, row: uint) -> Self;
    fn without_col(&self, col: uint) -> Self;

    fn is_empty(&self) -> bool {
        self.num_cols() == 0 || self.num_rows() == 0
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


    fn col(&self, col: uint) -> ~[bool] {
        let mut accum = ~[];
        let r = self.num_rows();
        for row in range(0, r) {
            accum.push(self.at(col, row));
        }
        accum
    }
    fn row(&self, row: uint) -> ~[bool] {
        let mut accum = ~[];
        let c = self.num_cols();
        for col in range(0, c) {
            accum.push(self.at(col, row));
        }
        accum
            }
        }

trait RowLabelled<L> { fn row_label<'a>(&'a self, row: uint) -> &'a L; }

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
struct Matrix<L, T> { width: uint, rows: ~[L], elems: ~[T] }

impl<L:fmt::String, T> RowLabelled<L> for Matrix<L, T> {
    fn row_label<'a>(&'a self, row: uint) -> &'a L {
        &'a self.rows[row]
    }
}

impl<L:fmt::String, T:fmt::Default> fmt::Default for Matrix<L, T> {
    fn fmt(obj: &Matrix<L, T>, f: &mut fmt::Formatter) {
        let mut line   = 0u;
        let mut cursor = 0;
        let mut opened = false;
        write!(f.buf, "\n{:s}( ", obj.rows[0]);
        while cursor < obj.elems.len() {
            if !opened { 
                opened = true;
            } else {
                write!(f.buf, "|\n{:s}| ", obj.rows[line]);
            }
            for e in obj.elems.slice(cursor, cursor + obj.width).iter() {
                write!(f.buf, "{} ", *e);
            }
            cursor = cursor + obj.width;
            line = line + 1;
        }
        write!(f.buf, ")");
    }
}

impl<L, T> Matrix<L, T> {
    fn at<'a>(&'a self, col: uint, row: uint) -> &'a T {
        let rw = row * self.width;
        let len = self.elems.len();
        if (rw + col) >= len {
            println!("row: {} * width: {} (= {}) + col: {} (= {}) len: {}",
                     row, self.width, rw, col, rw+col, len);
        }
        &self.elems[(row * self.width) + col]
    }
}

trait ToBool {
    fn to_bool(&self) -> bool;
}

impl ToBool for bool { fn to_bool(&self) -> bool { *self } }
impl ToBool for uint { fn to_bool(&self) -> bool { *self != 0u } }
impl ToBool for int  { fn to_bool(&self) -> bool { *self != 0 } }

impl<L:Clone, B:ToBool+Clone> BitMatrix for Matrix<L, B> {
    fn num_cols(&self) -> uint { self.width }
    fn num_rows(&self) -> uint { self.elems.len() / self.width }
    fn at(&self, col: uint, row: uint) -> bool { self.at(col, row).to_bool() }
    fn without_row(&self, row: uint) -> Matrix<L, B> {
        assert!(row < self.num_rows());
        let elems = self.elems.clone();
        let lft = self.elems.slice_to(row * self.width);
        let rgt = self.elems.slice_from((row+1) * self.width);
        let mut rows = self.rows.clone();
        rows.remove(row);
        Matrix { width: self.width, rows: rows, elems: lft + rgt }
    }
    fn without_col(&self, col: uint) -> Matrix<L, B> {
        let mut accum = ~[];
        let mut cursor = 0;
        let mut next_drop = col;
        let len = self.elems.len();
        while cursor < len {
            accum.push_all(self.elems.slice(cursor, next_drop));
            cursor = next_drop + 1;
            next_drop = min(len, next_drop + self.width);
        }
        Matrix { width: self.width - 1, rows: self.rows.clone(), elems: accum }
    }
}

mod x {
    use std::fmt;
    use super::BitMatrix;
    use super::RowLabelled;

    pub trait Solution<T> {
        fn include(&self, component: &T) -> Self;
    }
    impl<S:Str> Solution<S> for ~[~str] {
        fn include(&self, component: &S) -> ~[~str] {
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
        M:Clone+BitMatrix+fmt::Default+RowLabelled<L>, S:Clone+Solution<L>>(
        a: &M,
        partial_soln: &S,
        select_col: &|&M| -> uint) -> ~[S] {
        recur(0, a, partial_soln, select_col)
    }

    pub fn recur<
        L:fmt::String+::std::str::Str, // work-around issue #8075
        M:Clone+BitMatrix+fmt::Default+RowLabelled<L>,
        S:Clone+Solution<L>>(level: uint,
                             a: &M,
                             partial_soln: &S,
                             select_col: &|&M| -> uint) -> ~[S] {

        let mut solns = ~[];
        let indent = "    ".repeat(level);

        if a.is_empty() { // problem is solved,
            println!("{}a is_empty, soln: {:?}", indent, partial_soln);
            return ~[partial_soln.clone()]; // success.
        }
        let c = (*select_col)(a);
        let rows = a.rows_on(c, &super::Accumulate);
        for &r in rows.iter() {
            println!("{}solving mat {} for (c,r) = ({},{:s})", indent, *a, c, *a.row_label(r));
            let partial = partial_soln.include(a.row_label(r));
            let cols = a.cols_on(r, &super::Accumulate);

            let mut a_new = a.clone();
            // println!("{:s}  removing row {:s}", indent, *a_new.row_label(r));
            // a_new = a_new.without_row(r);
            for &j in cols.iter().invert() {
                println!("{:s} removing column {:u} due to it being covered by row {:s}", indent, j, *a.row_label(r));

                let rows = a_new.rows_on(j, &super::Accumulate);
                for &i in rows.iter().invert() {
                    if i == r {
                        println!("{:s}  removing row {:s}", indent, *a_new.row_label(i));
                    } else {
                        println!("{:s}  removing row {:s} (as it collides with row {:s} on column {:u})", indent, *a_new.row_label(i), *a.row_label(r), j);
                    }
                    a_new = a_new.without_row(i);
                }

                if a_new.is_empty() {
                    println!("{:s} removing rows for col {:u} yielded empty", indent, j);
                } else {
                    a_new = a_new.without_col(j);
                    println!("{:s} removing col {:u} yielded {}", indent, j, a_new);
                }

            }
            let sub = recur(level + 1, &a_new, &partial, select_col);
            solns.push_all(sub);
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

fn simple_exact_cover_instance_1() {
    let m = Matrix {
        width: 7,
        rows: ~["1", "2", "3", "4", "5", "6", "fake"],
        elems: ~[0, 0, 1, 0, 1, 1, 0,
                 1, 0, 0, 1, 0, 0, 1,
                 0, 1, 1, 0, 0, 1, 0,
                 1, 0, 0, 1, 0, 0, 0,
                 0, 1, 0, 0, 0, 0, 1,
                 0, 0, 0, 1, 1, 0, 1,
                 0, 0, 0, 0, 0, 0, 0, // fake all-zero row (see note below).
                 ]
    };
    let unconstrained_soln : ~[~str] = ~[];
    println!("simple_exact_cover_instance begin: {}", m);
    let solns = x::find_solutions(&m,
                                  &unconstrained_soln,
                                  &|m| { choose_nonzero_col(m).unwrap_or(0) } );
    println!("simple_exact_cover_instance solns: {:?}", solns);
}

fn simple_exact_cover_instance_2() {
    // (This instance of the problem is not solvable; I am trying to debug
    //  why my transcription of Knuth's algorithm X goes wrong.  I suspect
    //  he left out a number of details about where it should terminate
    //  unsuccessfully.)
    let m = Matrix {
        width: 3,
        rows: ~["fake", "   a", "   b"],
        elems: ~[0, 0, 0, // a fake all-zero row to work-around bug in my
                          // understanding of the algorithm.  (Removing all
                          // rows should not be interpreted as an empty matrix,
                          // because that would not necessarily cover all
                          // columns.)
                 0, 1, 1, 
                 1, 1, 0, ]
    };
    let unconstrained_soln : ~[~str] = ~[];
    println!("simple_exact_cover_instance begin: {}", m);
    let solns = x::find_solutions(&m,
                                  &unconstrained_soln,
                                  &|m| { choose_nonzero_col(m).unwrap_or(0) } );
    println!("simple_exact_cover_instance solns: {:?}", solns);
}

fn main() {
    simple_exact_cover_instance_1();
    // simple_exact_cover_instance_2();
}
