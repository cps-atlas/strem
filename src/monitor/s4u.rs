use std::collections::HashMap;

use itertools::Itertools;

use crate::compiler::ir::ast::{OperandKind, SpatialFormula};
use crate::compiler::ir::ops::{FolOperatorKind, Operator, S4uOperatorKind, SpatialOperatorKind};
use crate::compiler::ir::Node;
use crate::datastream::frame::sample::detections::Annotation;

use super::{s4, s4m};

/// A monitor for evaluating S4u formulas.
///
/// This monitor evaluates against a series of object detection obtained from the
/// perception stream.
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
    ) -> bool {
        match formula {
            Node::Operand(op) => match op {
                OperandKind::Symbol(label) => {
                    if detections.get(label).is_some() {
                        return true;
                    }

                    false
                }
                _ => panic!("monitor: s4u: operand: unsupported `{:?}`", op),
            },
            Node::UnaryExpr { op, child } => match op {
                Operator::SpatialOperator(op) => match op {
                    SpatialOperatorKind::S4uOperator(op) => match op {
                        S4uOperatorKind::NonEmpty => {
                            !s4::Monitor::evaluate(detections, table, child).is_empty()
                        }

                        S4uOperatorKind::Exists(t) => {
                            // For each variable, resolve valuations.
                            //
                            // The valuations of each variable return a, possibly
                            // empty, list of annotations.
                            let mut bindings = Vec::new();

                            for (v, formula) in t.iter() {
                                let mut entries = Vec::new();

                                // Create an entry for each annotation.
                                //
                                // For each annotation retrieved from the
                                // [`formula`], create an entry with its
                                // corresponding variable.
                                for a in s4::Monitor::evaluate(detections, table, formula) {
                                    entries.push((v.clone(), a));
                                }

                                bindings.push(entries);
                            }

                            // For each binding, create a table.
                            //
                            // In this case, we must create all possible
                            // combinations of tables in order to effectively
                            // find a possible satisfying formula.
                            let mut res = Vec::new();
                            for entries in bindings.into_iter().multi_cartesian_product() {
                                // Create a lookup table.
                                //
                                // This table maps a variable to an annotation,
                                // accordingly.
                                let mut lookup: HashMap<String, Annotation> = HashMap::new();

                                if let Some(table) = table {
                                    // Extend the lookup table.
                                    //
                                    // The lookup table needs to check for parent
                                    // lookup tables declared beforehand and
                                    // include them accordingly.
                                    //
                                    // p.s., To resolve name clashes, we use the
                                    // the most recent name (i.e., the youngest
                                    // lookup table).
                                    for (v, annotation) in table.iter() {
                                        lookup.insert(v.clone(), annotation.clone());
                                    }
                                }

                                // Insert the most recent entries.
                                //
                                // This ensures that the most recent definitions
                                // are used, accordingly.
                                for (v, annotation) in entries.iter() {
                                    lookup.insert(v.clone(), annotation.clone());
                                }

                                res.push(Monitor::evaluate(detections, Some(&lookup), child));
                            }

                            res.iter().any(|x| *x)
                        }

                        S4uOperatorKind::Forall(t) => {
                            // For each variable, resolve valuations.
                            //
                            // The valuations of each variable return a, possibly
                            // empty, list of annotations.
                            let mut bindings = Vec::new();

                            for (v, formula) in t.iter() {
                                let mut entries = Vec::new();

                                // Create an entry for each annotation.
                                //
                                // For each annotation retrieved from the
                                // [`formula`], create an entry with its
                                // corresponding variable.
                                for a in s4::Monitor::evaluate(detections, table, formula) {
                                    entries.push((v.clone(), a));
                                }

                                bindings.push(entries);
                            }

                            // For each binding, create a table.
                            //
                            // In this case, we must create all possible
                            // combinations of tables in order to effectively
                            // find a possible satisfying formula.
                            let mut res = Vec::new();
                            for entries in bindings.into_iter().multi_cartesian_product() {
                                // Create a lookup table.
                                //
                                // This table maps a variable to an annotation,
                                // accordingly.
                                let mut lookup: HashMap<String, Annotation> = HashMap::new();

                                if let Some(table) = table {
                                    // Extend the lookup table.
                                    //
                                    // The lookup table needs to check for parent
                                    // lookup tables declared beforehand and
                                    // include them accordingly.
                                    //
                                    // p.s., To resolve name clashes, we use the
                                    // the most recent name (i.e., the youngest
                                    // lookup table).
                                    for (v, annotation) in table.iter() {
                                        lookup.insert(v.clone(), annotation.clone());
                                    }
                                }

                                // Insert the most recent entries.
                                //
                                // This ensures that the most recent definitions
                                // are used, accordingly.
                                for (v, annotation) in entries.iter() {
                                    lookup.insert(v.clone(), annotation.clone());
                                }

                                res.push(Monitor::evaluate(detections, Some(&lookup), child));
                            }

                            if res.is_empty() {
                                return false;
                            }

                            res.iter().all(|x| *x)
                        }
                    },
                    SpatialOperatorKind::FolOperator(op) => match op {
                        FolOperatorKind::Negation => {
                            let res = Monitor::evaluate(detections, table, child);
                            !res
                        }
                        _ => panic!("monitor: s4u: unrecognized unary FOL operator"),
                    },
                    _ => panic!("monitor: s4u: unrecognized unary operator"),
                },
                _ => panic!("monitor: s4u: unrecognized unary operator"),
            },
            Node::BinaryExpr { op, lhs, rhs } => match op {
                Operator::SpatialOperator(kind) => match kind {
                    SpatialOperatorKind::FolOperator(kind) => match kind {
                        FolOperatorKind::Conjunction => {
                            let lhs = Monitor::evaluate(detections, table, lhs);
                            let rhs = Monitor::evaluate(detections, table, rhs);

                            lhs && rhs
                        }
                        FolOperatorKind::Disjunction => {
                            let lhs = Monitor::evaluate(detections, table, lhs);
                            let rhs = Monitor::evaluate(detections, table, rhs);

                            lhs || rhs
                        }
                        FolOperatorKind::LessThan => {
                            let lhs = s4m::Monitor::evaluate(detections, table, lhs);
                            let rhs = s4m::Monitor::evaluate(detections, table, rhs);

                            // Compute the comparison of all possible options.
                            //
                            // In order to accurately get a result, all
                            // combinations of comparisons are needed.
                            for l in lhs.iter() {
                                for r in rhs.iter() {
                                    if l < r {
                                        return true;
                                    }
                                }
                            }

                            false
                        }
                        FolOperatorKind::GreaterThan => {
                            let lhs = s4m::Monitor::evaluate(detections, table, lhs);
                            let rhs = s4m::Monitor::evaluate(detections, table, rhs);

                            // Compute the comparison of all possible options.
                            //
                            // In order to accurately get a result, all
                            // combinations of comparisons are needed.
                            for l in lhs.iter() {
                                for r in rhs.iter() {
                                    if l > r {
                                        return true;
                                    }
                                }
                            }

                            false
                        }
                        FolOperatorKind::LessThanEqualTo => {
                            let lhs = s4m::Monitor::evaluate(detections, table, lhs);
                            let rhs = s4m::Monitor::evaluate(detections, table, rhs);

                            // Compute the comparison of all possible options.
                            //
                            // In order to accurately get a result, all
                            // combinations of comparisons are needed.
                            for l in lhs.iter() {
                                for r in rhs.iter() {
                                    if l <= r {
                                        return true;
                                    }
                                }
                            }

                            false
                        }
                        FolOperatorKind::GreaterThanEqualTo => {
                            let lhs = s4m::Monitor::evaluate(detections, table, lhs);
                            let rhs = s4m::Monitor::evaluate(detections, table, rhs);

                            // Compute the comparison of all possible options.
                            //
                            // In order to accurately get a result, all
                            // combinations of comparisons are needed.
                            for l in lhs.iter() {
                                for r in rhs.iter() {
                                    if l >= r {
                                        return true;
                                    }
                                }
                            }

                            false
                        }
                        _ => panic!("monitor: unkown FOL operator {:#?}", kind),
                    },
                    _ => panic!("monitor: unknown binary operator {:#?}", kind),
                },
                _ => panic!("monitor: unknown binary operator {:#?}", op),
            },
        }
    }
}
