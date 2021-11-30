use super::fetcher::{GitFetcher, LogValueKind};
use crate::Config;
use serde::{Serialize, Deserialize};
use simple_error::SimpleError;
use const_format::concatcp;
use std::io::{Result, Error, ErrorKind};

const fn error_message() -> &'static str {
    const MSG: &str = "\
        Failed collecting git data. \
        Did you use command line arguments ? \
        Are you running coveralls inside a git repository ?\
    ";

    if cfg!(feature = "use-libgit") {
        MSG
    } else {
        concatcp!(MSG, " Is git installed ?")
    }
}

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

    pub(super) fn fetch_from_git(&mut self, git_fetcher: &GitFetcher) -> Result<()> {
        if let Some(v) = git_fetcher.get_log(LogValueKind::Id)? {
            self.id = v;
        }

        if let Some(v) = git_fetcher.get_log(LogValueKind::AuthorName)? {
            self.author_name = v;
        }

        if let Some(v) = git_fetcher.get_log(LogValueKind::AuthorEmail)? {
            self.author_email = v;
        }

        if let Some(v) = git_fetcher.get_log(LogValueKind::CommitterName)? {
            self.committer_name = v;
        }

        if let Some(v) = git_fetcher.get_log(LogValueKind::CommitterEmail)? {
            self.committer_email = v;
        }

        if let Some(v) = git_fetcher.get_log(LogValueKind::Message)? {
            self.message = v;
        }

        Ok(())
    }

    pub(super) fn check(&self) -> Result<()> {
        macro_rules! e {
            ($f:ident) => {self.$f.is_empty()};
        }

        if e!(id) || e!(author_name) || e!(author_email) || e!(committer_name) || e!(committer_email) || e!(message) {
            Err(Error::new(ErrorKind::Other, SimpleError::new(error_message())))
        } else {
            Ok(())
        }
    }
}

