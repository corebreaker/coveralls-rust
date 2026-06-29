use regex::Regex;
use simple_error::SimpleError;
use log::trace;
use std::{
    env::var_os,
    io::{Result, Error, ErrorKind},
};

/// Accessor for the environment variables of the current process.
///
/// `Env` reads process environment variables and offers a couple of helpers to extract the values
/// that need parsing (such as the GitHub Actions `GITHUB_REF`). Pre-compiled regular expressions
/// used by those helpers are cached in the struct, so an `Env` is meant to be created once with
/// [`Env::new`] and reused.
///
/// Empty variables are treated as if they were unset, and values are never logged so that secrets
/// like `COVERALLS_REPO_TOKEN` are not leaked.
pub struct Env {
    github_actions_branch_re:       Regex,
    github_actions_pull_request_re: Regex,
}

impl Env {
    /// Build a new `Env`, compiling the regular expressions used by the helper methods.
    pub fn new() -> Env {
        Env {
            github_actions_branch_re:       Regex::new(r"^refs/(?:heads|tags)/(.+)$").expect("Bad regex"),
            github_actions_pull_request_re: Regex::new(r"^refs/pull/(.+)$").expect("Bad regex"),
        }
    }

    /// Read the environment variable `name`.
    ///
    /// Returns `Ok(None)` when the variable is unset *or* set to an empty string. Only the presence
    /// of the variable is traced, never its value.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if the variable holds non-Unicode data.
    pub fn get_var(&self, name: &str) -> Result<Option<String>> {
        let value = match var_os(name) {
            None => None,
            Some(s) => match s.into_string() {
                Ok(v) => (!v.is_empty()).then_some(v),
                Err(err) => {
                    return Err(Error::new(ErrorKind::Other, SimpleError::new(format!("{:?}", err))));
                }
            },
        };

        // Note: only the presence is traced, never the value (e.g. COVERALLS_REPO_TOKEN is a secret).
        trace!(
            "Environment variable `{name}` is {}",
            if value.is_some() { "set" } else { "unset" }
        );

        Ok(value)
    }

    /// Extract the branch (or tag) name from the GitHub Actions environment.
    ///
    /// The name is parsed out of `GITHUB_REF` (`refs/heads/<name>` or `refs/tags/<name>`); when it
    /// does not match, `GITHUB_HEAD_REF` is used as a fallback, which is set for pull requests.
    pub(crate) fn get_github_actions_branch(&self) -> Result<Option<String>> {
        if let Some(github_ref) = self.get_var("GITHUB_REF")? {
            if let Some(captures) = self.github_actions_branch_re.captures(&github_ref) {
                return Ok(Some(captures[1].to_string()));
            }
        }

        self.get_var("GITHUB_HEAD_REF")
    }

    /// Extract the pull request number from the GitHub Actions environment.
    ///
    /// The number is parsed out of `GITHUB_REF` when it has the `refs/pull/<number>/...` shape;
    /// returns `Ok(None)` otherwise.
    pub(crate) fn get_github_actions_pull_request(&self) -> Result<Option<String>> {
        if let Some(github_ref) = self.get_var("GITHUB_REF")? {
            if let Some(captures) = self.github_actions_pull_request_re.captures(&github_ref) {
                return Ok(Some(captures[1].to_string()));
            }
        }

        Ok(None)
    }
}
