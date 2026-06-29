use super::{fetcher::GitFetcher, remote::GitRemote, head::GitHead};
use crate::config::Config;
use log::debug;
use serde::{Serialize, Deserialize};
use std::{io::Result, mem::replace};

/// Git metadata attached to a coverage report.
///
/// Groups the information about the `HEAD` commit (see [`GitHead`]), the current branch and the
/// configured remotes, as expected in the `git` object of the Coveralls JSON format. It is
/// resolved with [`GitInfos::update`], which reads the local repository and then overlays the
/// values coming from the configuration.
#[derive(Default, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct GitInfos {
    pub(crate) head: GitHead,

    #[serde(skip_serializing_if = "String::is_empty")]
    #[serde(default)]
    pub(crate) branch: String,

    #[serde(default)]
    pub(crate) remotes: Vec<GitRemote>,
}

impl GitInfos {
    /// Overlay the Git fields coming from the configuration on top of the current values.
    fn fetch_from_config(&mut self, config: &Config) {
        self.head.fetch_from_config(config);

        if let Some(value) = GitRemote::fetch_from_config(config) {
            if self.remotes.is_empty() {
                self.remotes.push(value);
            } else {
                match self.remotes.iter().position(|remote| remote.name == value.name) {
                    None => {
                        self.remotes.push(value);
                    }
                    Some(idx) => {
                        self.remotes[idx].url = value.url;
                    }
                }
            }
        }

        if let Some(v) = config.git_branch.clone() {
            self.branch = v;
        }
    }

    /// Collect the `HEAD` commit, remotes and branch from the local repository.
    fn fetch_from_git(&mut self) -> Result<()> {
        debug!("Fetching Git information from the local repository");

        let git_fetcher = GitFetcher::new()?;

        self.head.fetch_from_git(&git_fetcher)?;
        self.remotes = GitRemote::fetch_list(&git_fetcher, replace(&mut self.remotes, vec![]))?;

        if let Some(v) = git_fetcher.get_branch()? {
            debug!("Resolved Git branch `{v}`");

            self.branch = v;
        }

        Ok(())
    }

    /// Resolve the Git metadata: read the local repository, then overlay the configuration.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if the repository cannot be read or if the resulting `HEAD`
    /// information is incomplete (see [`GitHead::check`]).
    pub fn update(&mut self, config: &Config) -> Result<()> {
        self.fetch_from_git()?;
        self.fetch_from_config(config);

        self.head.check()
    }
}
