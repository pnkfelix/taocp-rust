#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Op {
    /// ⊥: 0; Contradiction; falsehood; antilogy; constant 0
    False = 0b0000,
    /// ∧: xy, x∧y, xs1&y;  Conjunction; and
    And = 0b0001,
    /// ⊃̄: x∧ȳ, x⊅y, [x>y], x∸y; Nonimplication; difference; but not
    ButNot = 0b0010,
    /// Ｌ: x; Left projection; first dictator
    Left = 0b0011,
    /// ⊂̄: x̄∧y, x⊄y, [x<y], y∸x; Converse nonimplication; not ... but
    NotBut = 0b0100,
    /// Ｒ: y; Right projection; second dictator
    Right = 0b0101,
    /// ⊕: x⊕y, x≢y, x^y; Exclusive disjunction; nonequivalence; “xor”
    Xor = 0b0110,
    /// ∨: x∨y, x|y; (Inclusive) disjunction; or; and/or
    Or = 0b0111,
    /// ⊽: x̄∧ȳ, x⊽y, x↓y; Nondisjunction; joint denial; neither ... nor
    Nor = 0b1000,
    /// ≡: x≡y, x↔y, x⇔y; Equivalence; if and only if; “iff”
    Equiv = 0b1001,
    /// Ｒ̄: ȳ, ¬y, !y, ~y; Right complementation
    NotRight = 0b1010,
    /// ⊂: x∨ȳ, x⊂y, x⇐y, [x≥y], xʸ; Converse implication; if
    If = 0b1011,
    /// Ｌ̄: x̄, ¬x, !x, ~x; Left complementation
    NotLeft = 0b1100,
    /// ⊃: x̄∨y, x⊃y, x⇒y, [x≤y], yˣ; Implication; only if; if . . . then
    OnlyIf = 0b1101,
    /// ⊼: x̄∨ȳ, x⊼y, x|y; Nonconjunction; not both . . . and; “nand”
    Nand = 0b1110,
    /// ⊤: 1; Affirmation; validity; tautology; constant 1
    True = 0b1111,
}

type Uvar = u32;

struct X(Uvar);

struct Step(X, Op, X);

struct Chain {
    vars: Uvar,
    steps: Vec<Step>,
}

#[test]
fn s7_1_2_eqn2() {
    let mux1 = Chain {
        vars: 3,
        steps: vec![Step(X(1), Op::And, X(2)),
                    Step(X(1), Op::NotBut, X(3)),
                    Step(X(4), Op::Or, X(5))],
    };
    let mux2 = Chain {
        vars: 3,
        steps: vec![Step(X(2), Op::Xor, X(3)),
                    Step(X(1), Op::And, X(4)),
                    Step(X(3), Op::Xor, X(5))],
    };
}

enum Tree {
    Leaf(X),
    Node(Op, Box<Tree>, Box<Tree>),
}

impl From<X> for Box<Tree> {
    fn from(x: X) -> Self { Box::new(Tree::Leaf(x)) }
}

impl Op {
    fn tree(self, l: impl Into<Box<Tree>>, r: impl Into<Box<Tree>>) -> Tree {
        Tree::Node(self, l.into(), r.into())
    }
}

#[test]
fn s7_1_2_eqn4() {
    let mux1 = Op::Or.tree(Op::And.tree(X(1), X(2)),
                           Op::NotBut.tree(X(1), X(3)));
    let mux2 = Op::Xor.tree(X(3),
                            Op::And.tree(X(1),
                                         Op::Xor.tree(X(2), X(3))));
}

#[test]
fn s7_1_2_eqn5_6() {
    // truth table: 1100 1001 0000 1111
    // DNF: (x̄₁∧x̄₂∧x̄₃)∨(x̄₁∧x̄₃∧x̄₄)∨(x₂∧x₃∧x₄)∨(x₁∧x₂)
    // chain: (((x₂∧x̄₄)⊕x̄₃)∧x̄₁)⊕x₂
}

fn main() {
    println!("Hello, world!");
}
