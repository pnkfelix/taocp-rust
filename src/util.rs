use extra::sort;

fn powervec<A:Clone>(vec: ~[A]) -> ~[~[A]] {
    let mut accum : ~[~[A]] = ~[~[]];
    foreach i in range(0, vec.len()) {
        let x = &vec[i];
        let v = {
            let mut i = do accum.iter().transform |v| {
                let mut v = v.clone();
                v.push(x.clone());
                v
            };
            i.to_owned_vec()
        };
        accum.push_all(v);
    }
    sort::merge_sort(accum, |v1, v2| { v1.len() <= v2.len() })
}
