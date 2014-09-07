extern mod extra;

use std::hashmap::HashSet;
fn perms_vec<A:Clone + IterBytes + Eq>(vec: ~[A]) -> ~[~[A]] {
    println(fmt!("generating perms for %?", vec));
    type Set = HashSet<~[A]>;
    let mut set : Set = HashSet::new();
    set.insert(vec.clone());
    for i in range(0, vec.len()) {
        // working way backwards
        let i = vec.len() - i - 1;
        for j in range(i+1, vec.len()) {
            let mut to_add = ~[];
            println(fmt!("%3u %3u %15u", i, j, set.len()));
            for v0 in set.iter() {
                let mut v = v0.clone();
                v.swap(i, j);
                let present = set.contains(&v);
                if !present { to_add.push(v); }
            };
            for s in to_add.iter() {
                set.insert(s.clone());
            }
        }
    }
    set.iter().transform(|x|x.clone()).to_owned_vec()
}

fn two_n(n: u8) -> ~[u8] {
    let mut accum : ~[u8] = ~[];
    for i in range(0, n) {
        accum.push(i + 1);
        accum.push(i + 1);
    }
    accum
}

#[deriving(Clone, Eq)]
enum Match { Absent, Matches, Mismatch }

fn is_langford_ordered(row: &[u8]) -> bool {
    let len = row.len() as u8;
    for i in range(0, len) {
        let k = row[i];
        let prev_match = (if i < k + 1 || i >= k + len + 1 { Absent }
                          else if row[i - k - 1] == k { Matches }
                          else { Mismatch });
        let post_match = (if k + 1 >= len - i { Absent }
                          else if row[i + k + 1] == k { Matches }
                          else { Mismatch });
        // println(fmt!("i: %? prev:%? post:%?", i, prev_match, post_match));

        if prev_match != Matches && post_match != Matches {
            return false;
        }
    }
    return true;
}

fn find_langfords(k:u8) {
    let n_k = perms_vec(two_n(k));
    println(fmt!("n_%03u.len(): %15u", k as uint, n_k.len()));
    let mut idx = 0;
    let mut i = 0;
    for s in n_k.iter() {
        idx += 1;
        // println(fmt!("%?", s));
        if is_langford_ordered(*s) {
            i += 1;
            println(fmt!("%6u n_%03u[%13u]: %?", i, k as uint, idx, s));
        }
    }
}

fn main() {
    for i in range(0, 10) {
        print(fmt!("%d ", i));
    }
    println(fmt!("permutations(~[1,2,3]): %?", perms_vec(~[1,2,3])));

    let input = ~[2,3,1,2,1,3];
    println(fmt!("is_langford_ordered(%?): %?",
                 input,is_langford_ordered(input)));


    for k in range(3u, 12) {
        let k = k as u8;
        find_langfords(k);
    }
}
