//! Application printer.
//!

use std::error::Error;
use std::fmt;

use colored::*;
use strem::config::Configuration;
use strem::datastream::frame::Frame;
use strem::datastream::io::exporter::DataExporter;

pub struct Printer {}

impl Printer {
    /// Print a [`Match`].
    pub fn print(frames: &[Frame], config: &Configuration) -> Result<(), Box<dyn Error>> {
        if config.quiet {
            return Ok(());
        }

        let mut msg = String::new();

        if let Some(path) = config.datastream {
            let prefix = path.display().to_string();

            // Print the prefix.
            //
            // This also includes coloring the text appropriately.
            msg = Self::delimit(msg);
            msg = format!("{}{}", msg, prefix.magenta());
        }

        // Print the bounds of the [`Frame`] set.
        //
        // If true, the boundary [start, end) will be printed to the
        // terminal, accordingly.
        let start = frames.first().unwrap().index;
        let end = frames.last().unwrap().index + 1;

        // Print the interval of the match.
        //
        // This also includes coloring the text appropriately.
        msg = Self::delimit(msg);
        msg = format!("{}{}", msg, format!("{}..{}", start, end).green());

        if config.export {
            let s = serde_json::to_string(&DataExporter::new().export(frames)?)?;

            // Print the exported data.
            //
            // This also includes coloring the text appropriately.
            msg.clear();
            msg = Self::delimit(msg);
            msg = format!("{}{}", msg, s.red());
        }

        // Print a the message, accordingly.
        if !msg.is_empty() {
            println!("{}", msg);
        }

        Ok(())
    }

    fn delimit(msg: String) -> String {
        // If the [`msg`] is not empty, then add delimeter.
        //
        // This is used for visual clarity as well as improving the parse-ability
        // of the resulting outputs for post-processing.
        if !msg.is_empty() {
            return format!("{}{}", msg, ":".cyan());
        }

        msg
    }
}

#[derive(Debug, Clone)]
struct PrinterError {
    msg: String,
}

impl From<&str> for PrinterError {
    fn from(msg: &str) -> Self {
        PrinterError {
            msg: msg.to_string(),
        }
    }
}

impl From<String> for PrinterError {
    fn from(msg: String) -> Self {
        PrinterError { msg }
    }
}

impl fmt::Display for PrinterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "printer: {}", self.msg)
    }
}

impl Error for PrinterError {}
