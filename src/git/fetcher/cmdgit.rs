use super::LogValueKind;
use simple_error::SimpleError;
use std::{io::{Result, Error, ErrorKind}, process::Command, ffi::OsStr};

fn extract_remote(line: &str) -> Option<(String, String)> {
    if line.ends_with(" (fetch)") {
        let mut parts = line.split_whitespace().map(|s| s.trim());
        let name = parts.next().map(String::from);
        let url = parts.next().map(String::from);

        match (name, url) {
            (Some(name), Some(url)) => Some((name, url)),
            _ => None,
        }
    } else {
        None
    }
}

pub(in super::super) struct GitFetcher;

impl GitFetcher {
    pub(in super::super) fn new() -> Result<Self> {
        Ok(GitFetcher)
    }

    fn run_command(&self, args: Vec<&str>) -> Result<String> {
        let res = Command::new("git").args(args.into_iter().map( OsStr::new)).output()?;

        if res.status.success() {
            match String::from_utf8(res.stdout) {
                Ok(v) => Ok(v),
                Err(err) => Err(Error::new(ErrorKind::Other, err)),
            }
        } else {
            let out = String::from_utf8_lossy(res.stdout.as_slice());
            let err = String::from_utf8_lossy(res.stderr.as_slice());
            let msg = format!("GIT command return code {}\nSTDOUT: {:?}\nSTDERR: {:?}", res.status, out, err);

            Err(Error::new(ErrorKind::Other, SimpleError::new(msg)))
        }
    }

    pub(in super::super) fn get_branch(&self) -> Result<Option<String>> {
        let res = self.run_command(vec!["rev-parse", "--abbrev-ref", "HEAD"])?.trim().to_string();

        if res.is_empty() {
            Ok(None)
        } else {
            Ok(Some(res))
        }
    }

    pub(in super::super) fn get_log(&self, kind: LogValueKind) -> Result<Option<String>> {
        let pretty = format!("--pretty=format:%{}", kind.to_format_str());
        let res = self.run_command(vec!["--no-pager", "log", "-1", &pretty])?.trim().to_string();

        if res.is_empty() {
            Ok(None)
        } else {
            Ok(Some(res))
        }
    }

    pub(in super::super) fn get_remotes(&self) -> Result<Option<Vec<(String, String)>>> {
        let r = self.run_command(vec!["remote", "-v"])?.trim().lines().filter_map(extract_remote).collect::<Vec<_>>();

        if r.is_empty() {
            Ok(None)
        } else {
            Ok(Some(r))
        }
    }
}