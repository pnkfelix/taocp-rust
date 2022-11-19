#[derive(Copy, Clone)]
struct Node(u64);
struct BddArena(Vec<Node>);
#[derive(Copy, Clone)]
struct BddIndex(u32);

#[derive(Copy, Clone)]
pub enum BddPtr {
    Sink(bool),
    Index(u32),
}

const BOT: BddPtr = BddPtr::bot();
const TOP: BddPtr = BddPtr::top();

const INDENT: &'static str = "    ";

impl BddArena {
    fn make(&mut self, n: Node) -> BddPtr {
        let len = self.0.len();
        self.0.push(n);
        BddPtr::from_index(len)
    }

    fn render(&self) -> String {
        let mut accum = String::new();
        accum.push_str("digraph {\n");
        // accum.push_str("    rankdir = \"LR\"\n");
        for (j, n) in self.0.iter().enumerate() {
            let name = format!("node{}", j);
            let var = n.var();
            // let line = format!("{INDENT}{name} [label = \"{var} | <lo> * | <hi> * \" shape=record]\n");
            let line = format!("{INDENT}{name} [label = \"{var}\" shape = circle]\n");
            accum.push_str(&line);
            match n.lo() {
                BddPtr::Index(lo) => {
                    let lo = format!("node{lo}");
                    accum.push_str(&format!("{INDENT}{name}:lo -> {lo} [ style = \"dashed\" ]\n"));
                }
                BddPtr::Sink(val) => {
                    let label = if val { "T" } else { "F" };
                    let lo = format!("node{j}lo_sink");
                    accum.push_str(&format!("{INDENT}{lo} [label={label}, shape=box]"));
                    accum.push_str(&format!("{INDENT}{name}:lo -> {lo} [ style = \"dashed\" ]\n"));
                }
            }
            match n.hi() {
                BddPtr::Index(hi) => {
                    let hi = format!("node{hi}");
                    accum.push_str(&format!("{INDENT}{name}:hi -> {hi}\n"));
                }
                BddPtr::Sink(val) => {
                    let label = if val { "T" } else { "F" };
                    let hi = format!("node{j}hi_sink");
                    accum.push_str(&format!("{INDENT}{hi} [label={label}, shape=box]"));
                    accum.push_str(&format!("{INDENT}{name}:hi -> {hi}\n"));
                }
            }
        }
        accum.push_str("}\n");
        accum
    }
}

// Node layout
//
// |     V     |     LO     |     HI     |
// | <-- 8 --> | <-- 28 --> | <-- 28 --> |

const VAR_WIDTH: u32 = 8;
const VAR_LIMIT: u32 = 1 << VAR_WIDTH;
const VAR_MASK: u32 = (1 << VAR_WIDTH) - 1;
const PTR_WIDTH: u32 = 28;
// Note that bound implied by PTR_LIMIT *includes* TOP_PTR and BOT_PTR; the
// vec-index limit is IDX_LIMIT
const PTR_LIMIT: u32 = 1 << PTR_WIDTH;
const IDX_LIMIT: u32 = BOT.as_u32();
const PTR_MASK: u32 = (1 << PTR_WIDTH) - 1;
const TOP_PTR: u32 = PTR_MASK & u32::MAX;
const BOT_PTR: u32 = PTR_MASK & (u32::MAX - 1);

impl BddPtr {
    const fn top() -> Self { Self::Sink(true) }
    const fn bot() -> Self { Self::Sink(false) }
    fn from_index(idx: usize) -> Self {
        assert!(idx < (IDX_LIMIT as usize));
        Self::Index(idx as u32)
    }

    const fn as_u32(&self) -> u32 {
        match *self {
            BddPtr::Sink(true) => TOP_PTR,
            BddPtr::Sink(false) => BOT_PTR,
            BddPtr::Index(idx) => idx,
        }
    }

    fn from_u32(code: u32) -> Self {
        if code == TOP_PTR {
            Self::top()
        } else if code == BOT_PTR {
            Self::bot()
        } else {
            Self::from_index(code as usize)
        }
    }
}


impl Node {
    pub fn var(&self) -> u32 { (self.0 >> (PTR_WIDTH * 2)) as u32 }
    pub fn lo(&self) -> BddPtr { BddPtr::from_u32((self.0 >> PTR_WIDTH) as u32 & PTR_MASK) }
    pub fn hi(&self) -> BddPtr { BddPtr::from_u32(self.0 as u32 & PTR_MASK) }

    pub fn new(var: u32, lo: BddPtr, hi: BddPtr) -> Result<Self, &'static str> {
        if var >= VAR_LIMIT { return Err("var too large"); }

        let var_shifted = (var as u64) << (PTR_WIDTH * 2);
        let lo_shifted  = (lo.as_u32() as u64) << PTR_WIDTH;
        Ok(Node(var_shifted | lo_shifted | hi.as_u32() as u64))
    }
}

fn main() -> Result<(), &'static str> {
    let mut arena = BddArena(Vec::new());
    let n0 = arena.make(Node::new(3, BOT, TOP)?);
    let n1 = arena.make(Node::new(2, BOT, n0)?);
    let n2 = arena.make(Node::new(2, n0, TOP)?);
    let n3 = arena.make(Node::new(1, n1, n2)?);

    let mut arena = BddArena(Vec::new());
    let n0 = arena.make(Node::new(4, TOP, BOT)?);
    let n1 = arena.make(Node::new(4, BOT, TOP)?);
    let n2 = arena.make(Node::new(3, TOP, BOT)?);
    let n3 = arena.make(Node::new(3, n0, n1)?);
    let n4 = arena.make(Node::new(2, n2, n3)?);
    let n5 = arena.make(Node::new(2, BOT, TOP)?);
    let n6 = arena.make(Node::new(1, n4, n5)?);

    let mut arena = BddArena(Vec::new());
    let n0 = arena.make(Node::new(6, TOP, BOT)?);
    let n1 = arena.make(Node::new(5, TOP, n0)?);
    let n2 = arena.make(Node::new(5, TOP, BOT)?);
    let n3 = arena.make(Node::new(5, n0, BOT)?);
    let n4 = arena.make(Node::new(4, n1, n2)?);
    let n5 = arena.make(Node::new(4, n1, BOT)?);
    let n6 = arena.make(Node::new(4, n0, n3)?);
    let n7 = arena.make(Node::new(4, n0, BOT)?);
    let n8 = arena.make(Node::new(3, n4, n5)?);
    let n9 = arena.make(Node::new(3, n4, BOT)?);
    let n10 = arena.make(Node::new(3, n6, n7)?);
    let n11 = arena.make(Node::new(2, n8, n9)?);
    let n12 = arena.make(Node::new(2, n10, BOT)?);
    let n13 = arena.make(Node::new(1, n11, n12)?);


    
    let rendered = arena.render();
    layout::core::utils::save_to_file("/tmp/out.dot", &rendered);
    let mut parser = layout::gv::DotParser::new(&rendered);
    let g = parser.process().map_err(|_e| { dbg!(_e); "parse error" })?;
    let mut gb = layout::gv::GraphBuilder::new();
    gb.visit_graph(&g);
    let mut vg = gb.get();
    let mut svg = layout::backends::svg::SVGWriter::new();
    vg.do_it(false, false, false, &mut svg);
    layout::core::utils::save_to_file("/tmp/out.svg", &svg.finalize());
    Ok(())
}
