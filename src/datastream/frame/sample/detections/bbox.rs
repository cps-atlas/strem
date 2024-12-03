pub mod region;

#[derive(Clone, Debug)]
pub enum BoundingBox {
    /// An Axis-Aligned Bounding Box (AABB) annotation.
    AxisAligned(region::aa::Region),

    /// An Oriented Bounding Box (OBB) annotation.
    Oriented(region::oriented::Region),
}

impl BoundingBox {
    /// Compute the intersection of a [`BoundingBox`].
    ///
    /// This is a general function that attempts to compute the intersection
    /// between different types of [`BoundingBox`] kinds, accordingly.
    ///
    /// TODO: Support intersection computation between two different kinds of
    /// bounding boxes if possible (e.g., AABB and OBB).
    pub fn intersects(&self, other: &BoundingBox) -> Option<BoundingBox> {
        // Compute the intersection between two Axis-Aligned Bounding Boxes.
        //
        // This requires that both bounding boxes are AABBs in order to
        // effectively compute the intersection.
        if let BoundingBox::AxisAligned(a) = &self {
            if let BoundingBox::AxisAligned(b) = &other {
                // Compute the intersection.
                //
                // We first check if an intersection exists. If yes, then we
                // return the appropriate bounding box wrapping the region.
                if let Some(region) = a.intersects(b) {
                    return Some(BoundingBox::AxisAligned(region));
                }
            }

            return None; // exit early
        }

        // Compute the intersection between two Oriented Bounding Boxes.
        //
        // This requires that both bounding boxes are OBBs in order to
        // effectively compute the intersection.
        if let BoundingBox::Oriented(a) = &self {
            if let BoundingBox::Oriented(b) = &other {
                // Compute the intersection.
                //
                // We first check if an intersection exists. If yes, then we
                // return the appropriate bounding box wrapping the region.
                if let Some(region) = a.intersects(b) {
                    return Some(BoundingBox::Oriented(region));
                }
            }

            return None; // exit early
        }

        None
    }
}
