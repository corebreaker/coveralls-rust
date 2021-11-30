use super::LogValueKind;
use regex::Regex;
use git2::{Oid, Repository};
use simple_error::SimpleError;
use std::io::{Result, Error, ErrorKind};

macro_rules! checked {
    ($r:expr) => {match $r {
        Ok(v) => v,
        Err(err) => { return Err(Error::new(ErrorKind::Other, err)); }
    }};
}

struct BranchInfos {
    head_id: Oid,
    branch_name: Option<String>,
}

impl BranchInfos {
    fn from_repo(repo: &Repository) -> Result<Self> {
        let head = checked! { repo.head() };
        let head_id = match head.target() {
            Some(id) => id,
            None => {
                let msg = if head.is_branch() {
                    match head.name() {
                        None => String::from("The head of this repository is a branch that has no ID"),
                        Some(name) => {
                            format!("The head of this repository is a branch that has no ID (name: {})", name)
                        }
                    }
                } else {
                    String::from("The head of this repository is not a branch and has no ID")
                };

                return Err(Error::new(ErrorKind::Other, SimpleError::new(msg)));
            }
        };

        let branch_name = if head.is_branch() {
            head.name().map(String::from)
        } else {
            None
        };

        Ok(Self { head_id, branch_name })
    }
}

pub(in super::super) struct GitFetcher {
    repo: Repository,
    head_id: Oid,
    branch_name: Option<String>,
    branch_re: Regex,
}

impl GitFetcher {
    pub(in super::super) fn new() -> Result<Self> {
        let repo = checked! { Repository::open(".") };
        let infos = BranchInfos::from_repo(&repo)?;
        let branch_re = Regex::new(r"^refs/heads/(.+)$").expect("Bad regex");

        Ok(GitFetcher {
            repo,
            head_id: infos.head_id,
            branch_name: infos.branch_name,
            branch_re,
        })
    }

    pub(in super::super) fn get_branch(&self) -> Result<Option<String>> {
        if let Some(name) = self.branch_name.as_ref() {
            if let Some(caps) = self.branch_re.captures(name) {
                return Ok(Some(caps[1].to_string()));
            }
        }

        Ok(None)
    }

    pub(in super::super) fn get_log(&self, kind: LogValueKind) -> Result<Option<String>> {
        let commit = checked! { self.repo.find_commit(self.head_id) };

        Ok(match kind {
            LogValueKind::Id => Some(commit.id().to_string()),
            LogValueKind::AuthorName => commit.author().name().map(String::from),
            LogValueKind::AuthorEmail => commit.author().email().map(String::from),
            LogValueKind::CommitterName => commit.committer().name().map(String::from),
            LogValueKind::CommitterEmail => commit.committer().email().map(String::from),
            LogValueKind::Message => commit.message().map(String::from),
        })
    }

    pub(in super::super) fn get_remotes(&self) -> Result<Option<Vec<(String, String)>>> {
        let mut res = vec![];
        let remotes = checked! { self.repo.remotes() };

        for name in remotes.iter().filter_map(|x| x) {
            let remote = checked! { self.repo.find_remote(name) };

            if let Some(url) = remote.url() {
                res.push((name.to_string(), url.to_string()));
            }
        }

        if res.is_empty() {
            Ok(None)
        } else {
            Ok(Some(res))
        }
    }
}
