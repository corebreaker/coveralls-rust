use super::{git::GitInfos, api, helpers, Config, Coverage};
use simple_error::SimpleError;
use std::{io::{Result, Error, ErrorKind}, path::PathBuf};

pub struct CoverallsManager;

impl CoverallsManager {
    pub fn new() -> CoverallsManager {
        CoverallsManager
    }

    pub fn apply_config(&self, config: &Config, coverage: &mut Coverage) -> Result<()> {
        coverage.service_name = config.service.get_name().to_string();

        if let Some(infos) = coverage.git.as_mut() {
            infos.update(config)?;
        } else {
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

        if let Some(v) = config.service_number.as_ref() {
            coverage.service_number = v.clone();
        }

        if let Some(v) = config.service_pull_request.as_ref() {
            coverage.service_pull_request = v.clone();
        }

        if let Some(v) = config.service_job_id.as_ref() {
            coverage.service_job_id = v.clone();
        }

        let mut sources = vec![];

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

        coverage.source_files = sources;

        Ok(())
    }

    pub fn send(&self, coverage: &Coverage) -> Result<()> {
        api::send_to_api(coverage)
    }
}