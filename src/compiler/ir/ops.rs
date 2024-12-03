use std::collections::HashMap;

use super::ast::SpatialFormula;

/// Operations kinds supported.
#[derive(Debug)]
pub enum Operator {
    RegexOperator(RegexOperatorKind),
    SpatialOperator(SpatialOperatorKind),
}

/// The set of Regular Expression operations allowed in a query.
#[derive(Debug)]
pub enum RegexOperatorKind {
    KleeneStar,
    Concatenation,
    Alternation,
    Range(RangeKind),
}

/// Range operator kinds.
#[derive(Debug)]
pub enum RangeKind {
    Exactly(usize),
    AtLeast(usize),
    Between(usize, usize),
}

/// The set of spatial operations allowed against a frame.
///
/// These operators must be used within the `[]` enclosures. In addition, the
/// syntax for these operators may be the same as the syntax for some
/// non-spatial expressions (e.g., alternation and disjunction). Therefore,
/// these enumerations provide semantic meaning for symbolically
/// equivalent operators.
#[derive(Debug)]
pub enum SpatialOperatorKind {
    FolOperator(FolOperatorKind),
    SolOperator(SolOperatorKind),
    S4uOperator(S4uOperatorKind),
    S4mOperator(S4mOperatorKind),
    S4Operator(S4OperatorKind),
}

/// First-Order Logic operators.
///
/// For more information on FOL, please see:
/// [Stanford Encyclopedia of Philosophy: Classical Logic](https://plato.stanford.edu/entries/logic-classical/)
#[derive(Debug)]
pub enum FolOperatorKind {
    Negation,
    Conjunction,
    Disjunction,
    LessThan,
    GreaterThan,
    LessThanEqualTo,
    GreaterThanEqualTo,
}

/// Second-Order Logic operators.
///
/// For more information on SOL, please see:
/// [Stanford Encyclopedia of Philosophy: Second-order and Higher-order logic](https://plato.stanford.edu/entries/logic-higher-order/)
#[derive(Debug)]
pub enum SolOperatorKind {
    Exists,
}

/// S4u operators.
///
/// For more information on S4, please see:
/// [Combining Spatial and Temporal Logics: Expressiveness vs. Complexity](https://arxiv.org/abs/1)
#[derive(Debug)]
pub enum S4uOperatorKind {
    NonEmpty,
    Exists(HashMap<String, SpatialFormula>),
    Forall(HashMap<String, SpatialFormula>),
}

/// S4m operators.
///
/// For more information on S4m, please see:
///
#[derive(Debug)]
pub enum S4mOperatorKind {
    Function(String),
    Inverse,
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

/// S4 operators.
///
/// For more information on S4, please see:
/// [Combining Spatial and Temporal Logics: Expressiveness vs. Complexity](https://arxiv.org/abs/1110.2726)
#[derive(Debug)]
pub enum S4OperatorKind {
    Intersection,
    Union,
    Complement,
}
