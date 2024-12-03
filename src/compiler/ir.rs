//! Abstract Syntax Tree representations.
//!

pub mod ast;
pub mod ops;

use self::ops::Operator;

/// Generic representation of an AST.
///
/// This AST is used as an Intermediate Representation (IR) of expressions that
/// support unary and binary operator expressions.
#[derive(Debug)]
pub enum Node<T> {
    Operand(T),
    UnaryExpr {
        op: Operator,
        child: Box<Self>,
    },
    BinaryExpr {
        op: Operator,
        lhs: Box<Self>,
        rhs: Box<Self>,
    },
}

impl<T> From<T> for Node<T> {
    fn from(value: T) -> Self {
        Node::Operand(value)
    }
}

impl<T> Node<T> {
    pub fn unary<C>(op: Operator, child: C) -> Self
    where
        C: Into<Node<T>>,
    {
        Node::UnaryExpr {
            op,
            child: Box::new(child.into()),
        }
    }

    pub fn binary<L, R>(op: Operator, lhs: L, rhs: R) -> Self
    where
        L: Into<Node<T>>,
        R: Into<Node<T>>,
    {
        Node::BinaryExpr {
            op,
            lhs: Box::new(lhs.into()),
            rhs: Box::new(rhs.into()),
        }
    }
}
