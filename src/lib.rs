//! Send Rust code coverage reports to [Coveralls](https://coveralls.io).
//!
//! `coveralls` is primarily a command line tool, but it is published as a library crate as well so
//! its building blocks can be reused programmatically. It reads a coverage report in the Coveralls
//! JSON format (such as the one produced by [`grcov`](https://github.com/mozilla/grcov)), enriches
//! it with the metadata expected by the Coveralls API (CI service identifiers, Git information,
//! ...) and uploads the resulting job to <https://coveralls.io>.
//!
//! # Why another Coveralls client?
//!
//! Unlike [`coveralls-python`](https://github.com/TheKevJames/coveralls-python), which only accepts
//! the `lcov` format, this crate focuses on Rust projects and takes the Coveralls JSON format as
//! input. It can also prune dependencies and other unwanted source files from the report, either
//! all absolute paths or specific directories, so that the coverage published online only reflects
//! the project itself.
//!
//! # Command line usage
//!
//! Install the tool with Cargo:
//!
//! ```shell
//! cargo install coveralls
//! ```
//!
//! The coverage report is read from the standard input (or from a file passed as an argument) and a
//! subcommand selects the CI service that produced the build:
//!
//! ```shell
//! # Read the report from stdin and let the `circleci` subcommand pick up the
//! # relevant `CIRCLE_*` environment variables.
//! grcov ... --output-type coveralls | coveralls circleci
//!
//! # Read the report from a file and guess the service from the environment.
//! coveralls coverage.json env
//! ```
//!
//! Run `coveralls --help`, or `coveralls <service> --help`, for the list of accepted command line
//! arguments and environment variables. Command line arguments always take precedence over the
//! values read from the environment.
//!
//! ## Supported CI services
//!
//! The CI service can either be selected explicitly with a subcommand or guessed from the
//! environment with the `env` subcommand. The following services are recognized (see [`Service`]):
//!
//! - AppVeyor
//! - BuildKite
//! - Circle-CI
//! - GitHub Actions
//! - Jenkins
//! - Semaphore
//! - Travis
//!
//! # Cargo features
//!
//! Git metadata about the `HEAD` commit (author, committer, message, branch, remotes) is collected
//! from the local repository when it is missing from the report or when it is explicitly requested
//! with `--force-fetch-git-infos`. Two backends are available:
//!
//! - **default**: the `git` executable is invoked as a subprocess, so `git` must be available in the `PATH`.
//! - **`libgit`**: the repository is read in-process through [`git2`](https://docs.rs/git2), which removes the
//!   dependency on an external `git` binary.
//!
//! # Library usage
//!
//! The whole command line program is exposed through the single [`work`] entry point, which mirrors
//! the behaviour of the `coveralls` binary:
//!
//! ```rust,no_run
//! fn main() {
//!     if let Err(err) = coveralls::work() {
//!         eprintln!("{err}");
//!         std::process::exit(1);
//!     }
//! }
//! ```
//!
//! For finer grained control, the individual stages are available as well: build a [`Config`] from
//! the command line or the environment, parse a [`Coverage`] report with [`Coverage::from_reader`],
//! then let a [`CoverallsManager`] enrich and upload it. [`Env`] and [`Service`] are the supporting
//! types used to read environment variables and identify the CI service.
#![warn(missing_docs)]

mod api;
mod cli_args;
mod config;
mod coverage;
mod coveralls;
mod env;
mod git;
mod helpers;
mod service;
mod work;

pub use self::{env::Env, coverage::Coverage, coveralls::CoverallsManager, config::Config, service::Service};

pub use work::work;
