use std::error::Error;
use std::fmt;

use crate::datastream::frame::sample::detections::bbox::BoundingBox;
use crate::datastream::frame::sample::detections::ImageSource;
use crate::datastream::frame::sample::Sample;
use crate::datastream::frame::Frame;
use crate::datastream::io;

#[derive(Default)]
pub struct DataExporter {}

impl DataExporter {
    /// Create a new [`DataExporter`].
    pub fn new() -> Self {
        DataExporter {}
    }

    /// From a series of [`Frame`], convert to an [`io::DataStream`].
    ///
    /// This accepts a set of [`Frame`] and transforms it into a single
    /// serializable [`io::DataStream`].
    pub fn export(&self, frames: &[Frame]) -> Result<io::DataStream, Box<dyn Error>> {
        let mut datastream = io::DataStream {
            version: String::from(env!("CARGO_PKG_VERSION")),
            frames: Vec::new(),
        };

        for f in frames.iter() {
            let mut samples = Vec::new();

            for s in f.samples.iter() {
                match s {
                    Sample::ObjectDetection(record) => {
                        let i = record.image.as_ref().map(|i| io::Image {
                            path: match &i.source {
                                ImageSource::File(path) => String::from(path.to_str().unwrap()),
                            },
                            dimensions: io::ImageDimensions {
                                width: i.width,
                                height: i.height,
                            },
                        });

                        let mut a = Vec::new();
                        for annotations in record.annotations.values() {
                            for annotation in annotations.iter() {
                                let bbox = match &annotation.bbox {
                                    BoundingBox::AxisAligned(region) => {
                                        io::BoundingBox::AxisAligned {
                                            region: io::AxisAlignedRegion {
                                                center: io::AxisAlignedRegionCenter {
                                                    x: region.center().x,
                                                    y: region.center().y,
                                                },
                                                dimensions: io::AxisAlignedRegionDimensions {
                                                    w: region.width(),
                                                    h: region.height(),
                                                },
                                            },
                                        }
                                    }
                                    BoundingBox::Oriented(region) => io::BoundingBox::Oriented {
                                        region: io::OrientedRegion {
                                            center: io::OrientedRegionCenterPoint {
                                                x: region.center().x,
                                                y: region.center().y,
                                            },
                                            dimensions: io::OrientedRegionDimensions {
                                                w: region.width(),
                                                h: region.height(),
                                            },
                                            rotation: region.rotation(),
                                        },
                                    },
                                };

                                a.push(io::Annotation {
                                    class: annotation.label.clone(),
                                    score: annotation.score,
                                    bbox,
                                })
                            }
                        }

                        samples.push(io::Sample::ObjectDetection {
                            channel: record.channel.clone(),
                            image: i.unwrap(),
                            annotations: a,
                        })
                    }
                }
            }

            datastream.frames.push(io::Frame {
                index: f.index,
                samples,
            });
        }

        Ok(datastream)
    }
}

#[derive(Debug, Clone)]
struct DataExporterError {
    msg: String,
}

impl From<&str> for DataExporterError {
    fn from(msg: &str) -> Self {
        DataExporterError {
            msg: msg.to_string(),
        }
    }
}

impl From<String> for DataExporterError {
    fn from(msg: String) -> Self {
        DataExporterError { msg }
    }
}

impl fmt::Display for DataExporterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "exporter: stremf: {}", self.msg)
    }
}

impl Error for DataExporterError {}
