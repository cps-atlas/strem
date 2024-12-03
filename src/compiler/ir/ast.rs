//! Abstract Syntax Tree (AST) representation.
//!

use super::super::ir::Node;

pub type SpatialFormula = Node<OperandKind>;

/// The operands within the AST.
///
/// These kinds of operands are equivalent to the types of data that is stored on
/// the leaf nodes of the AST.
#[derive(Debug)]
pub enum OperandKind {
    Symbol(String),
    Number(f64),
    Variable(String),
}

#[derive(Debug)]
pub struct AbstractSyntaxTree {
    pub root: Option<Node<SpatialFormula>>,
}

impl AbstractSyntaxTree {
    pub fn new(root: Option<Node<SpatialFormula>>) -> Self {
        Self { root }
    }
}
