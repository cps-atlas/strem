//! Application-specific configurations.
//!

use std::path::PathBuf;

/// Configuration information for Application.
///
/// This information does not capture the subcommands used---just flags, options,
/// and arguments.
pub struct Configuration<'a> {
    /// The SpRE used for searching.
    pub pattern: &'a String,

    /// The data stream to search over. If this is `None`, then it is assumed
    /// the source is standard input.
    pub datastream: Option<&'a PathBuf>,

    /// Use the online algorithm.
    pub online: bool,

    /// A collection of channels to import.
    pub channels: Option<Vec<&'a String>>,

    /// Maximum number of matches to search for.
    pub limit: Option<usize>,

    /// Export the data of a match.
    pub export: bool,

    /// Do not print anything.
    pub quiet: bool,

    /// Ignore the first `skip` amount of frames.
    pub skip: Option<usize>,
}
