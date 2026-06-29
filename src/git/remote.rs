use super::fetcher::GitFetcher;
use crate::config::Config;
use log::debug;
use serde::{Serialize, Deserialize};
use std::{io::Result, collections::HashMap};

/// Map of remote name to remote URL, used to merge the configured and fetched remotes.
type Dict = HashMap<String, String>;

/// A Git remote, identified by its name and URL.
#[derive(Default, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub(crate) struct GitRemote {
    pub(crate) name: String,

    #[serde(default)]
    pub(crate) url: String,
}

impl GitRemote {
    /// Build a remote from a `(name, url)` pair.
    fn new((name, url): (String, String)) -> Self {
        Self {
            name,
            url,
        }
    }

    /// Build a remote and remove its name from `dict`, marking it as already handled.
    fn with_dict(name: String, url: String, dict: &mut Dict) -> Self {
        dict.remove(&name);

        Self::new((name, url))
    }

    /// Merge the remotes reported by the fetcher with the previously known `old` remotes.
    ///
    /// Remotes returned by the fetcher take precedence; any `old` remote whose name was not seen is
    /// appended so that no previously known remote is lost.
    pub(super) fn fetch_list(git_fetcher: &GitFetcher, old: Vec<GitRemote>) -> Result<Vec<GitRemote>> {
        let mut dict = old.into_iter().map(|entry| (entry.name, entry.url)).collect::<Dict>();
        let mut res = match git_fetcher.get_remotes()? {
            None => vec![],
            Some(list) => list
                .into_iter()
                .map(|(name, url)| GitRemote::with_dict(name, url, &mut dict))
                .collect::<Vec<_>>(),
        };

        res.extend(dict.into_iter().map(GitRemote::new));

        debug!("Collected {} Git remote(s)", res.len());

        Ok(res)
    }

    /// Build a remote from the configuration when both its name and URL are set.
    pub(super) fn fetch_from_config(config: &Config) -> Option<Self> {
        if let Some(name) = config.git_message.clone() {
            if let Some(url) = config.git_remote_url.clone() {
                return Some(Self::new((name, url)));
            }
        }

        None
    }
}
