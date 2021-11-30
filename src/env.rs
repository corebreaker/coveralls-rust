use regex::Regex;
use simple_error::SimpleError;
use std::{env::var_os, io::{Result, Error, ErrorKind}};

pub struct Env {
    github_actions_branch_re: Regex,
    github_actions_pull_request_re: Regex,
}

impl Env {
    pub fn new() -> Env {
        Env {
            github_actions_branch_re: Regex::new(r"^refs/(?:heads|tags)/(.+)$").expect("Bad regex"),
            github_actions_pull_request_re: Regex::new(r"^refs/pull/(.+)$").expect("Bad regex"),
        }
    }

    pub fn get_var(&self, name: &str) -> Result<Option<String>> {
        match var_os(name) {
            None => Ok(None),
            Some(s) => match s.into_string() {
                Ok(v) => Ok(if v.is_empty() { None } else { Some(v) }),
                Err(err) => Err(Error::new(ErrorKind::Other, SimpleError::new(format!("{:?}", err)))),
            }
        }
    }

    pub(crate) fn get_github_actions_branch(&self) -> Result<Option<String>> {
        if let Some(github_ref) = self.get_var("GITHUB_REF")? {
            if let Some(captures) = self.github_actions_branch_re.captures(&github_ref) {
                return Ok(Some(captures[1].to_string()))
            }
        }

        self.get_var("GITHUB_HEAD_REF")
    }

    pub(crate) fn get_github_actions_pull_request(&self) -> Result<Option<String>> {
        if let Some(github_ref) = self.get_var("GITHUB_REF")? {
            if let Some(captures) = self.github_actions_pull_request_re.captures(&github_ref) {
                return Ok(Some(captures[1].to_string()))
            }
        }

        Ok(None)
    }
}
