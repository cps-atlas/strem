//! The matching framework for SpREs.
//!

use std::error::Error;

use crate::compiler::ir::ops::{Operator, RangeKind, RegexOperatorKind};
use crate::compiler::ir::Node;
use crate::datastream::frame::Frame;
use crate::symbolizer::ast::{SymbolicAbstractSyntaxTree, SymbolicFormula};

pub mod automata;
pub mod offline;
pub mod online;

/// A trait for which all matchers must implement.
///
/// This is defined to provide a ubiquitous interface for all matchers to adhere
/// to for simplicity of switching (e.g., facade pattern).
pub trait Matching {
    /// Find a possible leftmost [`Match`] from the set of [`Frame`].
    fn leftmost(&self, frames: &[Frame]) -> Result<Option<Match>, Box<dyn Error>>;
}

/// A range of valid indices.
///
/// It should be noted that `start` is inclusive (closed) while `end` is
/// exclusive (open); so a [`Match`] takes the form: [start, end). This is also
/// referred to as a half-open interval.
#[derive(Debug)]
pub struct Match {
    pub start: usize,
    pub end: usize,
}

impl Match {
    /// Create a new complete [`Match`] with start and end indices.
    pub fn new(start: usize, end: usize) -> Self {
        Match { start, end }
    }
}

/// Construct a Regular Expression (RE) pattern from a [`SymbolicAbstractSyntaxTree`].
///
/// This traverses the outer components of a SpRE related solely to the RE-based
/// patterns and symbols.
pub fn regexify(ast: &SymbolicAbstractSyntaxTree) -> String {
    if let Some(root) = &ast.root {
        return self::regexit(root);
    }

    String::new()
}

/// Recursively construct an RE.
///
/// This is the helper function that walks the root [`Node`] of a
/// [`SymbolicAbstractSyntaxTree`] to build the appropriate pattern.
fn regexit(node: &Node<SymbolicFormula>) -> String {
    match node {
        Node::Operand(formula) => String::from(formula.symbol),
        Node::UnaryExpr { op, child } => {
            let child = self::regexit(child);

            match op {
                Operator::RegexOperator(kind) => match kind {
                    RegexOperatorKind::KleeneStar => format!("({}*)", child),
                    RegexOperatorKind::Range(kind) => match kind {
                        RangeKind::Exactly(size) => format!("({}{{{}}})", child, size),
                        RangeKind::AtLeast(min) => format!("({}{{{},}})", child, min),
                        RangeKind::Between(min, max) => format!("({}{{{},{}}})", child, min, max),
                    },
                    _ => String::new(),
                },
                _ => String::new(),
            }
        }
        Node::BinaryExpr { op, lhs, rhs } => {
            let lhs = self::regexit(lhs);
            let rhs = self::regexit(rhs);

            match op {
                Operator::RegexOperator(kind) => match kind {
                    RegexOperatorKind::Concatenation => format!("({}{})", lhs, rhs),
                    RegexOperatorKind::Alternation => format!("({}|{})", lhs, rhs),
                    _ => String::new(),
                },
                _ => String::new(),
            }
        }
    }
}

/// Compute the horizon of a Regular Expression (RE).
///
/// This traverses the outer components of a SpRE related solely to the RE-based
/// patterns and symbols.
pub fn horizon(ast: &SymbolicAbstractSyntaxTree) -> Option<usize> {
    if let Some(root) = &ast.root {
        return self::horizonit(root);
    }

    None
}

/// Recursively compute the horizon of an RE.
///
/// This is a helper function that walks the root [`Node`] of a
/// [`SymbolicAbstractSyntaxTree`] to build the appropriate pattern.
fn horizonit(node: &Node<SymbolicFormula>) -> Option<usize> {
    match node {
        Node::Operand(..) => Some(1),
        Node::UnaryExpr { op, child } => {
            let ret = self::horizonit(child);

            match op {
                Operator::RegexOperator(kind) => match kind {
                    RegexOperatorKind::KleeneStar => None,
                    RegexOperatorKind::Range(kind) => match kind {
                        RangeKind::Exactly(size) => {
                            if let Some(ret) = ret {
                                return Some(ret * (*size));
                            }

                            None
                        }
                        RangeKind::AtLeast(..) => None,
                        RangeKind::Between(.., max) => {
                            if let Some(ret) = ret {
                                return Some(ret * (*max));
                            }

                            None
                        }
                    },
                    _ => None,
                },
                _ => None,
            }
        }
        Node::BinaryExpr { op, lhs, rhs } => {
            let lhs = self::horizonit(lhs);
            let rhs = self::horizonit(rhs);

            match op {
                Operator::RegexOperator(kind) => match kind {
                    RegexOperatorKind::Concatenation => {
                        if let Some(lhs) = lhs {
                            if let Some(rhs) = rhs {
                                return Some(lhs + rhs);
                            }
                        }

                        None
                    }
                    RegexOperatorKind::Alternation => {
                        if let Some(lhs) = lhs {
                            if let Some(rhs) = rhs {
                                return Some(std::cmp::max(lhs, rhs));
                            }
                        }

                        None
                    }
                    _ => None,
                },
                _ => None,
            }
        }
    }
}
