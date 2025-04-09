use std::collections::{HashMap};
use std::collections::hash_map::{Entry};
use std::fmt::Write;

mod pretty {
    #[derive(Copy, Clone, Debug)]
    pub struct Spacing {
        indent: u32,
        delta_indent: u32,
        whitespace_significant: bool,
    }

    impl std::fmt::Display for Spacing {
        fn fmt(&self, w: &mut std::fmt::Formatter) -> std::fmt::Result {
            for _ in 0..(self.indent) {
                write!(w, " ")?;
            }
            Ok(())
        }
    }

    impl Spacing {
        pub(crate) fn initial_indent(indent: u32) -> Self {
            Spacing {
                indent,
                delta_indent: 4,
                whitespace_significant: false,
            }
        }

        pub(crate) fn is_whitespace_significant(&self) -> bool { self.whitespace_significant }
        pub(crate) fn significant_whitespace(&self) -> Self { Spacing { whitespace_significant: true, ..*self } }
        pub(crate) fn insignificant_whitespace(&self) -> Self { Spacing { whitespace_significant: false, ..*self } }
                                                                
        pub(crate) fn zero() -> Self { Spacing::initial_indent(0) }
        pub(crate) fn four() -> Self { Spacing::initial_indent(4) }
        pub(crate) fn reindent(&self) -> Self { Spacing { indent: self.indent + self.delta_indent, ..*self } }
        pub(crate) fn dedent(&self) -> Self { Spacing { indent: self.indent - self.delta_indent, ..*self } }
        pub(crate) fn extend(&self, x: u32) -> Self { Spacing { indent: self.indent + x, ..*self } }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct BinTree<L> {
    label: L,
    sibling: Option<Box<Self>>,
    first_child: Option<Box<Self>>,
}

impl<L> BinTree<L> {
    fn leaf(l: L) -> Self {
        BinTree::new(l, None, None)
    }
    fn only_child(l: L, first_child: Self) -> Self {
        BinTree::new(l, None, Some(first_child))
    }
    fn childless(l: L, sibling: Self) -> Self {
        BinTree::new(l, Some(sibling), None)
    }
    fn full(l: L, sibling: Self, first_child: Self) -> Self {
        BinTree::new(l, Some(sibling), Some(first_child))
    }
    fn new(label: L, sibling: Option<Self>, first_child: Option<Self>) -> Self {
        BinTree {
            label,
            sibling: sibling.map(Box::new),
            first_child: first_child.map(Box::new),
        }
    }
}

use pretty::Spacing;

#[cfg(feature = "s_exp")]
impl<L: std::fmt::Display> BinTree<L> {

    /// Renders the binary tree as an Lisp-style S-exp. If its a leaf, then we
    /// just print the label as if it were an atom. If its a non-leaf, then we
    /// print open-parenthesis, the label for this node, then the sibling chain
    /// immediately to the right of the label, the first child tree starting
    /// some number of lines below the label, and then the close parenthesis.
    pub fn render_binary_sexp(&self) -> String {
        self.render_binary_sexp_with(Spacing::zero())
    }

    // Helper function for render_binary_sexp. `indent_after_newline` matches
    // how many space characters would be necessary to indent a newline so
    // that it matches where this node is being printed at.
    fn render_binary_sexp_with(&self, indent_after_newline: Spacing) -> String {
        let extend = |extra| indent_after_newline.extend(extra);
        let render_opt = |opt_tree: &Option<Box<Self>>, extend_indent| {
            opt_tree.as_ref().map(|x|x.render_binary_sexp_with(extend(extend_indent))).unwrap_or("_".to_string())
        };
        match self {
            BinTree { label, sibling: None, first_child: None } => format!("{}", label),
            BinTree { label, sibling, first_child } => {
                let sib_rendered = render_opt(sibling, 3);
                let child_rendered = render_opt(first_child, 1);
                format!("({L} {S}\n\
                      {I} {C})",
                        L = label,
                        S = sib_rendered,
                        I = indent_after_newline,
                        C = child_rendered)
            }
        }
    }
}

impl<L: std::fmt::Display> BinTree<L> {
    pub fn render_ascii_art(&self) -> String {
        self.render_ascii_art_with(String::new())
    }

    fn render_ascii_art_with(&self, prefix: String) -> String {
        let mut buf = String::new();
        self.write_ascii_art_with(&mut buf, prefix).unwrap();
        buf
    }
    
    fn write_ascii_art_with(&self, buf: &mut dyn std::fmt::Write, prefix: String) -> std::fmt::Result {
        write!(buf, "{}", self.label)?;
        if let Some(sibling) = &self.sibling {
            let sibling_tree_prefix = format!("{prefix}{}   ", if self.first_child.is_some() { "|" } else { " " });
            write!(buf, " - ")?;
            sibling.write_ascii_art_with(buf, sibling_tree_prefix)?;
        }
        if let Some(first_child) = &self.first_child {
            write!(buf, "\n")?;          // end the line for the current (or sibling) tree
            write!(buf, "{prefix}|\n")?; // emit the vertical connector to the child
            write!(buf, "{prefix}")?;    // emit the prefix leading up to the child
            first_child.write_ascii_art_with(buf, prefix)? // write the child
        }
        Ok(())
    }
}

#[derive(Copy, Clone)]
struct Id(usize);

struct Indexer {
    next_id: usize,
    addr_to_index: HashMap<usize, Id>,
}

impl Indexer {
    fn new() -> Self { Indexer { next_id: 1, addr_to_index: HashMap::new() } }
    fn make_id(&mut self, x: &impl Sized) -> Id {
        let addr = x as *const _ as usize;
        match self.addr_to_index.entry(addr) {
            Entry::Occupied(occ) => *occ.get(),
            Entry::Vacant(vac) => {
                let id = Id(self.next_id);
                vac.insert(id);
                self.next_id += 1;
                id
            }
        }
    }
}

impl std::fmt::Display for Id {
    fn fmt(&self, w: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(w, "id{}", self.0)
    }
}

impl<L: std::fmt::Display> BinTree<L> {
    pub fn render_graphviz(&self) -> String {
        let mut buf = String::new();
        let mut indexer = Indexer::new();
        let indent = Spacing::four();
        write!(&mut buf, "digraph G {{\n").unwrap();
        self.write_graphviz(&mut buf, indent, &mut indexer).unwrap();
        write!(&mut buf, "}}").unwrap();
        buf
    }
    pub fn render_graphviz_contents(&self) -> String {
        let mut buf = String::new();
        let mut indexer = Indexer::new();
        self.write_graphviz(&mut buf, Spacing::four(), &mut indexer).unwrap();
        buf
    }
    pub fn write_graphviz(&self, buf: &mut dyn std::fmt::Write, indent: Spacing, idx: &mut Indexer) -> Result<Id, std::fmt::Error> {
        let id = idx.make_id(self);
        write!(buf, "{indent}{id} [label = \"{}\"]\n", self.label)?;
        if let Some(sibling) = &self.sibling {
            let sib_id = sibling.write_graphviz(buf, indent, idx)?;
            write!(buf, "{indent}{id} -> {sib_id} [label=\"sibling\"]\n")?;
        }
        if let Some(child) = &self.first_child {
            let child_id = child.write_graphviz(buf, indent, idx)?;
            write!(buf, "{indent}{id} -> {child_id} [label=\"child\"]\n")?;
        }
        Ok(id)
    }
}

impl<L: std::fmt::Display> BinTree<L> {
    pub fn render_mermaid(&self) -> String {
        let mut buf = String::new();
        let mut indexer = Indexer::new();
        let indent = Spacing::four();
        write!(&mut buf, "flowchart G \n").unwrap();
        self.write_mermaid(&mut buf, indent, &mut indexer).unwrap();
        write!(&mut buf, "").unwrap();
        buf
    }
    pub fn write_mermaid(&self, buf: &mut dyn std::fmt::Write, indent: Spacing, idx: &mut Indexer) -> Result<Id, std::fmt::Error> {
        let id = idx.make_id(self);
        write!(buf, "{indent}{id}[\"{}\"]\n", self.label)?;
        if let Some(sibling) = &self.sibling {
            let sib_id = sibling.write_mermaid(buf, indent, idx)?;
            write!(buf, "{indent}{id} -- sibling --> {sib_id} \n")?;
        }
        if let Some(child) = &self.first_child {
            let child_id = child.write_mermaid(buf, indent, idx)?;
            write!(buf, "{indent}{id} -- child --> {child_id} \n")?;
        }
        Ok(id)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Xy { x: u32, y: u32 }
impl Xy {
    fn new(x: u32, y: u32) -> Xy { Xy { x, y } }
    fn add_x(&self, x: u32) -> Xy { Xy { x: self.x + x, ..*self } }
    fn add_y(&self, y: u32) -> Xy { Xy { y: self.y + y, ..*self } }
    fn add_xy(&self, xy: Xy) -> Xy { Xy { x: self.x + xy.x, y: self.y + xy.y } }
    fn with_x(&self, x: u32) -> Xy { Xy { x, ..*self } }
    fn with_y(&self, y: u32) -> Xy { Xy { y, ..*self } }
    fn max(&self, xy: Xy) -> Xy {
        use std::cmp;
        Xy { x: cmp::max(self.x, xy.x),
             y: cmp::max(self.y, xy.y), }
    }
}

trait LayoutArea {
    fn height(&self) -> u32 { 1 }
    fn width(&self) -> u32 { 1 }
}

impl LayoutArea for char { }
impl LayoutArea for str {
    fn height(&self) -> u32 { 1 + u32::try_from(bytecount::count(self.as_bytes(), b'\n')).unwrap() }
    fn width(&self) -> u32 { self.len() as u32 }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct LayoutAnswer<L> {
    tree: BinTree<(L, Xy)>,
    bounding_box: Xy,
}

impl<L: Clone + LayoutArea> BinTree<L> {
    pub fn layout_naively(&self) -> LayoutAnswer<L> {
        self.layout(Xy::new(0, 0), Xy::new(1, 1))
    }
    pub fn layout(&self, start: Xy, space: Xy) -> LayoutAnswer<L> {
        let (sibling, bb1) =
            if let Some(sibling) = &self.sibling {
                let LayoutAnswer { tree, bounding_box: bb1 } = sibling.layout(start.add_x(space.x), space);
                (Some(Box::new(tree)), bb1)
        
            } else {
                (None, start)
            };
        let (first_child, bb2) =
            if let Some(child) = &self.first_child {
                let LayoutAnswer { tree, bounding_box: bb2 } = child.layout(start.with_y(bb1.y + space.y), space);
                (Some(Box::new(tree)), bb2)
            } else {
                (None, start)
            };
        LayoutAnswer {
            tree: BinTree {
                label: (self.label.clone(), start),
                sibling,
                first_child,
            },
            bounding_box: bb1.max(bb2),
        }
    }
}

pub mod attrs { 
    #[derive(Clone, PartialEq, Eq, Debug)]
    pub struct Attrs(pub Vec<(String, String)>);
    
    pub fn svg_attrs(width: &str, height: &str, #[allow(non_snake_case)] viewBox: &str, style: &str) -> Attrs {
        Attrs(vec![
            ("xmlns".into(), "http://www.w3.org/2000/svg".into()),
            ("width".into(), width. into()),
            ("height".into(), height.into()),
            ("viewBox".into(), viewBox.into()),
            ("style".into(), style.into()),
        ])
    }
}

pub mod css {
    use super::attrs::Attrs;
    use super::pretty::Spacing;
    #[derive(Clone, PartialEq, Eq, Debug)]
    pub struct Class(pub String);
    #[derive(Clone, PartialEq, Eq, Debug)]
    pub struct Rule {
        // At some point, I might want/need to generalize this to other selectors.
        class: Class,
        pub attrs: Attrs,
    }
    impl Rule {
        pub fn class(name: &str, attrs: Attrs) -> Self {
            Rule { class: Class(name.into()), attrs }
        }
    }
    impl Rule {
        pub fn write_xml(&self, buf: &mut dyn std::fmt::Write, _indent: Spacing) -> Result<(), std::fmt::Error> {
            write!(buf, " .{} {} ", self.class.0, "{")?;
            for (k, v) in &self.attrs.0 {
                write!(buf, " {}: {};", k, v)?;
            }
            write!(buf, " {}", "} ")?;
            Ok(())
        }
    }
}

pub mod xml {
    use super::attrs::Attrs;
    use super::css;
    #[derive(Clone, PartialEq, Eq, Debug)]
    pub struct Tag(pub String);
    impl Tag {
        pub fn svg() -> Tag { Tag("svg".into()) }
        pub(crate) fn body_has_significant_whitespace(&self) -> bool {
            self.0 == "text"
        }
    }
    #[derive(Clone, PartialEq, Eq, Debug)]
    pub struct Xexpr {
        pub tag: Tag,
        pub attrs: Attrs,
        pub body: Body,
    }
    impl Xexpr {
        pub fn style(items: Vec<css::Rule>) -> Xexpr {
            Xexpr {
                tag: Tag("style".into()),
                attrs: Attrs(vec![]),
                body: Body::Css(items),
            }
        }
        pub fn path(d: &str, class: css::Class) -> Xexpr {
            Xexpr {
                tag: Tag("path".into()),
                attrs: Attrs(vec![("d".into(), d.into()), ("class".into(), class.0.into())]),
                body: Body::EMPTY,
            }
        }
        pub fn g(transform: Option<&str>, elems: Vec<Xexpr>) -> Xexpr {
            Xexpr {
                tag: Tag("g".into()),
                attrs: Attrs(if let Some(transform) = transform {
                    vec![("transform".into(), transform.into())]
                } else {
                    vec![]
                }),
                body: Body::X(elems),
            }
        }
        pub fn circle(r: &str, fill: &str, class: css::Class) -> Xexpr {
            Xexpr {
                tag: Tag("circle".into()),
                attrs: Attrs(vec![("r".into(), r.into()), ("fill".into(), fill.into()), ("class".into(), class.0.into())]),
                body: Body::empty(),
            }
        }
        pub fn text(class: css::Class, content: &str) -> Xexpr {
            Xexpr {
                tag: Tag("text".into()),
                attrs: Attrs(vec![("class".into(), class.0.into())]),
                body: Body::Esc(vec![content.into()]),
            }
        }
    }
    #[derive(Clone, PartialEq, Eq, Debug)]
    pub enum Body {
        X(Vec<Xexpr>),
        /// This is only meant to be used with things like the `<style>` tag.
        Css(Vec<css::Rule>),
        Esc(Vec<String>),
        Raw(String),
    }
    impl Body {
        const EMPTY: Body = Body::X(vec![]);
        fn empty() -> Body {
            Body::X(vec![])
        }
        fn is_empty(&self) -> bool {
            match self {
                Body::X(v) => v.len() == 0,
                Body::Css(v) => v.len() == 0,
                Body::Esc(v) => v.len() == 0,
                Body::Raw(s) => s.len() == 0,
            }
        }
    }
    use super::pretty::Spacing;
    impl Xexpr {
        pub fn render_xml(&self) -> String {
            let mut buf = String::new();
            self.write_xml(&mut buf, Spacing::zero()).unwrap();
            buf
        }
        pub fn write_xml(&self, buf: &mut dyn std::fmt::Write, indent: Spacing) -> Result<(), std::fmt::Error> {
            let Xexpr { tag, attrs, body } = self;
            let mut indent = indent;
            if tag.body_has_significant_whitespace() {
                indent = indent.significant_whitespace();
            }
            write!(buf, "<{}", tag.0)?;
            for (k, v) in &attrs.0 {
                write!(buf, " {}=\"{}\"", k, v)?
            }
            if body.is_empty() {
                write!(buf, "/>")?;
            } else {
                write!(buf, ">")?;
                {
                    let indent = if let Body::X(_) = body {
                        indent.reindent()
                    } else {
                        indent
                    };
                    body.write_xml(buf, indent)?;
                }
                if !indent.is_whitespace_significant() {
                    write!(buf, "\n{indent}")?;
                }
                write!(buf, "</{}>", tag.0)?;
            }
            Ok(())
        }
    }
    trait EscStr {
        fn write_escaped(&self, buf: &mut dyn std::fmt::Write, indent: Spacing) -> Result<(), std::fmt::Error>;
    }
    impl EscStr for Vec<String> {
        fn write_escaped(&self, buf: &mut dyn std::fmt::Write, indent: Spacing) -> Result<(), std::fmt::Error> {
            let mut saw_one = false;
            for s in self {
                if saw_one {
                    write!(buf, "\n{indent}")?;
                } else {
                    saw_one = true;
                }
                for c in s.chars() {
                    match c {
                        '<' => write!(buf, "&lt;")?,
                        '>' => write!(buf, "&gt;")?,
                        '&' => write!(buf, "&amp;")?,
                        '"' => write!(buf, "&quot;")?,
                        '\'' => write!(buf, "&apos;")?,
                        c => write!(buf, "{}", c)?,
                    }
                }
            }
            Ok(())
        }
    }
    impl Body {
        pub fn write_xml(&self, buf: &mut dyn std::fmt::Write, indent: Spacing) -> Result<(), std::fmt::Error> {
            match self {
                Body::X(v) => {
                    let mut saw_one = true;
                    for x in v {
                        if saw_one {
                            write!(buf, "\n{indent}")?;
                        } else {
                            saw_one = true;
                        }
                        x.write_xml(buf, indent)?
                    }
                }
                Body::Css(v) => {
                    for r in v {
                        write!(buf, "\n{}", indent)?;
                        r.write_xml(buf, indent)?
                    }
                }
                Body::Esc(v) => { v.write_escaped(buf, indent)? }
                Body::Raw(s) => { write!(buf, "{}", s)? }
            }
            Ok(())
        }
    }
}

impl<L: std::fmt::Display> LayoutAnswer<L> {
    pub fn render_svg(&self) -> xml::Xexpr {
        use attrs::Attrs;
        use css::Rule;
        use xml::{Body, Tag, Xexpr};
        let mut body: Vec<Xexpr> = vec![];

        let style = Xexpr::style(vec![Rule::class("black1", Attrs(vec![("stroke".into(), "black".into()),
                                                                       ("stroke-width".into(), "1".into())])),
                                      Rule::class("sibling", Attrs(vec![("stroke".into(), "orange".into()),
                                                                        ("stroke-width".into(), "1".into()),
                                                                        ("fill".into(), "none".into())])),
                                      Rule::class("child", Attrs(vec![("stroke".into(), "blue".into()),
                                                                      ("stroke-width".into(), "1".into()),
                                                                      ("fill".into(), "none".into())])),
                                      Rule::class("middle_text", Attrs(vec![("text-anchor".into(), "middle".into()),
                                                                            ("dominant-baseline".into(), "middle".into())]))]);
        body.push(style);
        
        // TODO sibling links
        body.push(Xexpr::g(None, vec![]));

        // TODO child links
        body.push(Xexpr::g(None, vec![]));

        fn translate(loc: Xy) -> String {
            format!("translate({}, {})", loc.x, loc.y)
        }
        fn to_node<L: std::fmt::Display>(label: &L, loc: Xy) -> Xexpr {
            use css::Class;
            Xexpr::g(Some(&translate(loc)),
                     vec![Xexpr::circle("15", "white", Class("black1".into())),
                          Xexpr::text(Class("middle_text".into()),
                                      &format!("{}", label))])
        }
        fn accum_nodes<L: std::fmt::Display>(tree: &BinTree<(L, Xy)>, out: &mut Vec<Xexpr>) {
            out.push(to_node(&tree.label.0, tree.label.1));
            if let Some(sibling) = &tree.sibling {
                accum_nodes(sibling, out);
            }
            if let Some(child) = &tree.first_child {
                accum_nodes(child, out);
            }
        }
        accum_nodes(&self.tree, &mut body);
        Xexpr {
            tag: Tag::svg(),
            attrs: attrs::svg_attrs("500", "150", "0 0 500 150", "background_color: white;"),
            body: Body::X(body),
        }
    }
}

fn sect_2_3_2() -> BinTree<char> {
    BinTree::full('A', BinTree::only_child('D',
                                           BinTree::full('E', BinTree::full('F', BinTree::leaf('G'),
                                                                            BinTree::leaf('J')),
                                                         BinTree::leaf('H'))),
                  BinTree::childless('B', BinTree::only_child('C',
                                                              BinTree::leaf('K'))))
}

#[cfg(feature = "s_exp")]
#[test]
fn render_s_exp_tests() {
    assert_eq!(BinTree::leaf('A').render_binary_sexp(), "A");
    assert_eq!(BinTree::only_child('A', BinTree::leaf('B')).render_binary_sexp(), "(A _\n \
                                                                                    B)");
}

#[test]
fn layout_tests() {
    assert_eq!(BinTree::leaf('A').layout(Xy::new(0, 0), Xy::new(1, 1)).tree,
               BinTree::leaf(('A', Xy::new(0, 0))));
    assert_eq!(BinTree::only_child('A',
                                   BinTree::leaf('B')).layout(Xy::new(0, 0), Xy::new(1, 1)).tree,
               BinTree::only_child(('A', Xy::new(0, 0)),
                                   BinTree::leaf(('B', Xy::new(0, 1)))));
    assert_eq!(BinTree::childless('A', BinTree::leaf('C')).layout(Xy::new(0, 0), Xy::new(1, 1)).tree,
               BinTree::childless(('A', Xy::new(0, 0)), BinTree::leaf(('C', Xy::new(1, 0)))));
    assert_eq!(BinTree::full('A', BinTree::leaf('C'),
                             BinTree::childless('B', BinTree::leaf('D'))).layout_naively().tree,
               BinTree::full(('A', Xy::new(0, 0)), BinTree::leaf(('C', Xy::new(1, 0))),
                             BinTree::childless(('B', Xy::new(0, 1)), BinTree::leaf(('D', Xy::new(1, 1))))));
}


// This is a pretty big hack: Instead of trying to implement a smart "fuzzy" comparison between two XML documents
// (that would, e.g., ignore insignicant whitespace), instead we do a dumb (as in naive) "normalization" that
// maps a document into a pseudo-normal form that removes insignificant whitespace, and then apply that normalization
// before doing our comparison for test purposes. Its a bit fragile since the details of the normalization (or rather,
// gaps thereof) can end up leaking into choices in the implementation itself about where, or where not, to insert
// whitespace in order to compensate for this testing hack.
trait Norm { type Out; fn norm(&self) -> Self::Out; fn norm_step(&self) -> Self::Out; }
impl Norm for str {
    type Out = String;
    fn norm(&self) -> String {
        let mut old = self.to_string();
        let mut new = old.norm_step();
        loop {
            if old == new {
                return new;
            }
            let tmp = new.norm_step();
            old = new;
            new = tmp;
        }
    }
    fn norm_step(&self) -> String {
        let old = self.to_string();
        let new = self.replace("\n", "");
        if old != new { return new; }
        let new = self.replace("  ", " ");
        if old != new { return new; }
        let new = self.replace("> <", "><");
        if old != new { return new; }
        return new;
    }
}

#[test]
fn svg_tests() {
    pretty_assertions::assert_eq!(
        LayoutAnswer { tree: BinTree::leaf(('A', Xy::new(0, 0))),
                       bounding_box: Xy::new(500, 150),
        }.render_svg().render_xml().norm(),
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="500" height="150" viewBox="0 0 500 150" style="background_color: white;">

  <style>
  .black1 { stroke: black; stroke-width: 1; }
  .sibling {
    stroke: orange; stroke-width: 1; fill: none;
  }
  .child {
    stroke: blue; stroke-width: 1; fill: none;
  }
  .middle_text {
    text-anchor: middle; dominant-baseline: middle;
  }
  </style>
<g/>
<g/>
<g transform="translate(0, 0)"><circle r="15" fill="white" class="black1"/><text class="middle_text">A</text>
</g>

 </svg>"#.norm());
}

fn main() {
    #[cfg(feature = "s_exp")]
    println!("{}", BinTree::only_child('A', BinTree::leaf('B')).render_binary_sexp());
    println!("{}", sect_2_3_2().render_ascii_art());
    println!("{}", sect_2_3_2().render_graphviz());
    println!("{}", sect_2_3_2().render_mermaid());
    println!("{}", sect_2_3_2().layout_naively().render_svg().render_xml());
}
