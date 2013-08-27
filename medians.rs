fn median_labels(g: MedianGraph, a: SourceVertex) {

    // H1. [Initialize.] Preprocesss `g` by visiting ll vertices in
    // order of their distance from `a`.  For each edge `u` -- `v`, we
    // say that `u` is an _early neighbor_ of `b` if `a` is closer to
    // `u` than to `v`, otherwise `u` is a _late neighbor_; in other
    // words, the early neighbors of `v` will already have been
    // visited when `v` is nencountered, but the late neighbors will
    // still be awaiting their turn.  Rearrange all adjacency lists so
    // that early neighbors are listed first.  Place each edge
    // initially in its own equivalence class; a "union-find
    // algorithm" like Algorithm 2.3.3E will be used to merge classes
    // when the algorithm learns that they're equivalent.

    // H2. [Call the subroutine.] Set j := 0 and invoke Subroutine I
    // with parameter a.  (Subroutine I appears below.  The global
    // variable j will be used to crate a master list of edges r_j --
    // s_j for 1 <= j < n, where n is the total number of vertices;
    // there will be one entry with s_j = v for each vertex v != a.)

    // H3. [Assign the labels.] Number the equivalence classes from 1
    // to t.  Then set l(a) to the t-bit string 0 ... 0.  For j = 1,
    // 2, ..., n-1 (in this order), set l(s_j) to l(r_j) with bit k
    // changed from 0 to 1, where k is the equivalence class of edge
    // r_j -- s_j.

    // Subroutine I (Process descendants of r).  This recursive
    // subroutine, with parameter r and global variable j, does the
    // main work of Algorithm H on the graph pf all vertices currently
    // reachable from vertex r.  In the course of processing, all such
    // vertices will be recorded on the master list, except r itself,
    // and all edges between them will be removed from the current
    // graph.  Each vertex has four fields called its LINK, MARK, RANK
    // and MATE, initially null.

    // I1. [Loop over s.] Choose a vertex s with r -- s.  If there is
    // no such vertex, return from the subroutine.

    // I2. [Record the edge.] Set j := j + 1, r_j := r, and s_j := s.

    // I3. [Begin breadth-first search.] (Now we want to find and
    // delete all edges of the current graph that are equvialent to r
    // -- s.)  Set MARK9s) := s, RNAK(s) := 1, LINK(s) := null,
    // and v := q := s.

    // I4. [Find the mate of v.] Find the early neighbor u of v for
    // which MARK(u) != s.  (There will be exactly one such vertex u.
    // Recall that early neighbors have been placed first, in step
    // H1.)  Set MATE(v) := u.

    // I5. [Delete u -- v.] Make the edges u -- v and r -- s
    // equivalent by merging their equivalence classes.  Remove u and
    // v from each other's adjacency lists.

    // I6. [Classify the neighbors of v.]  For each early neighbor u
    // of v, do step I7; for each late neighbor u of v, do step I8.
    // Then go to step I9.

    // I7. [Note a possible equivalence.] If MARK(u) == s and RANK(u)
    // == 1, make the edge u -- v equivalent to the edge MATE(u) --
    // MATE(v).  Return to I6.

    // I8. [Rank u.] If MARK(u) == s and RANK(u) == 1, return to I6.
    // Otherwise set MARK(u) := s and RANK(u) := 2.  Set w to the
    // first neight of u (it will be early).  If w == v, reset w to
    // u's second early neighbor; but return to I6 if u has only one
    // early neighbor.  If MARK(w) != s or RANK(w) != 2, set RANK(u)
    // := 1, LINK(u) := null, LINK(q) := u, and q := u.  Return to I6.

    // i9. [Continue breadth-first search.] Set v := LINK(v).  Return
    // to I4 if v != null.

    // I10. [Process subgraph s.] Call subroutine I recursively with
    // parameter s.  Then return to I1.

}
