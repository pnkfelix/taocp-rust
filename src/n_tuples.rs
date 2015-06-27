pub struct MixedRadixLexIterator<N> { m: ~[N], a: ~[N] }

fn mixed_radix_lexicographic<N:Num>(m: ~[N]) -> MixedRadixLexIterator<N> {
    let mut a = ~[Zero::zero()];
    for m_i in m.iter() { a.push(*m_i) }
    MixedRadixLexIterator{ m: ~[(One::one() + One::one())].append(m), a: a }
}

impl<N:Num> Iterator<~[N]> for MixedRadixLexIterator<N> {
    fn next(&mut self) -> Option<N> {

    }
}

fn main() {
    println("Hello World");
}
