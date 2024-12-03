use super::Point;

/// An Oriented Region.
///
/// The selected representation of the region uses the four coordinates of the
/// region (i.e., the corners) to represent the rectangle.
#[derive(Clone, Debug)]
pub struct Region {
    pub tl: Point,
    pub tr: Point,
    pub br: Point,
    pub bl: Point,
}

impl Region {
    /// Create a new [`Region`].
    ///
    /// This accepts a standard representation of an Oriented region and
    /// transforms it into a more easier to use representation (i.e., a rectangle
    /// composed of four coordinates).
    pub fn new(center: Point, width: f64, height: f64, rotation: f64) -> Self {
        let x = width / 2.0;
        let y = height / 2.0;

        Region {
            tl: Point::new(
                center.x + ((-x * f64::cos(rotation)) - (-y * f64::sin(rotation))),
                center.y + ((-x * f64::sin(rotation)) + (-y * f64::cos(rotation))),
            ),
            tr: Point::new(
                center.x + ((x * f64::cos(rotation)) - (-y * f64::sin(rotation))),
                center.y + ((x * f64::sin(rotation)) + (-y * f64::cos(rotation))),
            ),
            br: Point::new(
                center.x + ((x * f64::cos(rotation)) - (y * f64::sin(rotation))),
                center.y + ((x * f64::sin(rotation)) + (y * f64::cos(rotation))),
            ),
            bl: Point::new(
                center.x + ((-x * f64::cos(rotation)) - (y * f64::sin(rotation))),
                center.y + ((-x * f64::sin(rotation)) + (y * f64::cos(rotation))),
            ),
        }
    }

    /// Compute the center point of the Oriented region.
    ///
    /// This calculates the x and y component of the coordinate and wraps it
    /// into a [`Point`].
    #[inline]
    pub fn center(&self) -> Point {
        Point::new((self.tl.x + self.br.x) / 2.0, (self.tl.y + self.br.y) / 2.0)
    }

    /// Compute the width of the Oriented region.
    #[inline]
    pub fn width(&self) -> f64 {
        f64::sqrt((self.tr.x - self.tl.x).powi(2) + (self.tr.y - self.tl.y).powi(2))
    }

    /// Compute the height of the Oriented region.
    #[inline]
    pub fn height(&self) -> f64 {
        f64::sqrt((self.tl.x - self.bl.x).powi(2) + (self.tl.y - self.bl.y).powi(2))
    }

    /// Compute the rotation of the Oriented region.
    #[inline]
    pub fn rotation(&self) -> f64 {
        f64::atan2(self.tr.y - self.tl.y, self.tr.x - self.tl.x)
    }

    /// Compute the intersection of a [`Region`].
    ///
    /// This computes the intersection between two Oriented regions,
    /// accordingly
    #[allow(unused_variables)]
    pub fn intersects(&self, other: &Region) -> Option<Region> {
        unimplemented!("support for intersection of oriented regions coming soon!")
    }
}

#[cfg(test)]
mod tests {
    use crate::datastream::frame::sample::detections::bbox::region::Point;

    use super::Region;

    #[test]
    fn region_transformation() {
        let region = Region::new(Point::new(0.0, 0.0), 10.0, 10.0, 0.0);

        assert_eq!(region.center().x, 0.0);
        assert_eq!(region.center().y, 0.0);
        assert_eq!(region.width(), 10.0);
        assert_eq!(region.height(), 10.0);
        assert_eq!(region.rotation(), 0.0);
    }
}
