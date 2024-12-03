//! STREM application.
//!

use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{stdin, BufReader};
use std::path::PathBuf;

use clap::ArgMatches;
use strem::config::Configuration;
use strem::controller::{Controller, Status};
use strem::datastream::DataStream;

use self::printer::Printer;

mod printer;

pub struct App {
    matches: ArgMatches,
    paths: Option<Vec<PathBuf>>,
}

impl App {
    pub fn new(matches: ArgMatches) -> Self {
        let mut app = Self {
            matches,
            paths: None,
        };

        // Set the positional arguments to search over.
        //
        // This includes the set of files that are manually provided. If not
        // files are provided, standard input ("stdin") is assumed.
        app.paths = app
            .matches
            .get_many("DATASTREAM")
            .map(|p| p.cloned().collect());

        app
    }

    /// Run the strem application.
    ///
    /// This method is responsible for selecting what to run with what
    /// [`Configuration`] based on the arguments, options, and (most importantly)
    /// the subcommand(s).
    pub fn run(&self) -> Result<Status, Box<dyn Error>> {
        // Set the default status for running the [`App`].
        //
        // By default, a match is not found. This should only be changed through
        // running the [`Controller`].
        let mut status = Status::MatchNotFound;

        // Set up the [`Configuration`].
        //
        // The configuration is used to control the behavior of the
        // [`Controller`] as well as the [`Printer`].
        let mut config = self.configure()?;

        // 1. Read from file(s).
        //
        // If a file is supplied, then the input source will be from a file that
        // is loaded, accordingly.
        if let Some(paths) = &self.paths {
            for path in paths {
                config.datastream = Some(path);
                let controller = Controller::new(&config, Some(Printer::print));

                // Run the controller on the [`DataStream`].
                //
                // This creates a new [`DataStream`] with a source from the
                // loaded file, accordingly.
                let f = File::open(path).or(Err(Box::new(AppError::from(format!(
                    "{}: no such file found",
                    path.display()
                )))))?;

                let s = controller.run(DataStream::new(BufReader::new(f)))?;

                // Set the status.
                //
                // This gets set one time when any match is found from running
                // the [`Controller`] on any of the paths.
                if matches!(s, Status::MatchFound) {
                    status = Status::MatchFound;
                }
            }

            return Ok(status);
        };

        // 2. Read from stdin.
        //
        // If no files are provided, then the input source will be from the
        // standard input ("stdin"), accordingly.
        let controller = Controller::new(&config, Some(Printer::print));

        // Run the controller on the [`DataStream`].
        //
        // This creates a new [`DataStream`] with a source from the standard
        // input ("stdin"), accordingly.
        status = controller.run(DataStream::new(BufReader::new(stdin().lock())))?;

        Ok(status)
    }

    /// Create a [`Configuration`] from the CLI arguments.
    fn configure(&self) -> Result<Configuration, Box<dyn Error>> {
        Ok(Configuration {
            pattern: self.matches.get_one("PATTERN").unwrap(),
            datastream: None,
            online: self.matches.get_flag("online"),
            channels: self.matches.get_many("channel").map(|c| c.collect()),
            limit: self.matches.get_one("max-count").copied(),
            export: self.matches.get_flag("export"),
            quiet: self.matches.get_flag("quiet"),
            skip: self.matches.get_one("skip").copied(),
        })
    }
}

#[derive(Debug, Clone)]
struct AppError {
    msg: String,
}

impl From<&str> for AppError {
    fn from(msg: &str) -> Self {
        AppError {
            msg: msg.to_string(),
        }
    }
}

impl From<String> for AppError {
    fn from(msg: String) -> Self {
        AppError { msg }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "app: {}", self.msg)
    }
}

impl Error for AppError {}
