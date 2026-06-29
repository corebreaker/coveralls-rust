//! Binary entry point of the `coveralls` command line tool.
//!
//! It initializes the logger and delegates the whole work to [`coveralls::work`], exiting with a
//! non-zero status code when it fails.

use coveralls::work;
use log::error;
use std::process::exit;

/// Initialize logging and run the [`coveralls::work`] workflow, exiting with `1` on error.
fn main() {
    env_logger::init();

    if let Err(err) = work() {
        error!("{err}");
        exit(1);
    }
}
