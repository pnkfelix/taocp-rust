mod exact_cover;

struct Df(uint);
struct Cf(uint);

enum DC { Ddc(Df), Cdc(Cf), }
enum CR { Ccr(Cf), Rootcr, }

struct DataObj { L: Df, R: Df, U: DC, D: DC, C: Cf }
struct ColumnObj<Label> { L: CR, R: CR, U: DC, D: DC, C: Cf, S: uint, N: Label }
struct RootObj { L: CR, R: CR }

struct dlx_matrix<L> {
    data: ~[DataObj],
    cols: ~[ColumnObj<L>],
    root: RootObj,
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
            data: ~[], cols: ~[], root: RootObj { L: Rootcr, R: Rootcr }
        };

        let mut col_labels = ~[];
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
            let init_ptrs : ~[Option<Df>] = ~[];
            let mut ptrs = Matrix {
                col_indent: ~"",
                cols: col_labels,
                rows: ~[],
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

fn main() {
    let input = exact_cover::simple_exact_cover_instance_2();
    println!("Hello world input: {}", input);
    let m = dlx_matrix::new(&input);
    println!("yields {:?}", m);
}
