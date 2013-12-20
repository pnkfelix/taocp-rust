trait LatinSquareElem<Latin, Greek> {
    fn latin(&self) -> Latin;
    fn greek(&self) -> Greek;
}

impl<L:Clone, G:Clone> LatinSquareElem<L, G> for (L, G) {
    fn latin(&self) -> L { let &(ref l, _) = self; l.clone() }
    fn greek(&self) -> G { let &(_, ref g) = self; g.clone() }
}

struct Square<Elem> {
    priv contents: ~[Elem] // invariant: length is n^2 for some n.
}



impl<E> Square<E> {
    fn dim(&self) -> uint {
        for i in ::std::iter::range_inclusive(0, (self.contents.len()+1)/2) {
            if i*i == self.contents.len() { return i; }
        }
        fail!("non square.");
    }
    fn new(dim: uint, generate: |uint, uint| -> E) -> Square<E> {
        let mut contents = ~[];
        for i in range(0, dim) {
            for j in range(0, dim) {
                contents.push(generate(i, j));
            }
        }
        Square { contents: contents }
    }
}

impl<E:ToStr> ToStr for Square<E> {
    fn to_str(&self) -> ~str {
        let mut s = ~"";
        let dim = self.dim();
        for j in range(0, dim) {
            for i in range(0, dim) {
                s = s + " " + self.contents[i*dim + j].to_str();
            }
            s = s + "\n";
        }
        return s;
    }
}

struct El<L,G>(L, G);

impl<L:ToStr, G:ToStr> ToStr for El<L,G> {
    fn to_str(&self) -> ~str {
        let &El(ref l, ref g) = self;
        ~"" + l.to_str() + g.to_str()
    }
}

fn main() {
    let suits = ~["♤", "♡", "♢", "♧"];
    let faces = ~["A", "K", "Q", "J"];
    let s = Square::new(4, |i, j| El(suits[i], faces[j]) );
    println!("square: \n{:s}, dim: {}", s.to_str(), s.dim());
}
