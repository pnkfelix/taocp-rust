extern mod sets (vers="0.3");
use sets::*;

trait Edge<V, Vset:Set<V> > {
    fn neighbors(&self) -> Vset;
}

trait Graph<V, EdgeVset:Set<V>, E:Edge<V, EdgeVset>,
            Eset:Set<E> + SubsetSpecialized<E>,
            GraphVset:Set<V> + SubsetSpecialized<V>
           > {
  fn vertices<'s>(&'s self) -> &'s GraphVset;
  fn edges<'s>(&'s self) -> &'s Eset;
}

struct AdjacencyMatrix {
    vertex_count : uint,
    arcs : ~[uint]
}

fn is_subgraph<V,
               EdgeVset:Set<V>, E:Edge<V, EdgeVset>,
               Eset:Set<E> + SubsetSpecialized<E>,
               GraphVset:Set<V> + SubsetSpecialized<V>,
               G:Graph<V, EdgeVset, E, Eset, GraphVset>
              >(g1: &G, g2: &G) -> bool
{
    let v1 : &GraphVset = g1.vertices();
    let v2 : &GraphVset = g2.vertices();
    return v1.subset(v2) && g1.edges().subset(g2.edges());
}

fn main() {
    sets::main();
    io::print("hello world");
}
