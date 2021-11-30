use super::fetcher::GitFetcher;
use crate::Config;
use serde::{Serialize, Deserialize};
use std::{io::Result, collections::HashMap};

type Dict = HashMap<String, String>;

#[derive(Default, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub(crate) struct GitRemote {
    pub(crate) name: String,

    #[serde(default)]
    pub(crate) url: String,
}

impl GitRemote {
    fn new((name, url): (String, String)) -> Self {
        Self { name, url }
    }

    fn with_dict(name: String, url: String, dict: &mut Dict) -> Self {
        dict.remove(&name);

        Self::new((name, url))
    }

    pub(super) fn fetch_list(git_fetcher: &GitFetcher, old: Vec<GitRemote>) -> Result<Vec<GitRemote>> {
        let mut dict = old.into_iter().map(|entry| (entry.name, entry.url)).collect::<Dict>();
        let mut res = match git_fetcher.get_remotes()? {
            None => vec![],
            Some(list) => list.into_iter()
                .map(|(name, url)| GitRemote::with_dict(name, url, &mut dict))
                .collect::<Vec<_>>(),
        };

        res.extend(dict.into_iter().map(GitRemote::new));

        Ok(res)
    }

    pub(super) fn fetch_from_config(config: &Config) -> Option<Self> {
        if let Some(name) = config.git_message.clone() {
            if let Some(url) = config.git_remote_url.clone() {
                return Some(Self::new((name, url)));
            }
        }

        None
    }
}
