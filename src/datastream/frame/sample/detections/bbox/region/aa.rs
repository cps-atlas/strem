use super::Point;

/// An Axis-Aligned Region.
///
/// The selected representation of the region uses the major and minor coordinates
/// (i.e., the corners) to represent the rectangle.
#[derive(Clone, Debug)]
pub struct Region {
    pub min: Point,
    pub max: Point,
}

impl Region {
    /// Create a new [`Region`].
    pub fn new(center: Point, width: f64, height: f64) -> Self {
        let min = Point::new(center.x - (width / 2.0), center.y - (height / 2.0));
        let max = Point::new(center.x + (width / 2.0), center.y + (height / 2.0));

        Region { min, max }
    }

    /// Compute the center point of the Axis-Aligned region.
    ///
    /// This calculates the x and y component of the coordinate and wraps it
    /// into a [`Point`].
    #[inline]
    pub fn center(&self) -> Point {
        Point::new(
            self.min.x + ((self.width()) / 2.0),
            self.min.y + ((self.height()) / 2.0),
        )
    }

    /// Compute the width of the Axis-Aligned region.
    #[inline]
    pub fn width(&self) -> f64 {
        self.max.x - self.min.x
    }

    /// Compute the height of the Axis-Aligned region.
    #[inline]
    pub fn height(&self) -> f64 {
        self.max.y - self.min.y
    }

    /// Compute the intersection of a [`Region`].
    ///
    /// This computes the intersection between two Axis-Aligned regions,
    /// accordingly
    pub fn intersects(&self, other: &Region) -> Option<Region> {
        let a = &self;
        let b = &other;

        if a.min.x < b.max.x && b.min.x < a.max.x && a.min.y < b.max.y && b.min.y < a.max.y {
            let min = Point::new(
                std::cmp::max(a.min.x as i64, b.min.x as i64) as f64,
                std::cmp::max(a.min.y as i64, b.min.y as i64) as f64,
            );

            let max = Point::new(
                std::cmp::min(a.max.x as i64, b.max.x as i64) as f64,
                std::cmp::min(a.max.y as i64, b.max.y as i64) as f64,
            );

            return Some(Region { min, max });
        }

        None
    }
}
