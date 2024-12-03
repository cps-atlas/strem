use std::collections::HashMap;

use crate::{
    compiler::ir::{
        ast::{OperandKind, SpatialFormula},
        ops::{Operator, S4mOperatorKind, SpatialOperatorKind},
        Node,
    },
    datastream::frame::sample::detections::{bbox::BoundingBox, Annotation},
};

use super::s4;

/// A monitor for evaluating S4m expressions.
///
/// This monitor evaluates against a series of object detections obtained from the
/// perception stream.
#[derive(Default)]
pub struct Monitor {}

impl Monitor {
    /// Create a new [`Monitor`]
    pub fn new() -> Self {
        Monitor {}
    }

    /// Evaluate the formula against the set of annotations.
    ///
    /// This returns a set of possible real numbers obtained from evaluating the
    /// expression, accordingly.
    pub fn evaluate(
        detections: &HashMap<String, Vec<Annotation>>,
        table: Option<&HashMap<String, Annotation>>,
        formula: &SpatialFormula,
    ) -> Vec<f64> {
        match formula {
            Node::Operand(op) => match op {
                OperandKind::Number(num) => vec![*num],
                _ => panic!("monitor: s4m: operand: unsupported `{:?}`", op),
            },
            Node::UnaryExpr { op, child } => match op {
                Operator::SpatialOperator(op) => match op {
                    SpatialOperatorKind::S4mOperator(op) => match op {
                        S4mOperatorKind::Inverse => {
                            let res = Monitor::evaluate(detections, table, child);
                            res.iter().map(|x| -x).collect()
                        }
                        S4mOperatorKind::Function(name) => match &name[..] {
                            // Retrieve the x-coordinate value.
                            //
                            // The direction that the x-axis represents is
                            // entirely dependent on the format/representation
                            // selected by the user.
                            "x" => {
                                let annotations = s4::Monitor::evaluate(detections, table, child);

                                let mut res = Vec::new();
                                for annotation in annotations.iter() {
                                    let center = match &annotation.bbox {
                                        BoundingBox::AxisAligned(region) => region.center(),
                                        BoundingBox::Oriented(region) => region.center(),
                                    };

                                    res.push(center.x);
                                }

                                res
                            }

                            // Retrieve the y-coordinate value.
                            //
                            // The direction that the y-axis represents is
                            // entirely dependent on the format/representation
                            // selected by the user.
                            "y" => {
                                let annotations = s4::Monitor::evaluate(detections, table, child);

                                let mut res = Vec::new();
                                for annotation in annotations.iter() {
                                    let center = match &annotation.bbox {
                                        BoundingBox::AxisAligned(region) => region.center(),
                                        BoundingBox::Oriented(region) => region.center(),
                                    };

                                    res.push(center.y);
                                }

                                res
                            }

                            // Compute the distance from an annotation to origin.
                            //
                            // This is equivalent to computing the Euclidean
                            // distance between a bounding box and the origin
                            // point of the space.
                            "dist" => {
                                let annotations = s4::Monitor::evaluate(detections, table, child);

                                let mut res = Vec::new();
                                for annotation in annotations.iter() {
                                    let center = match &annotation.bbox {
                                        BoundingBox::AxisAligned(region) => region.center(),
                                        BoundingBox::Oriented(region) => region.center(),
                                    };

                                    res.push(f64::sqrt((center.x).powi(2) + (center.y).powi(2)));
                                }

                                res
                            }

                            // Compute the area of the annotation.
                            //
                            // This works only on 2D-based bounding boxes such as
                            // Axis-Aligned or Oriented.
                            "area" => {
                                let annotations = s4::Monitor::evaluate(detections, table, child);

                                let mut res = Vec::new();
                                for annotation in annotations.iter() {
                                    let area = match &annotation.bbox {
                                        BoundingBox::AxisAligned(region) => {
                                            region.width() * region.height()
                                        }
                                        BoundingBox::Oriented(region) => {
                                            region.width() * region.height()
                                        }
                                    };

                                    res.push(area);
                                }

                                res
                            }
                            _ => panic!(
                                "monitor: s4m: unary: operator: function not supported: `{}`",
                                name
                            ),
                        },
                        _ => panic!("monitor: s4m: unary: operator: unsupported `{:?}`", op),
                    },
                    _ => panic!("monitor: s4m: unary: operator: unsupported `{:?}`", op),
                },
                _ => panic!("monitor: s4m: unary: operator: unsupported `{:?}`", op),
            },
            Node::BinaryExpr { op, lhs, rhs } => match op {
                Operator::SpatialOperator(op) => match op {
                    SpatialOperatorKind::S4mOperator(op) => match op {
                        S4mOperatorKind::Addition => {
                            let lhs = Monitor::evaluate(detections, table, lhs);
                            let rhs = Monitor::evaluate(detections, table, rhs);

                            // Compute the addition of all possibilities.
                            //
                            // In order to accurately get a result, we need to
                            // compute the addition of all possible numbers.
                            let mut res = Vec::new();

                            for l in lhs.iter() {
                                for r in rhs.iter() {
                                    res.push(l + r);
                                }
                            }

                            res
                        }
                        S4mOperatorKind::Subtraction => {
                            let lhs = Monitor::evaluate(detections, table, lhs);
                            let rhs = Monitor::evaluate(detections, table, rhs);

                            // Compute the subtraction of all possibilities.
                            //
                            // In order to accurately get a result, we need to
                            // compute the subtraction of all possible numbers.
                            let mut res = Vec::new();

                            for l in lhs.iter() {
                                for r in rhs.iter() {
                                    res.push(l - r);
                                }
                            }

                            res
                        }
                        S4mOperatorKind::Multiplication => {
                            let lhs = Monitor::evaluate(detections, table, lhs);
                            let rhs = Monitor::evaluate(detections, table, rhs);

                            // Compute the multiplication of all possibilities.
                            //
                            // In order to accurately get a result, we need to
                            // compute the multiplication of all possible numbers.
                            let mut res = Vec::new();

                            for l in lhs.iter() {
                                for r in rhs.iter() {
                                    res.push(l * r);
                                }
                            }

                            res
                        }
                        S4mOperatorKind::Division => {
                            let lhs = Monitor::evaluate(detections, table, lhs);
                            let rhs = Monitor::evaluate(detections, table, rhs);

                            // Compute the division of all possibilities.
                            //
                            // In order to accurately get a result, we need to
                            // compute the division of all possible numbers.
                            let mut res = Vec::new();

                            for l in lhs.iter() {
                                for r in rhs.iter() {
                                    res.push(l / r);
                                }
                            }

                            res
                        }
                        S4mOperatorKind::Function(name) => match &name[..] {
                            // Compute the distance from an annotation to another
                            // annotation.
                            //
                            // This is equivalent to computing the Euclidean
                            // distance between a bounding box and another
                            // bounding box in space.
                            "dist" => {
                                let lhs = s4::Monitor::evaluate(detections, table, lhs);
                                let rhs = s4::Monitor::evaluate(detections, table, rhs);

                                let mut res = Vec::new();

                                for l in lhs.iter() {
                                    for r in rhs.iter() {
                                        if let Some(distance) = self::euclidean(&l.bbox, &r.bbox) {
                                            res.push(distance)
                                        }
                                    }
                                }
                                res
                            }
                            _ => panic!(
                                "monitor: s4m: binary: operator: function not supported: `{}`",
                                name
                            ),
                        },
                        _ => panic!("monitor: s4m: binary: operator: unsupported `{:?}`", op),
                    },
                    _ => panic!("monitor: s4m: binary: operator: unsupported `{:?}`", op),
                },
                _ => panic!("monitor: s4m: binary: operator: unsupported `{:?}`", op),
            },
        }
    }
}

/// Compute the Euclidean distance between [`BoundingBox`].
///
/// This performs a distance computation based on the center point of the
/// relevant bounding boxes, accordingly.
fn euclidean(a: &BoundingBox, b: &BoundingBox) -> Option<f64> {
    if let BoundingBox::AxisAligned(a) = a {
        if let BoundingBox::AxisAligned(b) = b {
            let a = a.center();
            let b = b.center();

            return Some(f64::sqrt((b.x - a.x).powi(2) + (b.y - a.y).powi(2)));
        }
    }

    if let BoundingBox::AxisAligned(a) = a {
        if let BoundingBox::Oriented(b) = b {
            let a = a.center();
            let b = b.center();

            return Some(f64::sqrt((b.x - a.x).powi(2) + (b.y - a.y).powi(2)));
        }
    }

    if let BoundingBox::Oriented(a) = a {
        if let BoundingBox::AxisAligned(b) = b {
            let a = a.center();
            let b = b.center();

            return Some(f64::sqrt((b.x - a.x).powi(2) + (b.y - a.y).powi(2)));
        }
    }

    if let BoundingBox::Oriented(a) = a {
        if let BoundingBox::Oriented(b) = b {
            let a = a.center();
            let b = b.center();

            return Some(f64::sqrt((b.x - a.x).powi(2) + (b.y - a.y).powi(2)));
        }
    }

    None
}
