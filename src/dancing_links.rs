#![feature(phase)]
#[phase(plugin, link)] extern crate log;
extern crate debug;

use std::fmt;

mod exact_cover;

/// A Df is a reference to a DataObj owned by the matrix.
#[deriving(PartialEq)]
struct Df(uint);

/// A Cf is a reference to a ColumnObj owned by the matrix.
#[deriving(PartialEq)]
struct Cf(uint);

/// A DC is reference to either a DataObj or a ColumnObj.
#[deriving(PartialEq)]
enum DC { Ddc(Df), Cdc(Cf), }

/// A CR is a reference to either a ColumnObj or the Root sentinel node.
#[deriving(PartialEq)]
enum CR { Ccr(Cf), Rootcr, }

struct DataObj { L: Df, R: Df, U: DC, D: DC, C: Cf }
struct ColumnObj<Label> { L: CR, R: CR, U: DC, D: DC, C: Cf, S: uint, N: Label }
struct RootObj { L: CR, R: CR }

struct dlx_matrix<L> {
    data: Vec<DataObj>,
    cols: Vec<ColumnObj<L>>,
    root: RootObj,
}

impl Df {
    fn L<LBL>(&Df(i): &Self, m: &dlx_matrix<LBL>) -> Df { m.data[i].L }
    fn R<LBL>(&self, m: &dlx_matrix<LBL>) -> Df { m.data[**self].R }
    fn D<LBL>(&self, m: &dlx_matrix<LBL>) -> DC { m.data[**self].D }
    fn U<LBL>(&self, m: &dlx_matrix<LBL>) -> DC { m.data[**self].U }
    fn C<LBL>(&self, m: &dlx_matrix<LBL>) -> Cf { m.data[**self].C }
}

impl Cf {
    fn L<LBL>(&self, m: &dlx_matrix<LBL>) -> CR { m.cols[**self].L }
    fn R<LBL>(&self, m: &dlx_matrix<LBL>) -> CR { m.cols[**self].R }
    fn D<LBL>(&self, m: &dlx_matrix<LBL>) -> DC { m.cols[**self].D }
    fn U<LBL>(&self, m: &dlx_matrix<LBL>) -> DC { m.cols[**self].U }
    fn S<'a, LBL>(&self, m: &'a mut dlx_matrix<LBL>) -> &'a mut uint { &mut m.cols[**self].S }
}

trait LRLinked<Ctxt, Rf> {
    fn update_l(&self, m: &mut Ctxt, new_l: Rf);
    fn update_r(&self, m: &mut Ctxt, new_r: Rf);
}

trait UDLinked<Ctxt, Rf> {
    fn update_u(&self, m: &mut Ctxt, new_u: Rf);
    fn update_d(&self, m: &mut Ctxt, new_d: Rf);
}

trait QuadLinked<Ctxt, LRf, UDf> : LRLinked<Ctxt, LRf>
                                 + UDLinked<Ctxt, UDf>
{
}

impl<L> LRLinked<dlx_matrix<L>, CR> for CR {
    fn update_l(&self, m: &mut dlx_matrix<L>, new_l: CR) {
        match self {
            &Rootcr     => { m.root.L = new_l; }
            &Ccr(Cf(i)) => { m.cols[i].L = new_l; }
        }
    }
    fn update_r(&self, m: &mut dlx_matrix<L>, new_r: CR) {
        match self {
            &Rootcr     => { m.root.R = new_r; }
            &Ccr(Cf(i)) => { m.cols[i].R = new_r; }
        }
    }
}
impl<L> UDLinked<dlx_matrix<L>, DC> for CR {
    fn update_u(&self, m: &mut dlx_matrix<L>, new_u: DC) {
        match self {
            &Rootcr     => fail!(),
            &Ccr(Cf(i)) => { m.cols[i].U = new_u; }
        }
    }
    fn update_d(&self, m: &mut dlx_matrix<L>, new_d: DC) {
        match self {
            &Rootcr     => fail!(),
            &Ccr(Cf(i)) => { m.cols[i].D = new_d; }
        }
    }
}
impl<L> UDLinked<dlx_matrix<L>, DC> for DC {
    fn update_u(&self, m: &mut dlx_matrix<L>, new_u: DC) {
        match self {
            &Ddc(Df(i)) => { m.data[i].U = new_u; }
            &Cdc(Cf(i)) => { m.cols[i].U = new_u; }
        }
    }
    fn update_d(&self, m: &mut dlx_matrix<L>, new_d: DC) {
        match self {
            &Ddc(Df(i)) => { m.data[i].D = new_d; }
            &Cdc(Cf(i)) => { m.cols[i].D = new_d; }
        }
    }
}

impl<L> LRLinked<dlx_matrix<L>, Df> for Df {
    fn update_l(&self, m: &mut dlx_matrix<L>, new_l: Df) {
        m.data[**self].L = new_l;
    }
    fn update_r(&self, m: &mut dlx_matrix<L>, new_r: Df) {
        m.data[**self].R = new_r;
    }
}

impl<L> LRLinked<dlx_matrix<L>, CR> for Cf {
    fn update_l(&self, m: &mut dlx_matrix<L>, new_l: CR) {
        m.cols[**self].L = new_l;
    }
    fn update_r(&self, m: &mut dlx_matrix<L>, new_r: CR) {
        m.cols[**self].R = new_r;
    }
}

impl<L> UDLinked<dlx_matrix<L>, DC> for Df {
    fn update_u(&self, m: &mut dlx_matrix<L>, new_u: DC) {
        m.data[**self].U = new_u;
    }
    fn update_d(&self, m: &mut dlx_matrix<L>, new_d: DC) {
        m.data[**self].D = new_d;
    }
}

impl<L:Clone,M:exact_cover::BitMatrix+exact_cover::ColLabelled<L>+exact_cover::RowLabelled<L>> dlx_matrix<L> {
    fn new(input: &M) -> dlx_matrix<L> {
        use exact_cover::Matrix;

        let mut m : dlx_matrix<L> = dlx_matrix::<L> {
            data: vec![], cols: vec![], root: RootObj { L: Rootcr, R: Rootcr }
        };

        let mut col_labels = vec![];
        for i in range(0, input.num_cols()) {
            let l : L = input.col_label(i).clone();
            col_labels.push(l.clone());

            //// Yucko, had to manually inline body of `prepend_empty_col`
            //// to placate type-checker.  Reduce to something small later.
            // m.prepend_empty_col(l);
            let idx = m.cols.len();
            let dc = Cdc(Cf(idx));
            let col_obj = ColumnObj {
                L: Rootcr,
                R: m.root.R,
                U: dc,
                D: dc,
                C: Cf(idx),
                S: 0,
                N: l
            };
            m.cols.push(col_obj);
            let root_r = m.root.R;
            let cr = Ccr(Cf(idx));
            root_r.update_l(&mut m, cr);
            Rootcr.update_r(&mut m, cr);
        }

        // As we scan the input, `ptrs` keeps track of how its
        // elements map to entries in the matrix `m` we are building.

        type Ptrs<L> = Matrix<L, Option<Df>>;
        let mut ptrs = {
            let init_ptrs : Vec<Option<Df>> = vec![];
            let mut ptrs = Matrix {
                col_indent: "".into_owned(),
                cols: col_labels,
                rows: vec![],
                elems: init_ptrs
            };

            for _ in range(0, input.num_rows()) {
                for _ in range(0, input.num_cols()) {
                    ptrs.elems.push(None); // placeholders
                }
            }

            ptrs
        };

        fn find<L>(m: &Ptrs<L>, col: uint, row: uint, dir: int) -> Option<Df> {
            let update = |x:uint| {
                let next = (x as int + dir);
                if next == -1 { m.cols.len() - 1 }
                    else if next as uint == m.cols.len() { 0u }
                    else { next as uint }
            };
            let mut cursor = update(col);
            while cursor != col {
                match m.at(cursor, row) {
                    &Some(df) => return Some(df),
                    &None     => {},
                }
                cursor = update(cursor);

            }
            return None;
        }

        for col in range(0, input.num_cols()) {
            debug!("transcribing col: {:?}", *input.col_label(col));
            let cf = Cf(col);
            let hdr = Cdc(cf);
            let mut last_in_col : DC = hdr;
            for row in range(0, input.num_rows()) {
                if input.at(col, row) {
                    debug!("transcribing entry: ({:?},{:?})", *input.col_label(col), *input.row_label(row));
                    let l = find(&ptrs, col, row, -1);
                    let r = find(&ptrs, col, row,  1);

                    debug!("entry: ({:?},{:?}) found l: {:?} r: {:?}",
                           *input.col_label(col), *input.row_label(row), l, r);

                    let df = Df(m.data.len());
                    let l = l.unwrap_or(df);
                    let r = r.unwrap_or(df);

                    let d = DataObj { L: l, R: r, U: last_in_col, D: hdr, C: cf };
                    debug!("entry: ({:?},{:?}) gets obj {:?}",
                           *input.col_label(col), *input.row_label(row), d);
                    let df = Df(m.data.len());
                    m.data.push(d);

                    l.update_r(&mut m, df);
                    r.update_l(&mut m, df);
                    last_in_col.update_d(&mut m, Ddc(df));
                    hdr.update_u(&mut m, Ddc(df));

                    ptrs.put(col, row, Some(df));
                    last_in_col = Ddc(df);
                }
            }
        }

        m
    }

    #[cfg(not_now)]
    fn prepend_empty_col(&mut self, label: L) -> Cf {
        let idx = self.cols.len();
        let dc = Cdc(Cf(idx));
        let col_obj = ColumnObj {
            L: Rootcr,
            R: self.root.R,
            U: dc,
            D: dc,
            C: Cf(idx),
            S: 0,
            N: label
        };
        self.cols.push(col_obj);
        let root_r = self.root.R;
        let cr = Ccr(Cf(idx));
        root_r.update_l(self, cr);
        Rootcr.update_r(self, cr);
        Cf(idx)
    }
}

struct Dlx<'a, L> {
    m: &'a mut dlx_matrix<L>,
    soln: Vec<Df>,
    count_updates: uint,
}

impl<'a, L:fmt::String> Dlx<'a, L> {

    fn cover(&mut self, c: Cf) {
        let new_l = c.L(self.m);
        c.R(self.m).update_l(self.m, new_l);
        let new_r = c.R(self.m);
        c.L(self.m).update_r(self.m, new_r);
        // self.count_updates += 1;

        let mut i = c.D(self.m);
        loop {
            match i {
                Cdc(_) => break,
                Ddc(id) => {
                    let mut j = id.R(self.m);
                    while j != id {
                        let new_u = j.U(self.m);
                        j.D(self.m).update_u(self.m, new_u);
                        let new_d = j.D(self.m);
                        j.U(self.m).update_d(self.m, new_d);
                        self.count_updates += 1;

                        *j.C(self.m).S(self.m) -= 1;

                        j = j.R(self.m);
                    }
                    i = id.D(self.m);
                }
            }
        }
    }

    fn uncover(&mut self, c: Cf) {
        let mut i = c.U(self.m);
        loop {
            match i {
                Cdc(_) => break,
                Ddc(id) => {
                    let mut j = id.L(self.m);
                    while j != id {
                        *j.C(self.m).S(self.m) += 1;
                        j.D(self.m).update_u(self.m, Ddc(j));
                        j.U(self.m).update_d(self.m, Ddc(j));
                        j = j.L(self.m);
                    }
                    i = id.U(self.m);
                }
            }
        }
        c.R(self.m).update_l(self.m, Ccr(c));
        c.L(self.m).update_r(self.m, Ccr(c));
    }

    fn search(&mut self,
              k: uint,
              select_col: &|&dlx_matrix<L>| -> Cf
) {
        if self.m.root.R == Rootcr {
            self.print_soln();
            return;
        }
        // Does the algorithm truly mutate `c` as it runs ...?
        let mut c = (*select_col)(self.m);
        self.cover(c);
        let mut r = c.D(self.m);
        loop {
            match r {
                Cdc(_) => break,
                Ddc(rd) => {
                    assert!(self.soln.len() >= k);
                    if self.soln.len() == k {
                        self.soln.push(rd);
                    } else {
                        self.soln[k] = rd;
                    }
                    let mut j = rd.R(self.m);
                    while j != rd {
                        let c = j.C(self.m);
                        self.cover(c);
                        j = j.R(self.m);
                    }
                    self.search(k+1, select_col);
                    rd = self.soln[k];
                    c = rd.C(self.m); // ... is this line supposed to be here?
                    let mut j = rd.L(self.m);
                    while j != rd {
                        let c = j.C(self.m);
                        self.uncover(c);
                        j = j.L(self.m);
                    }
                    r = rd.D(self.m);
                }
            }
        }
        self.uncover(c);
    }

    fn print_soln(&self) {
        for &d in self.soln.iter() {
            self.print_row_containing(d);
            println!("");
        }
        println!("");
    }

    fn print_row_containing(&self, d: Df) {
        let mut cursor = d;
        loop {
            let obj = self.m.data[*cursor];
            print!("{:s} ", self.m.cols[*cursor.C(self.m)].N);
            cursor = obj.R;
            if cursor == d {
                break;
            }
        }
    }
}

fn main() {
    let input = exact_cover::simple_exact_cover_instance_1();
    println!("Hello world input: {}", input);
    let mut m = dlx_matrix::new(&input);
    // println!("yields {:?}", m);
    let mut dlx = Dlx { m: &mut m, soln: vec![], count_updates: 0 };

    let trivial_col_choice = |m:&dlx_matrix<&'static str>| {
        match m.root.R {
            Ccr(cf) => cf,
            Rootcr  => fail!("should not choose col on empty matrix")
        }
    };

    let min_branching_col_choice = |m:&dlx_matrix<&'static str>| -> Cf {
        let mut j = m.root.R;
        let mut c = j;
        let mut s = None; // represents +infinity
        loop {
            match j {
                Rootcr => break,
                Ccr(jd) => {
                    match (m.cols[*jd].S, s) {
                        (s2, None)                => { s = Some(s2); c = j; }
                        (s2, Some(s1)) if s2 < s1 => { s = Some(s2); c = j; }
                        _ => {} }
                    j = jd.R(m);
                }
            }
        }

        match c {
            Ccr(cf) => cf,
            Rootcr  => fail!("should not choose col on empty matrix")
        }

    };

    // dlx.search(0, &trivial_col_choice);
    dlx.search(0, &min_branching_col_choice);
    println!("dlx.count_updates: {}", dlx.count_updates);
}
