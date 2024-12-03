use serde::{Deserialize, Serialize};

pub mod exporter;
pub mod importer;

#[derive(Debug, Deserialize, Serialize)]
pub struct DataStream {
    version: String,
    frames: Vec<Frame>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Frame {
    index: usize,
    samples: Vec<Sample>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum Sample {
    #[serde(rename = "@stremf/sample/detection")]
    ObjectDetection {
        channel: String,
        image: Image,
        annotations: Vec<Annotation>,
    },
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Image {
    path: String,
    dimensions: ImageDimensions,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ImageDimensions {
    width: u32,
    height: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Annotation {
    class: String,
    score: f64,
    bbox: BoundingBox,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum BoundingBox {
    #[serde(rename = "@stremf/bbox/aabb")]
    AxisAligned { region: AxisAlignedRegion },

    #[serde(rename = "@stremf/bbox/obb")]
    Oriented { region: OrientedRegion },
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AxisAlignedRegion {
    center: AxisAlignedRegionCenter,
    dimensions: AxisAlignedRegionDimensions,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AxisAlignedRegionCenter {
    x: f64,
    y: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AxisAlignedRegionDimensions {
    w: f64,
    h: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OrientedRegion {
    center: OrientedRegionCenterPoint,
    dimensions: OrientedRegionDimensions,
    rotation: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OrientedRegionCenterPoint {
    x: f64,
    y: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OrientedRegionDimensions {
    w: f64,
    h: f64,
}
