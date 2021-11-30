use super::{super::Config, fetcher::GitFetcher, remote::GitRemote, head::GitHead};
use serde::{Serialize, Deserialize};
use std::{io::Result, mem::replace};

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
    fn fetch_from_config(&mut self, config: &Config) {
        self.head.fetch_from_config(config);

        if let Some(value) = GitRemote::fetch_from_config(config) {
            if self.remotes.is_empty() {
                self.remotes.push(value);
            } else {
                match self.remotes.iter().position(|remote| remote.name == value.name) {
                    None => { self.remotes.push(value); }
                    Some(idx) => { self.remotes[idx].url = value.url; }
                }
            }
        }

        if let Some(v) = config.git_branch.clone() {
            self.branch = v;
        }
    }

    fn fetch_from_git(&mut self) -> Result<()> {
        let git_fetcher = GitFetcher::new()?;

        self.head.fetch_from_git(&git_fetcher)?;
        self.remotes = GitRemote::fetch_list(&git_fetcher, replace(&mut self.remotes, vec![]))?;

        if let Some(v) = git_fetcher.get_branch()? {
            self.branch = v;
        }

        Ok(())
    }

    pub fn update(&mut self, config: &Config) -> Result<()> {
        self.fetch_from_git()?;
        self.fetch_from_config(config);

        self.head.check()
    }
}
