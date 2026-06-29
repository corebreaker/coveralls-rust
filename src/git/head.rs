use super::fetcher::GitFetcher;
use crate::config::Config;
use serde::{Serialize, Deserialize};
use simple_error::SimpleError;
use const_format::concatcp;
use log::debug;
use std::io::{Result, Error, ErrorKind};

/// Build the error message shown when the `HEAD` information could not be fully collected.
///
/// The hint about a missing `git` binary is only appended when the `libgit` backend is not used.
const fn error_message() -> &'static str {
    const MSG: &str = "\
        Failed collecting git data. \
        Did you use command line arguments ? \
        Are you running coveralls inside a git repository ?\
    ";

    if cfg!(feature = "libgit") {
        MSG
    } else {
        concatcp!(MSG, " Is git installed ?")
    }
}

/// Information about the `HEAD` commit of the repository.
///
/// Holds the commit identifier, the author and committer identities and the commit message, as
/// expected in the `git.head` object of the Coveralls JSON format.
#[derive(Default, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub(crate) struct GitHead {
    #[serde(default)]
    pub(crate) id: String,

    #[serde(default)]
    pub(crate) author_name: String,

    #[serde(default)]
    pub(crate) author_email: String,

    #[serde(default)]
    pub(crate) committer_name: String,

    #[serde(default)]
    pub(crate) committer_email: String,

    #[serde(default)]
    pub(crate) message: String,
}

impl GitHead {
    /// Overlay the `HEAD` fields coming from the configuration on top of the current values.
    pub(super) fn fetch_from_config(&mut self, config: &Config) {
        if let Some(v) = config.git_id.clone() {
            self.id = v;
        }

        if let Some(v) = config.git_author_name.clone() {
            self.author_name = v;
        }

        if let Some(v) = config.git_author_email.clone() {
            self.author_email = v;
        }

        if let Some(v) = config.git_committer_name.clone() {
            self.committer_name = v;
        }

        if let Some(v) = config.git_committer_email.clone() {
            self.committer_email = v;
        }

        if let Some(v) = config.git_message.clone() {
            self.message = v;
        }
    }

    /// Fill the `HEAD` fields from the last commit reported by the fetcher.
    pub(super) fn fetch_from_git(&mut self, git_fetcher: &GitFetcher) -> Result<()> {
        let infos = git_fetcher.get_log()?;

        if let Some(id) = infos.id() {
            debug!("Fetched HEAD commit `{id}`");
        }

        if let Some(v) = infos.id() {
            self.id = v.to_string();
        }

        if let Some(v) = infos.author_name() {
            self.author_name = v.to_string();
        }

        if let Some(v) = infos.author_email() {
            self.author_email = v.to_string();
        }

        if let Some(v) = infos.committer_name() {
            self.committer_name = v.to_string();
        }

        if let Some(v) = infos.committer_email() {
            self.committer_email = v.to_string();
        }

        if let Some(v) = infos.message() {
            self.message = v.to_string();
        }

        Ok(())
    }

    /// Ensure every `HEAD` field has been collected.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] when any field is still empty, with a hint about the likely
    /// cause (not running inside a Git repository, or a missing `git` binary).
    pub(super) fn check(&self) -> Result<()> {
        macro_rules! e {
            ($f:ident) => {
                self.$f.is_empty()
            };
        }

        if e!(id) || e!(author_name) || e!(author_email) || e!(committer_name) || e!(committer_email) || e!(message) {
            debug!("Collected Git HEAD information is incomplete");

            Err(Error::new(ErrorKind::Other, SimpleError::new(error_message())))
        } else {
            Ok(())
        }
    }
}
