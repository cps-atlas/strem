use std::collections::HashMap;
use std::path::PathBuf;

use self::bbox::BoundingBox;

pub mod bbox;

/// A sample record of object detections produced for a single frame.
///
/// This includes the labels and regions associated with such. Furthermore,
/// additional data related to the detection record is populated here.
#[derive(Clone, Debug)]
pub struct DetectionRecord {
    pub channel: String,
    pub image: Option<Image>,

    /// A mapping between labels and annotations (i.e., bounding boxes).
    pub annotations: HashMap<String, Vec<Annotation>>,
}

impl DetectionRecord {
    /// Create a new [`DetectionRecord`].
    pub fn new(channel: String, image: Option<Image>) -> Self {
        DetectionRecord {
            channel,
            image,
            annotations: HashMap::new(),
        }
    }
}

/// An annotation of a label generated from a DNN.
///
/// This fundamentally includes the label, the region, and the confidence
/// ("score") of the resulting detection.
#[derive(Clone, Debug)]
pub struct Annotation {
    pub label: String,
    pub score: f64,
    pub bbox: BoundingBox,
}

impl Annotation {
    /// Create a new [`Annotation`] with associated data.
    pub fn new(label: String, score: f64, bbox: BoundingBox) -> Self {
        Annotation { label, score, bbox }
    }
}

/// An interface to handle image metadata.
///
/// This includes source, dimensions, and any additional data that would be
/// associated with an image at this level.
#[derive(Clone, Debug)]
pub struct Image {
    pub source: ImageSource,
    pub width: u32,
    pub height: u32,
}

impl Image {
    /// Create a new [`Image`].
    pub fn new(source: ImageSource, width: u32, height: u32) -> Self {
        Image {
            source,
            width,
            height,
        }
    }
}

/// A interface to collect the image.
///
/// The image can be sourced from a file path, url, etc.
#[derive(Clone, Debug)]
pub enum ImageSource {
    File(PathBuf),
}
