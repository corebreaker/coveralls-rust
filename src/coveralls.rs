use crate::{config::Config, coverage::Coverage, git::GitInfos, api, helpers};
use simple_error::SimpleError;
use log::{debug, info, warn};
use std::{
    io::{Result, Error, ErrorKind},
    path::PathBuf,
};

/// Bridge between a [`Config`] and a [`Coverage`] report, in charge of finalizing and uploading it.
///
/// The manager has no state of its own; it is a small handle whose two operations are applying a
/// configuration to a report ([`apply_config`](CoverallsManager::apply_config)) and uploading the
/// result to the Coveralls API ([`send`](CoverallsManager::send)).
pub struct CoverallsManager;

impl CoverallsManager {
    /// Create a new manager.
    pub fn new() -> CoverallsManager {
        CoverallsManager
    }

    /// Enrich a coverage report in place with the values held by the configuration.
    ///
    /// This sets the service name, repository token, flag name and service identifiers on the
    /// report, then resolves the Git metadata and prunes the source files according to the
    /// configured rules:
    ///
    /// - Git information is fetched from the local repository when `fetch_git_infos` is `true` or when the report
    ///   carries none; otherwise the existing data is updated from the config.
    /// - source files whose path is absolute are dropped when [`param_prune_absolutes`](Config) is set, those under a
    ///   configured pruned directory are dropped, and a source prefix is prepended to the remaining paths when
    ///   configured.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if the Git metadata cannot be collected or is incomplete, or
    /// if the repository token is missing from both the report and the configuration.
    pub fn apply_config(&self, config: &Config, coverage: &mut Coverage, mut fetch_git_infos: bool) -> Result<()> {
        coverage.service_name = config.service.get_name().to_string();

        if let Some(infos) = coverage.git.as_mut() {
            infos.update(config)?;
        } else {
            warn!("No Git information in the coverage report; it will be fetched from the local repository");

            fetch_git_infos = true;
        }

        if fetch_git_infos {
            info!("Fetching git infos...");
            let mut infos = GitInfos::default();

            infos.update(config)?;
            coverage.git.replace(infos);
        }

        if let Some(v) = config.repo_token.as_ref() {
            coverage.repo_token = v.clone();
        }

        if coverage.repo_token.is_empty() {
            let msg = String::from("Repo token is missing, set the COVERALLS_REPO_TOKEN env var.");

            return Err(Error::new(ErrorKind::Other, SimpleError::new(msg)));
        }

        if let Some(v) = config.flag_name.as_ref() {
            coverage.flag_name.replace(v.clone());
        }

        if let Some(v) = config.service_build_number.as_ref() {
            coverage.service_number = v.clone();
        }

        if let Some(v) = config.service_pull_request.as_ref() {
            coverage.service_pull_request = v.clone();
        }

        if let Some(v) = config.service_job_id.as_ref() {
            coverage.service_job_id = v.clone();
        }

        let mut sources = vec![];
        let total = coverage.source_files.len();

        'sources: for mut source in coverage.source_files.drain(..) {
            let mut path = PathBuf::from(&source.name);

            if config.param_prune_absolutes && path.is_absolute() {
                continue;
            }

            for prefix in &config.param_prune_dirs {
                if path.starts_with(prefix) {
                    continue 'sources;
                }
            }

            if let Some(prefix) = config.param_src_prefix.as_ref() {
                path = prefix.join(path);
            }

            source.name = helpers::path_to_string(&path);
            sources.push(source)
        }

        debug!("Kept {} of {} source file(s) after pruning", sources.len(), total);

        coverage.source_files = sources;

        Ok(())
    }

    /// Upload the coverage report to the Coveralls API.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if the request cannot be built or sent, or if the API responds
    /// with a non-`200` status.
    pub fn send(&self, coverage: &Coverage) -> Result<()> {
        api::send_to_api(coverage)
    }
}
