use std::error::Error;
use std::fmt;
use std::path::PathBuf;

use crate::config::Configuration;
use crate::datastream::frame::sample::detections::bbox::region::aa;
use crate::datastream::frame::sample::detections::bbox::region::oriented;
use crate::datastream::frame::sample::detections::bbox::region::Point;
use crate::datastream::frame::sample::detections::bbox::BoundingBox;
use crate::datastream::frame::sample::detections::{
    Annotation, DetectionRecord, Image, ImageSource,
};
use crate::datastream::frame::sample::Sample;
use crate::datastream::frame::Frame;

use super::super::io;

/// A reader for importing STREM-formatted data.
pub struct Importer<'a> {
    config: &'a Configuration<'a>,
    count: usize,
}

impl<'a> Importer<'a> {
    /// Create a new [`Importer`].
    pub fn new(config: &'a Configuration<'a>) -> Self {
        Importer { config, count: 0 }
    }

    /// From the [`io::DataStrema`], import a series of [`Frame`].
    ///
    /// This accepts a single deserialized [`io::DataStream`] and transforms it
    /// into a set of [`Frame`].
    pub fn import(&mut self, data: io::DataStream) -> Result<Option<Vec<Frame>>, Box<dyn Error>> {
        if data.version != env!("CARGO_PKG_VERSION") {
            return Err(Box::new(ImporterError::from(format!(
                "mismatched version... expected v{}",
                env!("CARGO_PKG_VERSION")
            ))));
        }

        let mut frames = Vec::new();

        for f in data.frames.iter() {
            let mut frame = Frame::new(f.index);

            // Skip this [`f`] if skip count not reached.
            //
            // If a skip limit exists, then compare the skip limit against the
            // total number of frames imported. If the number of imported frames
            // is less than the skip limit, then skip this frame.
            if let Some(skip) = self.config.skip {
                if self.count < skip {
                    // Increment the count.
                    //
                    // We perform this here to avoid unnecessary counting. For
                    // example, in the online case, this counting may be
                    // infinite; so this avoid the issue of overflow.
                    self.count += 1;
                    continue;
                }
            }

            for s in f.samples.iter() {
                let sample = match s {
                    io::Sample::ObjectDetection {
                        channel,
                        image,
                        annotations,
                    } => {
                        if let Some(channels) = &self.config.channels {
                            if !channels.contains(&channel) {
                                // The channel from the data is not in the
                                // specified channels. Therefore, we skip it.
                                continue;
                            }
                        }

                        let mut record = DetectionRecord::new(
                            channel.clone(),
                            Some(Image::new(
                                ImageSource::File(PathBuf::from(&image.path)),
                                image.dimensions.width,
                                image.dimensions.height,
                            )),
                        );

                        // Add annotations to the [`DetectionRecord`].
                        for a in annotations.iter() {
                            // Create the relevant [`BoundingBox`].
                            //
                            // The variant depends on the kind of bounding box
                            // parsed from the data.
                            let bbox = match &a.bbox {
                                io::BoundingBox::AxisAligned { region } => {
                                    BoundingBox::AxisAligned(aa::Region::new(
                                        Point::new(region.center.x, region.center.y),
                                        region.dimensions.w,
                                        region.dimensions.h,
                                    ))
                                }
                                io::BoundingBox::Oriented { region } => {
                                    BoundingBox::Oriented(oriented::Region::new(
                                        Point::new(region.center.x, region.center.y),
                                        region.dimensions.w,
                                        region.dimensions.h,
                                        region.rotation,
                                    ))
                                }
                            };

                            record
                                .annotations
                                .entry(a.class.clone())
                                .or_default()
                                .push(Annotation::new(a.class.clone(), a.score, bbox));
                        }

                        Sample::ObjectDetection(record)
                    }
                };

                frame.samples.push(sample);
            }

            frames.push(frame);
        }

        Ok(Some(frames))
    }
}

#[derive(Debug, Clone)]
struct ImporterError {
    msg: String,
}

impl From<&str> for ImporterError {
    fn from(msg: &str) -> Self {
        ImporterError {
            msg: msg.to_string(),
        }
    }
}

impl From<String> for ImporterError {
    fn from(msg: String) -> Self {
        ImporterError { msg }
    }
}

impl fmt::Display for ImporterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "importer: stremf: {}", self.msg)
    }
}

impl Error for ImporterError {}
