#[ link(name = "sets", vers = "0.3") ];

trait AbstractSet<Elem> {
    fn has(&self, e:&Elem) -> bool;
}

trait Set<Elem> : AbstractSet<Elem> {
    fn each_elem(&self, blk : &fn(v: &Elem) -> bool);
}

trait SubsetSpecialized<Elem> : AbstractSet<Elem> {
    fn subset(&self, s:&Self) -> bool;
}

fn print<E:ToStr, S:Set<E>>(s:&S) -> ~str {
    let mut ret = ~"{";
    let mut printed = false;
    for s.each_elem |v| {
        if printed { ret += "," }
        ret += v.to_str();
        printed = true;
    }
    ret += "}";
    return ret;
}

struct Singleton<X> { x : X }
fn Singleton<X>(x:X) -> Singleton<X> { Singleton{ x: x } }

struct TwoSet<X> { x: X, y: X }
fn TwoSet<X>(x:X, y:X) -> TwoSet<X> { TwoSet{ x: x, y: y } }

struct NatCount { limit : uint }
fn NatCount(limit:uint) -> NatCount { NatCount{ limit: limit } }

struct NatRange { start : uint, limit : uint }
fn NatRange(a:uint, b:uint) -> NatRange { NatRange{ start:a, limit:b } }

struct BitsUint { bits : uint }
fn BitsUint(x:uint) -> BitsUint { BitsUint{ bits: x } }

impl<X:Eq> AbstractSet<X> for Singleton<X> {
    fn has(&self, e:&X) -> bool { *e == self.x }
}

impl<X:Eq> SubsetSpecialized<X> for Singleton<X> {
    fn subset(&self, s:&Singleton<X>) -> bool {
        self.x == s.x
    }
}

impl<X:Eq> Set<X> for Singleton<X> {
    fn each_elem(&self, blk : &fn(v: &X) -> bool) {
        blk(&self.x);
    }
}

impl<X:Eq> AbstractSet<X> for TwoSet<X> {
    fn has(&self, e:&X) -> bool {
        *e == self.x || *e == self.y
    }
}

impl AbstractSet<uint> for NatCount {
    fn has(&self, e:&uint) -> bool {
        *e < self.limit
    }
}

impl SubsetSpecialized<uint> for NatCount {
    fn subset(&self, e:&NatCount) -> bool {
        self.limit <= e.limit
    }
}

impl AbstractSet<uint> for BitsUint {
    fn has(&self, e:&uint) -> bool {
        ((1 << *e) & self.bits) != 0
    }
}

impl SubsetSpecialized<uint> for BitsUint {
    fn subset(&self, s:&BitsUint) -> bool {
        (self.bits & s.bits) == self.bits
    }
}

impl Set<uint> for BitsUint {
    fn each_elem(&self, blk : &fn(v: &uint) -> bool) {
        for uint::range(0, sys::nonzero_size_of::<uint>())
            |i| {
            if self.has(&i) && !blk(&i) { break; }
        }
    }
}

/*
fn vec_subset<X:Eq>(x: &[X], y: &[X]) -> bool {
    do x.all |e| { do y.any |f| { e == f } }
}

impl<'self, X:Eq> AbstractSet<X> for &'self [X] {
    fn has(&self, e:&X) -> bool {
        do self.any |x| { e == x }
    }
}

impl<'self, X:Eq> SubsetSpecialized<X> for &'self [X] {
    fn subset(&self, s:& &[X]) -> bool {
        do self.all |x| { s.has(x) }
        // vec_subset(*self, *s)
    }
}
*/

pub fn main() {
    let _x001 = BitsUint { bits: 1u };
    let _x011 = BitsUint { bits: 3u };
    let _x101 = BitsUint { bits: 5u };
    let _x111 = BitsUint { bits: 7u };

    io::println(fmt!("%s <: %s %?", print(&_x101), print(&_x111), _x101.subset(&_x111)));
    io::println(fmt!("%s <: %s %?", print(&_x101), print(&_x001), _x101.subset(&_x001)));
}
