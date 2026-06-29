//! `GitFetcher` backend that reads the repository in-process through [`git2`] (the `libgit`
//! feature).

use super::LogInfos;
use regex::Regex;
use git2::{Oid, Repository, Error as GitError};
use simple_error::SimpleError;
use log::trace;
use std::{
    io::{Result, Error, ErrorKind},
    result::Result as StdResult,
};

/// Convert a [`git2`] result into an [`std::io::Error`], returning early on failure.
macro_rules! checked {
    ($r:expr) => {
        match $r {
            Ok(v) => v,
            Err(err) => {
                return Err(Error::new(ErrorKind::Other, err));
            }
        }
    };
}

/// The `HEAD` object id and, when `HEAD` points at a branch, its reference name.
struct BranchInfos {
    head_id:     Oid,
    branch_name: Option<String>,
}

impl BranchInfos {
    /// Resolve the `HEAD` of `repo` into its object id and optional branch reference name.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if `HEAD` cannot be resolved or has no target object.
    fn from_repo(repo: &Repository) -> Result<Self> {
        let head = checked! { repo.head() };
        let head_id = match head.target() {
            Some(id) => id,
            None => {
                let msg = if head.is_branch() {
                    match head.name() {
                        Err(err) => format!("The head of this repository is a branch that has no ID: {err}"),
                        Ok(name) => format!("The head of this repository is a branch that has no ID (name: {name})"),
                    }
                } else {
                    String::from("The head of this repository is not a branch and has no ID")
                };

                return Err(Error::new(ErrorKind::Other, SimpleError::new(msg)));
            }
        };

        let branch_name = if head.is_branch() {
            Some(checked! { head.name().map(String::from) })
        } else {
            None
        };

        Ok(Self {
            head_id,
            branch_name,
        })
    }
}

/// Collects Git metadata by reading the repository in-process with [`git2`].
pub(in super::super) struct GitFetcher {
    repo:        Repository,
    head_id:     Oid,
    branch_name: Option<String>,
    branch_re:   Regex,
}

impl GitFetcher {
    /// Open the repository in the current directory and resolve its `HEAD`.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if the current directory is not a Git repository or if `HEAD`
    /// cannot be resolved.
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

    /// Return the current branch name, stripped of its `refs/heads/` prefix, or `None` in detached
    /// `HEAD` state.
    pub(in super::super) fn get_branch(&self) -> Result<Option<String>> {
        if let Some(name) = self.branch_name.as_ref() {
            if let Some(caps) = self.branch_re.captures(name) {
                return Ok(Some(caps[1].to_string()));
            }
        }

        Ok(None)
    }

    /// Return the information about the `HEAD` commit.
    pub(in super::super) fn get_log(&self) -> Result<LogInfos> {
        trace!("Reading HEAD commit {} via libgit2", self.head_id);

        let commit = checked! { self.repo.find_commit(self.head_id) };
        let author = commit.author();
        let committer = commit.committer();

        Ok(LogInfos::new(
            Some(commit.id().to_string()),
            checked! { author.name().map(map_string) },
            checked! { author.email().map(map_string) },
            checked! { committer.name().map(map_string) },
            checked! { committer.email().map(map_string) },
            checked! { commit.message().map(map_string) },
        ))
    }

    /// Return the list of `(name, url)` remotes of the repository, or `None` when there is none.
    pub(in super::super) fn get_remotes(&self) -> Result<Option<Vec<(String, String)>>> {
        let mut res = vec![];
        let remotes = checked! { self.repo.remotes() };

        for name in remotes.iter().filter_map(map_remote_names) {
            let name = checked! { name };
            let remote = checked! { self.repo.find_remote(name) };
            let url = checked! { remote.url() };

            res.push((name.to_string(), url.to_string()));
        }

        Ok((!res.is_empty()).then_some(res))
    }
}

/// Drop unnamed remotes (`Ok(None)`) while keeping named ones and errors, for use in a
/// `filter_map`.
fn map_remote_names(remote: StdResult<Option<&str>, GitError>) -> Option<StdResult<&str, GitError>> {
    match remote {
        Ok(Some(name)) => Some(Ok(name)),
        Ok(None) => None,
        Err(err) => Some(Err(err)),
    }
}

/// Turn a borrowed string into an owned one, mapping the empty string to `None`.
fn map_string(s: &str) -> Option<String> {
    (!s.is_empty()).then(|| String::from(s))
}
