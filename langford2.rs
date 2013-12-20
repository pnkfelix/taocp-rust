extern mod std;

// Langford Pairs:
//
// Arrange the 2n objects {1, 1, 2, 2, ..., n, n} such that exactly k
// numbers occur between the two appearances of each digit k.

// E.g. for n=3: 231213

fn is_langford(seq: &[uint]) -> bool {
    fn both_exist(i:int, j:int, seq:&[uint]) -> bool {
        0 <= i && i < seq.len() as int && 0 <= j && j < seq.len() as int
    }
    fn matches(i:int, j:int, seq:&[uint]) -> bool {
        seq[i] == seq[j]
    }
    for int::range(0, seq.len() as int) |i| {
        let v = seq[i] as int;
        let pre = both_exist(i-v-1, i, seq) && matches(i-v-1, i, seq);
        let post = both_exist(i, i+v+1, seq) && matches(i, i+v+1, seq);
        if !pre && !post { return false }
    }
    return true;
}

fn seq_2n(n:uint) -> ~[uint] {
    let mut i = 1;
    let mut s = ~[];
    while i <= n {
        s = s + [i,i];
        i = i + 1;
    }
    s
}

// TAOCP 7.2.1.2
fn gen_perms<X:Ord>(seq:~[X], visit:&fn(seq:&[X])) {
    let mut a = seq;
    // assumes seq is initially ordered.
    loop {
        visit(a);
        let n = a.len();
        let mut j = n - 1;
        while a[j-1] >= a[j] {
            j = j - 1;
            if j == 0 { return; }
        }
        let mut l = n;
        while a[j-1] >= a[l-1] {
            l = l - 1;
        }
        a[j-1] <-> a[l-1];
        let mut k = j + 1;
        l = n;
        while k < l {
            a[k-1] <-> a[l-1];
            k = k + 1;
            l = l - 1;
        }
    }
}

fn check_langford(seq:~[uint]) {
    io::println(fmt!("Hello world %?: %?", seq, is_langford(seq)));
}

// The generate-and-test algorithm is simple + stupid + slow.  (Very slow.)
//   n: 1 count: 0 time: 0.00000787
//   n: 2 count: 0 time: 0.00000689
//   n: 3 count: 2 time: 0.00010408
//   n: 4 count: 2 time: 0.00074278
//   n: 5 count: 0 time: 0.03001265
//   n: 6 count: 0 time: 1.89905339
//   n: 7 count: 52 time: 172.0576888
//   n: 8 count: 300 time: 20521.34417215
// I.e., ~2 seconds for n=6, ~3 minutes for n=7, and >5 hours for n=8.
//
// It might be interesting to explore some alternative strategies.

fn main() {
    // use std::time;
    // io::println("time: %?", time::get_time());
    check_langford(~[1u, 2u, 3u]);
    check_langford(~[2u, 3u, 1u, 2u, 1u, 3u]);
    check_langford(~[1u, 3u, 1u, 2u, 1u, 3u]);
    for uint::range(1, 11) |n| {
        // use std;
        let start = std::time::precise_time_ns();
        let mut printed = 0;
        let mut counted = 0;
        // Generate-and-test to identify all Langford pairings of length 2n.
        // (Generate-and-test is a *slow* way to do this, since the set
        //  of permutations is quite large.)
        do gen_perms(seq_2n(n)) |v| {
            if is_langford(v) {
                counted += 1;
                if printed < 10 {
                    io::println(fmt!("      v: %?", v));
                    printed += 1;
                } else if printed == 10 {
                    io::println(fmt!("      ..."));
                    printed += 1;
                }
            }
        }
        let finis = std::time::precise_time_ns();
        io::println(fmt!("n: %? count: %? time: %?", n, counted, (finis - start) as f64/(1_000_000_000 as f64)));
    }
    io::println(fmt!("Hello world"));
}
