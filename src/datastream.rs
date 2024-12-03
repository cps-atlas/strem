//! The ubiquitous perception stream structure for matching.
//!
//! This is the format from which all importers must import to.

use std::error::Error;
use std::fmt;
use std::io::Read;

use serde_json::de::IoRead;
use serde_json::StreamDeserializer;

use self::frame::Frame;
use self::io::importer::Importer;

pub mod frame;
pub mod io;

/// An interface to interact with perception stream data.
///
/// It should be further noted that this interface provides basic mechanisms to
/// reading/writing of the stream regardless of offline/online application.
pub struct DataStream<'a, R: Read> {
    pub frames: Vec<Frame>,

    /// The source from which data is loaded.
    pub stream: StreamDeserializer<'a, IoRead<R>, io::DataStream>,

    /// A limit on the number of frames to keep in memory.
    pub capacity: Option<usize>,
}

impl<R: Read> DataStream<'_, R> {
    /// Create a new [`DataStream`] with the selected format.
    ///
    /// This function creates an empty [`DataStream`] instance that still must
    /// be further populated with frames.
    pub fn new(source: R) -> Self {
        let stream = StreamDeserializer::new(IoRead::new(source));

        DataStream {
            frames: Vec::new(),
            capacity: None,
            stream,
        }
    }

    /// Set the `capacity` of the [`DataStream`].
    pub fn capacity(&mut self, size: usize) {
        self.capacity = Some(size);
    }

    /// Request the next frame from the [`DataImport`].
    pub fn request(
        &mut self,
        importer: &mut Importer,
    ) -> Result<Option<Vec<Frame>>, Box<dyn Error>> {
        match self.stream.next() {
            Some(data) => importer.import(data?),
            None => Ok(None),
        }
    }

    /// Insert a [`Frame`] at the specified index.
    ///
    /// # Panics
    ///
    /// This function panics if the `index` > `self.frames.len()`.
    pub fn insert(&mut self, index: usize, frame: Frame) {
        self.frames.insert(index, frame);
    }

    /// Append a [`Frame`] at the end of the [`DataStream`].
    ///
    /// This is a shortcut method for [`Self::insert`] where the index is the length of
    /// the [`DataStream`].
    pub fn append(&mut self, frame: Frame) {
        self.insert(self.frames.len(), frame);
    }
}

impl<'a, R: Read> fmt::Debug for DataStream<'a, R> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("DataStream")
            .field("frames", &self.frames)
            .field("capacity", &self.capacity)
            .finish()
    }
}

#[derive(Debug, Clone)]
struct DataStreamError {
    msg: String,
}

impl From<&str> for DataStreamError {
    fn from(msg: &str) -> Self {
        DataStreamError {
            msg: msg.to_string(),
        }
    }
}

impl From<String> for DataStreamError {
    fn from(msg: String) -> Self {
        DataStreamError { msg }
    }
}

impl fmt::Display for DataStreamError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "datastream: {}", self.msg)
    }
}

impl Error for DataStreamError {}
