//! Configuration of a Coveralls job and its loading from the CLI and the environment.
//!
//! The [`Config`] type is the heart of this module. Each submodule holds the `load_config`
//! function that reads the environment variables specific to one CI service (or the generic and
//! Coveralls variable sets) into a partially built configuration.

mod appveyor;
mod buildkite;
mod circleci;
mod coveralls_env;
mod generic;
mod github_actions;
mod jenkins;
mod semaphore;
mod travis;

use crate::{
    cli_args::{CliArgs, CliService, CliServiceArgs},
    git::GitInfos,
    service::Service,
    env::Env,
    helpers,
};

use itertools::Itertools;
use simple_error::SimpleError;
use log::{debug, info, warn};
use std::{
    io::{Result, Error, ErrorKind},
    path::PathBuf,
};

/// Resolved configuration of a Coveralls job.
///
/// A `Config` aggregates everything needed to finalize a coverage report: the selected
/// [`Service`], the Coveralls credentials, the service-specific build/job identifiers, the Git
/// metadata and the local processing parameters (path pruning and source prefix).
///
/// It is built from the command line or from the environment (see
/// [`Config::load_from_environment`]); service-specific environment variables are read first, then
/// overridden by command line arguments, before the local parameters are applied. [`Config::show`]
/// logs the whole resolved configuration.
pub struct Config {
    pub(crate) service:               Service,
    pub(crate) flag_name:             Option<String>,
    pub(crate) repo_token:            Option<String>,
    pub(crate) service_project_id:    Option<String>,
    pub(crate) service_build_id:      Option<String>,
    pub(crate) service_build_version: Option<String>,
    pub(crate) service_build_number:  Option<String>,
    pub(crate) service_build_url:     Option<String>,
    pub(crate) service_pull_request:  Option<String>,
    pub(crate) service_job_id:        Option<String>,
    pub(crate) service_job_name:      Option<String>,
    pub(crate) service_job_number:    Option<String>,
    pub(crate) service_repo_name:     Option<String>,
    pub(crate) git_id:                Option<String>,
    pub(crate) git_tag:               Option<String>,
    pub(crate) git_branch:            Option<String>,
    pub(crate) git_message:           Option<String>,
    pub(crate) git_author_name:       Option<String>,
    pub(crate) git_author_email:      Option<String>,
    pub(crate) git_committer_name:    Option<String>,
    pub(crate) git_committer_email:   Option<String>,
    pub(crate) git_remote_name:       Option<String>,
    pub(crate) git_remote_url:        Option<String>,
    pub(crate) param_prune_absolutes: bool,
    pub(crate) param_prune_dirs:      Vec<PathBuf>,
    pub(crate) param_src_prefix:      Option<PathBuf>,
}

impl Config {
    fn new(service: Service, env: &Env) -> Result<Config> {
        Ok(Config {
            service,
            flag_name: env.get_var("COVERALLS_FLAG_NAME")?,
            repo_token: env.get_var("COVERALLS_REPO_TOKEN")?,
            service_project_id: None,
            service_build_id: None,
            service_build_version: None,
            service_build_number: None,
            service_build_url: None,
            service_pull_request: None,
            service_job_id: None,
            service_job_name: None,
            service_job_number: None,
            service_repo_name: None,
            param_prune_absolutes: false,
            param_prune_dirs: vec![],
            param_src_prefix: None,
            git_id: env.get_var("GIT_ID")?,
            git_message: env.get_var("GIT_MESSAGE")?,
            git_author_name: env.get_var("GIT_AUTHOR_NAME")?,
            git_author_email: env.get_var("GIT_AUTHOR_EMAIL")?,
            git_committer_name: env.get_var("GIT_COMMITTER_NAME")?,
            git_committer_email: env.get_var("GIT_COMMITTER_EMAIL")?,
            git_remote_name: env.get_var("GIT_REMOTE")?,
            git_remote_url: env.get_var("GIT_URL")?,
            git_tag: env.get_var("GIT_TAG")?,
            git_branch: match env.get_var("GIT_BRANCH")? {
                Some(v) => Some(v),
                None => env.get_var("BRANCH_NAME")?,
            },
        })
    }

    /// Load the environment variables native to `self.service` into the configuration.
    ///
    /// This dispatches to the matching service submodule (for instance `CIRCLE_*` for Circle-CI),
    /// and is shared by the subcommand path and the environment-guessing path.
    fn load_service_variables(&mut self, env: &Env) -> Result<()> {
        match self.service {
            Service::AppVeyor => appveyor::load_config(self, env),
            Service::BuildKite => buildkite::load_config(self, env),
            Service::CircleCI => circleci::load_config(self, env),
            Service::GithubActions => github_actions::load_config(self, env),
            Service::Jenkins => jenkins::load_config(self, env),
            Service::Semaphore => semaphore::load_config(self, env),
            Service::Travis => travis::load_config(self, env),
        }
    }

    /// Build a configuration from the selected command line subcommand.
    ///
    /// The subcommand names the CI service; its service-specific environment variables are loaded
    /// first and then overridden by the command line arguments. Returns `Ok(None)` for the `env`
    /// subcommand, which defers detection to [`Config::load_from_environment`].
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if an environment variable holds non-Unicode data.
    pub(crate) fn load_from_command(cli: &CliArgs, env: &Env) -> Result<Option<Config>> {
        let (service, args) = match &cli.service {
            CliService::Actions(args) => (Service::GithubActions, args),
            CliService::AppVeyor(args) => (Service::AppVeyor, args),
            CliService::BuildKite(args) => (Service::BuildKite, args),
            CliService::CircleCI(args) => (Service::CircleCI, args),
            CliService::Jenkins(args) => (Service::Jenkins, args),
            CliService::Semaphore(args) => (Service::Semaphore, args),
            CliService::Travis(args) => (Service::Travis, args),
            CliService::Env => {
                debug!("No service subcommand provided, will guess the service from the environment");

                return Ok(None);
            }
        };

        debug!("Loading configuration from the `{}` subcommand", service.get_name());

        let mut config = Config::new(service, env)?;

        config.load_service_variables(env)?;
        config.configure(args);
        Ok(Some(config))
    }

    /// Build a configuration by detecting the CI service from the environment.
    ///
    /// The service is determined, in order of precedence, from:
    ///
    /// 1. `CI_NAME`, then the generic `CI_*` variables,
    /// 2. `COVERALLS_SERVICE_NAME`, then the `COVERALLS_*` variables,
    /// 3. the native marker variables of each service (`CIRCLECI`, `TRAVIS`, `GITHUB_ACTIONS`, ...) through
    ///    [`Service::from_env`], in which case that service's own variables are loaded.
    ///
    /// Returns `Ok(None)` when none of them identifies a service.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if a recognized service name is not implemented, or if an
    /// environment variable holds non-Unicode data.
    pub fn load_from_environment(env: &Env) -> Result<Option<Config>> {
        if let Some(name) = env.get_var("CI_NAME")? {
            debug!("Found CI_NAME=`{name}`, loading generic CI configuration");

            return if let Some(service) = Service::from_name(&name) {
                let mut config = Config::new(service, env)?;
                generic::load_config(&mut config, env)?;

                Ok(Some(config))
            } else {
                warn!("CI_NAME=`{name}` designates a service that is not implemented");

                let msg = format!("Service name `{}` is not implemented", name);

                Err(Error::new(ErrorKind::Other, SimpleError::new(msg)))
            };
        }

        if let Some(name) = env.get_var("COVERALLS_SERVICE_NAME")? {
            debug!("Found COVERALLS_SERVICE_NAME=`{name}`, loading Coveralls configuration");

            return if let Some(service) = Service::from_name(&name) {
                let mut config = Config::new(service, env)?;
                coveralls_env::load_config(&mut config, env)?;

                Ok(Some(config))
            } else {
                warn!("COVERALLS_SERVICE_NAME=`{name}` designates a service that is not implemented");

                let msg = format!("Service name `{}` is not implemented", name);

                Err(Error::new(ErrorKind::Other, SimpleError::new(msg)))
            };
        }

        if let Some(service) = Service::from_env(env)? {
            debug!("Guessed service `{}` from its native variables", service.get_name());

            let mut config = Config::new(service, env)?;
            config.load_service_variables(env)?;

            return Ok(Some(config));
        }

        debug!("No CI service detected from the environment");

        Ok(None)
    }

    fn configure(&mut self, args: &CliServiceArgs) {
        if let Some(v) = &args.flag_name {
            self.flag_name.replace(v.clone());
        }

        if let Some(v) = &args.repo_token {
            self.repo_token.replace(v.clone());
        }

        if let Some(v) = &args.service_repo_name {
            self.service_repo_name.replace(v.clone());
        }

        if let Some(v) = &args.project_id {
            self.service_project_id.replace(v.clone());
        }

        if let Some(v) = &args.service_build_id {
            self.service_build_id.replace(v.clone());
        }

        if let Some(v) = &args.service_build_number {
            self.service_build_number.replace(v.clone());
        }

        if let Some(v) = &args.service_build_version {
            self.service_build_version.replace(v.clone());
        }

        if let Some(v) = &args.service_build_url {
            self.service_build_url.replace(v.clone());
        }

        if let Some(v) = &args.service_pull_request {
            self.service_pull_request.replace(v.clone());
        }

        if let Some(v) = &args.service_job_id {
            self.service_job_id.replace(v.clone());
        }

        if let Some(v) = &args.service_job_name {
            self.service_job_name.replace(v.clone());
        }

        if let Some(v) = &args.service_job_number {
            self.service_job_number.replace(v.clone());
        }

        if let Some(v) = &args.git_id {
            self.git_id.replace(v.clone());
        }

        if let Some(v) = &args.git_branch {
            self.git_branch.replace(v.clone());
        }

        if let Some(v) = &args.git_tag {
            self.git_tag.replace(v.clone());
        }

        if let Some(v) = &args.git_message {
            self.git_message.replace(v.clone());
        }

        if let Some(v) = &args.git_author_name {
            self.git_author_name.replace(v.clone());
        }

        if let Some(v) = &args.git_author_email {
            self.git_author_email.replace(v.clone());
        }

        if let Some(v) = &args.git_committer_name {
            self.git_committer_name.replace(v.clone());
        }

        if let Some(v) = &args.git_committer_email {
            self.git_committer_email.replace(v.clone());
        }

        if let Some(v) = &args.git_remote_name {
            self.git_remote_name.replace(v.clone());
        }

        if let Some(v) = &args.git_remote_url {
            self.git_remote_url.replace(v.clone());
        }
    }

    /// Apply the local processing parameters from the global command line arguments.
    ///
    /// These are the report-shaping options that are independent from the CI service: pruning of
    /// absolute paths, list of pruned directories and the source prefix to prepend to every file.
    pub(crate) fn init_parameters(mut self, args: &CliArgs) -> Self {
        self.param_prune_absolutes = args.prune_absolutes;

        if let Some(prefix) = &args.source_prefix {
            self.param_src_prefix.replace(prefix.clone());
        }

        if let Some(dirs) = &args.prune_dir {
            self.param_prune_dirs = dirs.clone();
        }

        debug!(
            "Parameters initialized (prune absolutes: {}, prune dirs: {}, source prefix: {})",
            self.param_prune_absolutes,
            self.param_prune_dirs.len(),
            self.param_src_prefix.is_some()
        );

        self
    }

    /// Log the whole resolved configuration at the `info` level.
    ///
    /// The Git fields fall back to the values discovered in `git` (the metadata collected from the
    /// repository) when they were not set explicitly. This is meant as a human-readable recap of
    /// what will be sent; the repository token is masked (only its last few characters are shown) so
    /// the log can be shared safely.
    pub fn show(&self, git: Option<&GitInfos>) {
        let empty = String::new();
        let prune_dirs = self.param_prune_dirs.iter().map(helpers::path_to_string).join(", ");
        let source_prefix = self
            .param_src_prefix
            .as_ref()
            .map(helpers::path_to_string)
            .unwrap_or_else(String::new);

        let git_id = self.git_id.as_ref().or_else(|| git.map(|v| &v.head.id));
        let git_tag = self.git_tag.as_ref();
        let git_branch = self.git_branch.as_ref().or_else(|| git.map(|v| &v.branch));
        let git_author_name = self
            .git_author_name
            .as_ref()
            .or_else(|| git.map(|v| &v.head.author_name));

        let git_author_email = self
            .git_author_email
            .as_ref()
            .or_else(|| git.map(|v| &v.head.author_email));

        let git_committer_name = self
            .git_committer_name
            .as_ref()
            .or_else(|| git.map(|v| &v.head.committer_name));

        let git_committer_email = self
            .git_committer_email
            .as_ref()
            .or_else(|| git.map(|v| &v.head.committer_email));

        let git_message = self.git_message.as_ref().or_else(|| git.map(|v| &v.head.message));

        let git_remote_name = self
            .git_remote_name
            .as_ref()
            .or_else(|| git.and_then(|v| v.remotes.get(0).map(|v| &v.name)));

        let git_remote_url = self
            .git_remote_url
            .as_ref()
            .or_else(|| git.and_then(|v| v.remotes.get(0).map(|v| &v.url)));

        info!("Parameters:");
        info!("Prune absolute paths:  {}", self.param_prune_absolutes);
        info!("Prune directories: ... [{prune_dirs}]");
        info!("Source prefix: ....... [{source_prefix}]");
        info!("");

        info!("Configuration:");
        info!("Service name: ........ {}", self.service.get_name());
        info!(
            "Repo token: .......... [{}]",
            self.repo_token.as_deref().map(helpers::mask_secret).unwrap_or_default()
        );

        info!(
            "Repo name: ........... [{}]",
            self.service_repo_name.as_ref().unwrap_or(&empty)
        );

        info!("Flag name: ........... [{}]", self.flag_name.as_ref().unwrap_or(&empty));
        info!(
            "Service build ID: .... [{}]",
            self.service_build_id.as_ref().unwrap_or(&empty)
        );

        info!(
            "Service build number:  [{}]",
            self.service_build_number.as_ref().unwrap_or(&empty)
        );

        info!(
            "Service build version: [{}]",
            self.service_build_version.as_ref().unwrap_or(&empty)
        );

        info!(
            "Service build URL: ... [{}]",
            self.service_build_url.as_ref().unwrap_or(&empty)
        );

        info!(
            "Service pull request:  [{}]",
            self.service_pull_request.as_ref().unwrap_or(&empty)
        );

        info!(
            "Service job ID: ...... [{}]",
            self.service_job_id.as_ref().unwrap_or(&empty)
        );

        info!(
            "Service job name: .... [{}]",
            self.service_job_name.as_ref().unwrap_or(&empty)
        );

        info!(
            "Service job number: .. [{}]",
            self.service_job_number.as_ref().unwrap_or(&empty)
        );

        info!("Git ID: .............. [{}]", git_id.unwrap_or(&empty));
        info!("Git branch: .......... [{}]", git_branch.unwrap_or(&empty));
        info!("Git tag: ............. [{}]", git_tag.unwrap_or(&empty));
        info!("Git author name: ..... [{}]", git_author_name.unwrap_or(&empty));
        info!("Git author email: .... [{}]", git_author_email.unwrap_or(&empty));
        info!("Git committer name: .. [{}]", git_committer_name.unwrap_or(&empty));
        info!("Git committer email: . [{}]", git_committer_email.unwrap_or(&empty));
        info!("Git remote name: ..... [{}]", git_remote_name.unwrap_or(&empty));
        info!("Git remote URL: ...... [{}]", git_remote_url.unwrap_or(&empty));
        info!("Git message: ......... [{}]", git_message.unwrap_or(&empty));
        info!("");
    }
}
