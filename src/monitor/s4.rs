use std::collections::HashMap;

use crate::compiler::ir::ast::{OperandKind, SpatialFormula};
use crate::compiler::ir::ops::{Operator, S4OperatorKind, SpatialOperatorKind};
use crate::compiler::ir::Node;
use crate::datastream::frame::sample::detections::Annotation;

/// A monitor for evaluating S4 formulas.
#[derive(Default)]
pub struct Monitor {}

impl Monitor {
    pub fn new() -> Self {
        Self {}
    }

    /// Evaluate formula satisfaction against set of annotations.
    ///
    /// This returns is a boolean result. If true, the formula is satisifed;
    /// else, if false, then it is not satisfied.
    pub fn evaluate(
        detections: &HashMap<String, Vec<Annotation>>,
        table: Option<&HashMap<String, Annotation>>,
        formula: &SpatialFormula,
    ) -> Vec<Annotation> {
        match formula {
            Node::Operand(op) => match op {
                OperandKind::Symbol(label) => {
                    // Retrieve an annotation with the same class category as
                    // specified by the label.
                    if let Some(annotations) = detections.get(label) {
                        return annotations.clone();
                    }

                    Vec::new()
                }
                OperandKind::Variable(name) => {
                    // Retrieve annoation by look-up.
                    //
                    // If no entry exists on the table, return an empty list,
                    // accordingly.
                    if let Some(table) = table {
                        if let Some(annotation) = table.get(name) {
                            return vec![annotation.clone()];
                        }
                    }

                    Vec::new()
                }
                _ => panic!("monitor: s4: operand: unsupported `{:?}`", op),
            },
            Node::UnaryExpr { op, .. } => match op {
                Operator::SpatialOperator(SpatialOperatorKind::S4Operator(
                    S4OperatorKind::Complement,
                )) => {
                    todo!()
                }
                _ => panic!("monitor: s4: unrecognized unary operator"),
            },
            Node::BinaryExpr { op, lhs, rhs } => {
                let lhs = Monitor::evaluate(detections, table, lhs);
                let rhs = Monitor::evaluate(detections, table, rhs);

                match op {
                    Operator::SpatialOperator(op) => match op {
                        SpatialOperatorKind::S4Operator(op) => match op {
                            S4OperatorKind::Intersection => {
                                // If either left or rhs is empty, then one
                                // side is not satisfied. Therefore, the
                                // resulting formula is not satisifed, entirely.
                                if lhs.is_empty() || rhs.is_empty() {
                                    return Vec::new();
                                }

                                let mut intersections = Vec::new();

                                for l in lhs.iter() {
                                    for r in rhs.iter() {
                                        if l.bbox.intersects(&r.bbox).is_some() {
                                            intersections.push(l.clone());
                                            intersections.push(r.clone());
                                        }
                                    }
                                }

                                intersections
                            }
                            S4OperatorKind::Union => {
                                // We don't care which one satisfied---just as
                                // long as left or right is valid. Therefore, we
                                // append all solutions.
                                lhs.into_iter().chain(rhs).collect()
                            }
                            _ => panic!("monitor: s4: unknown binary operator"),
                        },
                        _ => panic!("monitor: unknown binary operator {:#?}", op),
                    },
                    _ => panic!("monitor: unknown binary operator {:#?}", op),
                }
            }
        }
    }
}
