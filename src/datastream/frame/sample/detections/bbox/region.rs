pub mod aa;
pub mod oriented;

/// A Z axis-aligned point (i.e., 2D).
#[derive(Clone, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    /// Create a new [`Point`] with (x, y) coordinates.
    pub fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }
}
