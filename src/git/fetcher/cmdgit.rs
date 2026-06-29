//! `GitFetcher` backend that shells out to the `git` command (default, no `libgit` feature).

use super::LogInfos;
use simple_error::SimpleError;
use log::trace;
use std::{
    io::{Result, Error, ErrorKind},
    process::Command,
    ffi::OsStr,
};

/// Parse one line of `git remote -v` output into a `(name, url)` pair.
///
/// Only the `(fetch)` lines are kept so that each remote is reported once.
fn extract_remote(line: &str) -> Option<(String, String)> {
    if !line.ends_with(" (fetch)") {
        return None;
    }

    let mut parts = line.split_whitespace().map(|s| s.trim());
    let name = parts.next().map(String::from);
    let url = parts.next().map(String::from);

    match (name, url) {
        (Some(name), Some(url)) => Some((name, url)),
        _ => None,
    }
}

/// Collects Git metadata by invoking the `git` executable as a subprocess.
pub(in super::super) struct GitFetcher;

impl GitFetcher {
    /// Create a new fetcher. The `git` binary is only invoked lazily by the other methods.
    pub(in super::super) fn new() -> Result<Self> {
        Ok(GitFetcher)
    }

    /// Run `git` with the given arguments and return its standard output.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if the process cannot be spawned, exits with a non-zero status
    /// or produces non-UTF-8 output.
    fn run_command(&self, args: Vec<&str>) -> Result<String> {
        trace!("Running Git command: git {}", args.join(" "));

        let res = Command::new("git").args(args.into_iter().map(OsStr::new)).output()?;

        if !res.status.success() {
            let out = String::from_utf8_lossy(res.stdout.as_slice());
            let err = String::from_utf8_lossy(res.stderr.as_slice());
            let msg = format!(
                "GIT command return code {}\nSTDOUT: {:?}\nSTDERR: {:?}",
                res.status, out, err
            );

            return Err(Error::new(ErrorKind::Other, SimpleError::new(msg)));
        }

        match String::from_utf8(res.stdout) {
            Ok(v) => Ok(v),
            Err(err) => Err(Error::new(ErrorKind::Other, err)),
        }
    }

    /// Return the current branch name (`git rev-parse --abbrev-ref HEAD`), or `None` in detached
    /// `HEAD` state.
    pub(in super::super) fn get_branch(&self) -> Result<Option<String>> {
        let res = self
            .run_command(vec!["rev-parse", "--abbrev-ref", "HEAD"])?
            .trim()
            .to_string();

        Ok((!res.is_empty()).then_some(res))
    }

    /// Return the information about the `HEAD` commit (`git log -1`).
    pub(in super::super) fn get_log(&self) -> Result<LogInfos> {
        let res = self.run_command(vec![
            "--no-pager",
            "log",
            "-1",
            "--pretty=format:%H%n%aN%n%ae%n%cN%n%ce%n%s",
        ])?;

        let mut fields = res.split('\n').map(|s| {
            let s = s.trim();

            (!s.is_empty()).then(|| s.to_string())
        });

        Ok(LogInfos::new(
            fields.next().flatten(),
            fields.next().flatten(),
            fields.next().flatten(),
            fields.next().flatten(),
            fields.next().flatten(),
            fields.next().flatten(),
        ))
    }

    /// Return the list of `(name, url)` remotes (`git remote -v`), or `None` when there is none.
    pub(in super::super) fn get_remotes(&self) -> Result<Option<Vec<(String, String)>>> {
        let res = self
            .run_command(vec!["remote", "-v"])?
            .trim()
            .lines()
            .filter_map(extract_remote)
            .collect::<Vec<_>>();

        Ok((!res.is_empty()).then_some(res))
    }
}
