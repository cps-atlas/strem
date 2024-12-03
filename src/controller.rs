//! The matching frameworks controller.
//!
//! This module is responsible for managing and controlling the behavior of the
//! matching framework.

use std::error::Error;
use std::io::Read;

use crate::compiler::Compiler;
use crate::config::Configuration;
use crate::datastream::frame::Frame;
use crate::datastream::io::importer::Importer;
use crate::datastream::DataStream;
use crate::matcher;
use crate::matcher::offline;
use crate::matcher::online;
use crate::matcher::Matching;

type PrintCallback = fn(&[Frame], &Configuration) -> Result<(), Box<dyn Error>>;

#[derive(Debug)]
pub enum Status {
    MatchFound,
    MatchNotFound,
}

/// The main driver to perform matching.
///
/// This includes processing datastreams, monitoring, and matching. The main
/// influence on the controller is from the [`Configuration`] scheme.
pub struct Controller<'a> {
    config: &'a Configuration<'a>,

    /// A callback to use (e.g., printing results).
    callback: Option<PrintCallback>,
}

impl<'a> Controller<'a> {
    /// Create new [`Controller`] with associated [`Configuration`].
    pub fn new(config: &'a Configuration, callback: Option<PrintCallback>) -> Self {
        Self { config, callback }
    }

    /// Entrypoint to execute the [`Controller`].
    ///
    /// The [`DataStream`] only needs to be supplied. This allows the same
    /// [`Controller`] to be reused for differing streams without creating a new
    /// one for each run, accordingly.
    pub fn run<R: Read>(&self, datastream: DataStream<R>) -> Result<Status, Box<dyn Error>> {
        if self.config.online {
            return self.online(datastream);
        }

        self.offline(datastream)
    }

    /// Run the offline matching algorithm.
    pub fn offline<R: Read>(
        &self,
        mut datastream: DataStream<R>,
    ) -> Result<Status, Box<dyn Error>> {
        // Set the initial status to no matches found.
        //
        // This is changed upon the condition that any match is found; else, no
        // match found is used.
        let mut status = Status::MatchNotFound;

        // Compile the SpRE into an S-AST ("Symbolic AST").
        //
        // This also produces the symbolic mapping between uniques characters and
        // spatial formulas.
        let compiler = Compiler::new();
        let ast = compiler.compile(self.config.pattern)?;

        // Build [`offline::Matcher`].
        let matcher = offline::Matcher::from(&ast);

        // Load all [`Frame`](s) into the [`DataStream`].
        //
        // For offline, we want to search over the entire data stream, so all
        // frames are loaded into memory, and none are discarded. This differs to
        // online where it is possible that only some frames are loaded and is
        // done so incrementally.
        let mut importer = Importer::new(self.config);

        while let Some(frames) = datastream.request(&mut importer)? {
            for frame in frames {
                datastream.append(frame);
            }
        }

        // A counter for the number of [`Match`].
        //
        // Ideally, this variable should be stored at a higher level as it is
        // more appropriate to the [`Configuration`]. However, to reduce clutter
        // in the [`Configuration`] struct, it is declared here.
        let mut count = 0;

        let mut offset = 0;
        while offset < datastream.frames.len() {
            if let Some(m) = matcher.leftmost(&datastream.frames[offset..])? {
                // Set status to [`Status::MatchFound`].
                //
                // A match has been found, so the status can be set. This is only
                // set a single time.
                if matches!(status, Status::MatchNotFound) {
                    status = Status::MatchFound;
                }

                // Increment `count` and check for limit.
                //
                // This is done before display the [`Match`] as a `limit` of 0
                // may be requested.
                count += 1;

                if let Some(limit) = self.config.limit {
                    if count > limit {
                        break;
                    }
                }

                // Handle [`Match`].
                if let Some(callback) = self.callback {
                    callback(
                        &datastream.frames[(offset + m.start)..(offset + m.end)],
                        self.config,
                    )?;
                }

                offset += m.end;
                continue;
            }

            offset += 1;
        }

        Ok(status)
    }

    /// Run the online matching algorithm.
    pub fn online<R: Read>(&self, mut datastream: DataStream<R>) -> Result<Status, Box<dyn Error>> {
        // Set the initial status to no matches found.
        //
        // This is changed upon the condition that any match is found; else, no
        // match found is used.
        let mut status = Status::MatchNotFound;

        // Compile the SpRE into an S-AST ("Symbolic AST").
        //
        // This also produces the symbolic mapping between uniques characters and
        // spatial formulas.
        let compiler = Compiler::new();
        let ast = compiler.compile(self.config.pattern)?;

        // Compute the horizon.
        //
        // The horizon places a limit on the number of [`Frame`] that are loaded
        // into the [`DataStream`].
        if let Some(size) = matcher::horizon(&ast) {
            datastream.capacity(size);
        }

        // Build [`online::Matcher`].
        let matcher = online::Matcher::from(&ast);

        // A counter for the number of [`Match`].
        //
        // Ideally, this variable should be stored at a higher level as it is
        // more appropriate to the [`Configuration`]. However, to reduce clutter
        // in the [`Configuration`] struct, it is declared here.
        let mut count = 0;

        // Load all [`Frame`](s) into the [`DataStream`].
        //
        // For online, we want to search over the data stream incrementally, so
        // the algorithm is run for each new [`Frame`] imported. This differs to
        // offline where all [`Frame`](s) must be loadecd before running the
        // algorithm.
        let mut importer = Importer::new(self.config);

        while let Some(frames) = datastream.request(&mut importer)? {
            for frame in frames {
                if let Some(capacity) = datastream.capacity {
                    if datastream.frames.len() >= capacity {
                        // Remove the least recent [`Frame`] from the [`DataStream`].
                        //
                        // This procedure can be thought of as a LRU cache.
                        //
                        // OPTIMIZATION: The use of `remove` shifts all elements to
                        // the right one index to the left. Therefore, it may be
                        // worthwhile to find a better operation to remove the LRU
                        // element (e.g., use a reversed vector with `pop`).
                        datastream.frames.remove(0);
                    }
                }

                datastream.append(frame);

                if let Some(m) = matcher.leftmost(&datastream.frames[..])? {
                    // Set status to [`Status::MatchFound`].
                    //
                    // A match has been found, so the status can be set. This is only
                    // set a single time.
                    if matches!(status, Status::MatchNotFound) {
                        status = Status::MatchFound;
                    }

                    // Increment `count` and check for limit.
                    //
                    // This is done before display the [`Match`] as a `limit` of 0
                    // may be requested.
                    count += 1;

                    if let Some(limit) = self.config.limit {
                        if count > limit {
                            break;
                        }
                    }

                    // Handle [`Match`].
                    if let Some(callback) = self.callback {
                        callback(&datastream.frames[m.start..m.end], self.config)?;
                    }
                }
            }
        }

        Ok(status)
    }
}
