trait LatinSquareElem<Latin, Greek> {
    fn latin(&self) -> Latin;
    fn greek(&self) -> Greek;
}

impl<L:Clone, G:Clone> LatinSquareElem<L, G> for (L, G) {
    fn latin(&self) -> L { let &(ref l, _) = self; l.clone() }
    fn greek(&self) -> G { let &(_, ref g) = self; g.clone() }
}

trait Square<Elem> {
    fn contents<'a>(&'a self) -> &'a [Elem];

    fn dim(&self) -> uint {
        for i in ::std::iter::range_inclusive(0, (self.contents().len()+1)/2) {
            if i*i == self.contents().len() { return i; }
        }
        fail!("non square.");
    }

    fn at<'a>(&'a self, row: uint, col: uint) -> &'a Elem {
        let dim = self.dim();
        &'a self.contents()[row*dim + col]
    }

    fn row_iter<'a>(&'a self, row: uint) -> RowIter<'a, Elem, Self> {
        RowIter { sq: self, dim: self.dim(), row: row, idx: 0 }
    }

    fn col_iter<'a>(&'a self, col: uint) -> ColIter<'a, Elem, Self> {
        ColIter { sq: self, dim: self.dim(), col: col, idx: 0 }
    }
}

struct LGSquare<Elem> {
    priv contents: ~[Elem] // invariant: length is n^2 for some n.
}

impl<Elem> Square<Elem> for LGSquare<Elem> {
    fn contents<'a>(&'a self) -> &'a [Elem] {
        self.contents.slice(0, self.contents.len())
    }
}

struct RowIter<'a, E, SQ> { sq: &'a SQ, dim: uint, row: uint, idx: uint }
struct ColIter<'a, E, SQ> { sq: &'a SQ, dim: uint, col: uint, idx: uint }

impl<'a, E, SQ: Square<E>> Iterator<&'a E> for RowIter<'a, E, SQ> {
    fn next(&mut self) -> Option<&'a E> {
        if self.idx < self.dim {
            let e = self.sq.at(self.row, self.idx);
            self.idx += 1;
            Some(e)
        } else {
            None
        }
    }
}

impl<'a, E, SQ: Square<E>> Iterator<&'a E> for ColIter<'a, E, SQ> {
    fn next(&mut self) -> Option<&'a E> {
        if self.idx < self.dim {
            let e = self.sq.at(self.idx, self.col);
            self.idx += 1;
            Some(e)
        } else {
            None
        }
    }
}

impl<L, G, E> LGSquare<E> {
    fn new(dim: uint,
           _l_alphabet: ~[L],
           _g_alphabet: ~[G],
           generate: |uint, uint| -> E) -> LGSquare<E> {
        let mut contents = ~[];
        for i in range(0, dim) {
            for j in range(0, dim) {
                contents.push(generate(i, j));
            }
        }
        LGSquare { contents: contents }
    }
}

impl<E> LGSquare<E> {
    fn swap(&mut self, i: uint, j:uint) {
        self.contents.swap(i, j);
    }
}

impl<L:Clone, G:Clone> Mul<LGSquare<G>, LGSquare<(L,G)>> for LGSquare<L> {
    fn mul(&self, rhs: &LGSquare<G>) -> LGSquare<(L,G)> {
        let lft = self.contents.clone().move_iter();
        let rgt = rhs .contents.clone().move_iter();
        LGSquare{ contents: lft.zip(rgt).collect() }
    }
}


impl<L:Eq, G:Eq, E:Eq+LatinSquareElem<L, G>> LGSquare<E> {
    fn is_latin(&self) -> bool {
        let len = self.contents().len();
        for i in range(0, len) {
            let e1 = &self.contents[i];
            for j in range(i, len) {
                let e2 = &self.contents[j];
                if e1 == e2 {
                    return false;
                }
            }
        }

        let dim = self.dim();
        for i in range(0, dim) {
            for j in range(0, dim) {
                let e1 = self.at(i, j);
                for k in range(j+1, dim) {
                    let e2 = self.at(i, k);
                    if e1.latin() == e2.latin() {
                        return false;
                    }
                    if e1.greek() == e2.greek() {
                        return false;
                    }
                }
                for l in range(i+1, dim) {
                    let e3 = self.at(l, j);
                    if e1.latin() == e3.latin() {
                        return false;
                    }
                    if e1.greek() == e3.greek() {
                        return false;
                    }
                }
            }
        }
        return true;
    }
}

trait ElemToStr {
    fn to_str(&self) -> ~str;
}

impl<X:ToStr,Y:ToStr> ElemToStr for (X,Y) {
    fn to_str(&self) -> ~str {
        let &(ref x, ref y) = self;
        x.to_str() + y.to_str()
    }
}

impl<E:ElemToStr> ToStr for LGSquare<E> {
    fn to_str(&self) -> ~str {
        let mut s = ~"";
        let dim = self.dim();
        for j in range(0, dim) {
            for e in self.row_iter(j) {
                s = s + " " + e.to_str();
            }
            s = s + "\n";
        }
        return s;
    }
}

#[deriving(Clone,Eq)]
struct El<L,G>(L, G);

impl<L:ToStr, G:ToStr> ToStr for El<L,G> {
    fn to_str(&self) -> ~str {
        let &El(ref l, ref g) = self;
        l.to_str() + g.to_str()
    }
}

impl<L:ToStr, G:ToStr> ElemToStr for El<L,G> {
    fn to_str(&self) -> ~str {
        fn to_str<X:ToStr>(x:&X) -> ~str { x.to_str() }
        to_str(self)
    }
}

impl<L:Clone,G:Clone> LatinSquareElem<L, G> for El<L, G> {
    fn latin(&self) -> L { let &El(ref l, _) = self; l.clone() }
    fn greek(&self) -> G { let &El(_, ref g) = self; g.clone() }
}

fn main3() {
    let suits = ~["♡", "♢", "♧"];
    let faces = ~["K", "Q", "J"];
    let mut s = LGSquare::new(3, suits.clone(), faces.clone(), |i, j| El(suits[i], faces[j]) );

    let status = || {
        println!("square: \n{:s}, dim: {}", s.to_str(), s.dim());
        println!("square.is_latin(): {}", s.is_latin());
        println!("");
    };
    status();
    s.swap(1, 5);
    s.swap(2, 7);
    s.swap(5, 8);
    s.swap(4, 8);
    s.swap(3, 5);
    s.swap(6, 8);
    s.swap(7, 8);
    status();
}

fn main4() {
    let latin = ~["a", "b", "c", "d"];
    let greek = ~["α", "β", "δ", "γ"];

    let left = LGSquare{ contents: ~["d", "a", "b", "c",
                                     "c", "b", "a", "d",
                                     "a", "d", "c", "b",
                                     "b", "c", "d", "a"] };
    let right = LGSquare{ contents: ~["γ", "δ", "β", "α",
                                      "β", "α", "γ", "δ",
                                      "α", "β", "δ", "γ",
                                      "̣δ", "γ", "α", "β"] };
    let mut s = left * right;

    let status = || {
        println!("square: \n{:s}, dim: {}", s.to_str(), s.dim());
        println!("square.is_latin(): {}", s.is_latin());
        println!("");
    };
    status();
    s.swap(1, 5);
    s.swap(2, 10);
    s.swap(3, 15);
    s.swap(4, 12);
    s.swap(7, 11);
    s.swap(9, 13);

    s.swap(4, 5);
    s.swap(6, 7);
    s.swap(8, 10);
    s.swap(9, 11);
    s.swap(12, 15);
    s.swap(13, 14);
    s.swap(4, 14);
    s.swap(8, 13);
    s.swap(12, 9);
    s.swap(9, 6);
    s.swap(14, 11);
    s.swap(10, 15);
    status();

    s = left * left;
    status();
}

mod natset;

fn find_tranversals_extending<
    E:Clone+Eq,
    SQ:Square<E>,
    SET:natset::AppNatSet<uint>>(s: &SQ,
                                 prefix: &[E],
                                 rows_taken: SET) -> ~[~[E]] {
    // `prefix` implicitly encodes columns taken (via its length) and elements taken (via its contents).  Arguably it also implicitly encodes the rows taken if I wanted to rebuild that from prefix on demand.

    let mut accum = ~[];

    let dim = s.dim();
    let new_col = prefix.len();
    for (idx, e) in s.col_iter(new_col).enumerate() {
        if !rows_taken.has(idx) && !prefix.contains(e) {
            let mut v = prefix.to_owned();
            v.push(e.clone());
            if new_col+1 == dim {
                accum.push(v);
            } else {
                let v = v.as_slice();
                let recur = |prefix_, rows_taken_| {
                    find_tranversals_extending(s, prefix_, rows_taken_)
                };

                let sub = recur(v, rows_taken.plus(idx));
                accum.push_all_move(sub);
            }
        }
    }

    return accum;
}

fn main10() {
    let L = LGSquare{ contents: ~[0, 1, 2, 3, 4, 5, 6, 7, 8, 9,
                                  1, 8, 3, 2, 5, 4, 7, 6, 9, 0,
                                  2, 9, 5, 6, 3, 0, 8, 4, 7, 1,
                                  3, 7, 0, 9, 8, 6, 1, 5, 2, 4,
                                  4, 6, 7, 5, 2, 9, 0, 8, 1, 3,
                                  5, 0, 9, 4, 7, 8, 3, 1, 6, 2,
                                  6, 5, 4, 7, 1, 3, 2, 9, 0, 8,
                                  7, 4, 1, 8, 0, 2, 9, 3, 5, 6,
                                  8, 3, 6, 0, 9, 1, 5, 2, 4, 7,
                                  9, 2, 8, 1, 6, 7, 4, 0, 3, 5] };

    let sample              = [0, 8, 5, 9, 7, 3, 4, 2, 1, 6];
    let sample_name         = "0859734216";
    let rows : [uint, ..10] = [0, 1, 2, 3, 5, 6, 9, 8, 4, 7];

    let test = |to: uint| {
        let transversals =
            find_tranversals_extending(&L,
                                       sample.slice_to(to),
                                       rows.slice_to(to).to_owned());
        if transversals.len() < 4 {
            println!("transversals_{:s}{:s}: ({:3u}) {:?}",
                     sample_name.slice_to(to),
                     " ".repeat(sample_name.len() - to),
                     transversals.len(),
                     transversals);
        } else {
            println!("transversals_{:s}{:s}: ({:3u}) ...",
                     sample_name.slice_to(to),
                     " ".repeat(sample_name.len() - to),
                     transversals.len());
        }
    };

    test(9);
    test(6);
    test(3);
    test(2);
    test(1);
}

fn main() {
    main3();
    main4();
    main10();
}
