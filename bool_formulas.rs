mod horn_clauses {
    use std::hashmap;
    use std::util;

    struct prop_id { idx: uint }
    struct clause_id { idx: uint }

    // represents a clause c of the form: u_1 /\ ... /\ u_k => v,
    // where k >= 0
    struct clause {
        conclusion: prop_id,
        count: uint // number of hypotheses of c not yet asserted.
    }

    struct prop {
        truth: bool, // true if p known to be true; otherwise 
        last: Option<~hypothesis>
    }

    struct hypothesis {
        clause: clause_id, // clause for which this appears on this left
        prev: Option<~hypothesis> // previous hypothesis containing propostion of this.
    }

    fn clause_to_str<V:ToStr>(assumes: &[V], result: &V) -> ~str {
        let lhs = assumes.map(|x|x.to_str()).connect(" ∧ ");
        lhs.append(" ⇒ ").append(result.to_str())
    }

    pub fn core_computation<V:Eq+Hash+IterBytes+Clone+ToStr>(propvars: ~[V],
                                                             input_clauses: ~[(~[V], V)])
        -> (~[prop], ~[clause]) {
        type PI = hashmap::HashMap<V, prop_id>;
        type CI = hashmap::HashMap<(~[V], V), clause_id>;
        fn new_map<K:Eq+IterBytes, V>() -> hashmap::HashMap<K, V> { hashmap::HashMap::new() }
        for c in input_clauses.iter() {
            let &(ref assumes, ref result) = c;
            println(fmt!("%s", clause_to_str(*assumes, result)));
        }
        let mut s = ~[];

        // C1. [Initialize.] Set LAST(p) <- Λ and TRUTH(p) <- 0 for each
        //     proposition p.  Also set s <- 0, so that the stack is empty.
        //     Then for each clause c, having the form above,
        //     set CONCLUSION(c) <- v and COUNT(c) <- k.
        //     If k = 0 and TRUTH(v) = 0, set TRUTH(v) <- 1, S_s <- v,
        //     and s <- s + 1.  Otherwise, for 1 <= j <= k, create a
        //     hypothesis record h and set CLAUSE(h) <- c,
        //     PREV(h) <- LAST(u_j), LAST(u_j) <- h.

        let mut props = ~[];
        let mut pmap : PI = new_map();
        {
            for v in propvars.iter() {
                let p = prop { truth: false, last: None };
                let pi = prop_id{ idx: props.len() };
                props.push(p);
                pmap.insert(v.clone(), pi);
            }
        };


        let mut clauses = ~[];
        let mut cmap = new_map();
        {
            for i in input_clauses.iter() {
                let (ref u_k, ref v) = *i;
                let v = pmap.find(v).unwrap();
                let k = u_k.len();
                let c = clause { conclusion: *v, count: k };
                let ci = clause_id{ idx: clauses.len() };
                clauses.push(c);
                cmap.insert(i.clone(), ci);
                if k == 0 && !props[v.idx].truth {
                    props[v.idx].truth = true;
                    s.push(*v);
                } else {
                    for u_j in u_k.iter() {
                        let pi = pmap.get(u_j);
                        let mut l = None;
                        util::swap(&mut props[pi.idx].last, &mut l);
                        let h = ~hypothesis { clause: ci, prev: l };
                        props[pi.idx].last = Some(h);
                    }
                }
            }
        };

        // C2. [Prepare to assert p.] Terminate the algorithm if s = 0; the
        //     desired core now consists of all propositions whose TRUTH has
        //     been set to 1.  Otherwise set s <- s - 1, p <- S_s,
        //     and h <- LAST(p).
        if (s.len() == 0) {
            return (props, clauses);
        }
        let mut p = s.pop();
        let mut h = None;
        util::swap(&mut props[p.idx].last, &mut h);

        loop {
            // C3. [Done with hypotheses?] If h = Λ, return to C2.
            let hi = match h {
                None => {
                    if (s.len() == 0) {
                        return (props, clauses);
                    }
                    p = s.pop();
                    util::swap(&mut props[p.idx].last, &mut h);
                    loop;
                }
                Some(hi) => hi
            };

            // C4. [Validate h.] Set c <- CLAUSE(h) and COUNT(c) <- COUNT(c) - 1.
            //     If the new value of COUNT(c) is still nonzero, go to step C6.

            let ci = hi.clause;
            clauses[ci.idx].count -= 1;

            if clauses[ci.idx].count == 0 {
                // C5. [Deduce CONCLUSION(c).] Set p <- CONCLUSION(c).
                //     If TRUTH(p) = 0, set TRUTH(p) <- 1, S_s <- p, s <- s + 1.
                let pi = clauses[ci.idx].conclusion;
                if !props[pi.idx].truth {
                    props[pi.idx].truth = true;
                    s.push(pi);
                }
            }

            // C6. [Loop on h.] Set h <- PREV(h) and return to C3.
            h = hi.prev;
        }
    }
}

fn main() {
    let ret = horn_clauses::core_computation(~["a", "b", "c", "d"],
                                             ~[(~[], "a"),
                                               (~["d"], "b"),
                                               (~["a", "b"], "c"),
                                               (~["a"], "b")]);
    println(fmt!("Hello World: %?", ret));
}
