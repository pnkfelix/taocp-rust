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

trait QuadLinked<Ctxt, Lf, Rf, Uf, Df> {
    fn update_l(&self, m: &mut Ctxt, new_l: Lf);
    fn update_r(&self, m: &mut Ctxt, new_r: Rf);
    fn update_u(&self, m: &mut Ctxt, new_u: Uf);
    fn update_d(&self, m: &mut Ctxt, new_d: Df);
}

impl<L> QuadLinked<dlx_matrix<L>, CR, CR, DC, DC> for CR {
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

impl<L:Clone,M:exact_cover::BitMatrix+exact_cover::ColLabelled<L>> dlx_matrix<L> {
    fn new(input: &M) -> dlx_matrix<L> {
        let mut m : dlx_matrix<L> = dlx_matrix::<L> {
            data: ~[], cols: ~[], root: RootObj { L: Rootcr, R: Rootcr }
        };

        for i in range(0, input.num_cols()) {
            let l : L = input.col_label(i).clone();

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

        m
    }

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
    println!("Hello world");
}
