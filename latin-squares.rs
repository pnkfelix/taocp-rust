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


impl<L:Eq, G:Eq, E:LatinSquareElem<L, G>> LGSquare<E> {
    fn is_latin(&self) -> bool {
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

impl<E:ToStr> ToStr for LGSquare<E> {
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

#[deriving(Clone)]
struct El<L,G>(L, G);

impl<L:ToStr, G:ToStr> ToStr for El<L,G> {
    fn to_str(&self) -> ~str {
        let &El(ref l, ref g) = self;
        ~"" + l.to_str() + g.to_str()
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
    let suits = ~["♤", "♡", "♢", "♧"];
    let faces = ~["A", "K", "Q", "J"];
    let mut s = LGSquare::new(4, suits.clone(), faces.clone(), |i, j| El(suits[i], faces[j]) );

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
}

fn main() {
    main3();
    main4();
}
