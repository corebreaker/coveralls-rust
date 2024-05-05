mod travis;
mod generic;
mod jenkins;
mod appveyor;
mod circleci;
mod buildkite;
mod semaphore;
mod coveralls_env;
mod github_actions;

use super::{Service, Env, helpers};
use crate::{cli_args::{CliArgs, CliService, CliServiceArgs}, git::GitInfos};
use itertools::Itertools;
use simple_error::SimpleError;
use std::{io::{Result, Error, ErrorKind}, path::PathBuf};

pub struct Config {
    pub(crate) service: Service,
    pub(crate) flag_name: Option<String>,
    pub(crate) repo_token: Option<String>,
    pub(crate) service_project_id: Option<String>,
    pub(crate) service_build_id: Option<String>,
    pub(crate) service_build_version: Option<String>,
    pub(crate) service_build_number: Option<String>,
    pub(crate) service_build_url: Option<String>,
    pub(crate) service_pull_request: Option<String>,
    pub(crate) service_job_id: Option<String>,
    pub(crate) service_job_name: Option<String>,
    pub(crate) service_job_number: Option<String>,
    pub(crate) service_repo_name: Option<String>,
    pub(crate) git_id: Option<String>,
    pub(crate) git_tag: Option<String>,
    pub(crate) git_branch: Option<String>,
    pub(crate) git_message: Option<String>,
    pub(crate) git_author_name: Option<String>,
    pub(crate) git_author_email: Option<String>,
    pub(crate) git_committer_name: Option<String>,
    pub(crate) git_committer_email: Option<String>,
    pub(crate) git_remote_name: Option<String>,
    pub(crate) git_remote_url: Option<String>,
    pub(crate) param_prune_absolutes: bool,
    pub(crate) param_prune_dirs: Vec<PathBuf>,
    pub(crate) param_src_prefix: Option<PathBuf>,
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

    pub fn load_from_command(cli: &CliArgs, env: &Env) -> Result<Option<Config>> {
        let (service, args) = match &cli.service {
            CliService::Actions(args) => (Service::GithubActions, args),
            CliService::AppVeyor(args) => (Service::AppVeyor, args),
            CliService::BuildKite(args) => (Service::BuildKite, args),
            CliService::CircleCI(args) => (Service::CircleCI, args),
            CliService::Jenkins(args) => (Service::Jenkins, args),
            CliService::Semaphore(args) => (Service::Semaphore, args),
            CliService::Travis(args) => (Service::Travis, args),
            CliService::Env => { return Ok(None); }
        };

        let mut config = Config::new(service, env)?;

        match service {
            Service::AppVeyor => { appveyor::load_config(&mut config, env)?; }
            Service::BuildKite => { buildkite::load_config(&mut config, env)?; }
            Service::CircleCI => { circleci::load_config(&mut config, env)?; }
            Service::GithubActions => { github_actions::load_config(&mut config, env)?; }
            Service::Jenkins => { jenkins::load_config(&mut config, env)?; }
            Service::Semaphore => { semaphore::load_config(&mut config, env)?; }
            Service::Travis => { travis::load_config(&mut config, env)?; }
        }

        config.configure(args);
        Ok(Some(config))
    }

    pub fn load_from_environment(env: &Env) -> Result<Option<Config>> {
        if let Some(name) = env.get_var("CI_NAME")? {
            return if let Some(service) = Service::from_name(&name) {
                let mut config = Config::new(service, env)?;

                generic::load_config(&mut config, env)?;

                Ok(Some(config))
            } else {
                let msg = format!("Service name `{}` is not implemented", name);

                Err(Error::new(ErrorKind::Other, SimpleError::new(msg)))
            };
        }

        if let Some(name) = env.get_var("COVERALLS_SERVICE_NAME")? {
            return if let Some(service) = Service::from_name(&name) {
                let mut config = Config::new(service, env)?;

                coveralls_env::load_config(&mut config, env)?;

                Ok(Some(config))
            } else {
                let msg = format!("Service name `{}` is not implemented", name);

                Err(Error::new(ErrorKind::Other, SimpleError::new(msg)))
            };
        }

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

    pub fn init_parameters(mut self, args: &CliArgs) -> Self {
        self.param_prune_absolutes = args.prune_absolutes;

        if let Some(prefix) = &args.source_prefix {
            self.param_src_prefix.replace(prefix.clone());
        }

        if let Some(dirs) = &args.prune_dir {
            self.param_prune_dirs = dirs.clone();
        }

        self
    }

    pub fn show(&self, git: Option<&GitInfos>) {
        let empty = String::new();
        let prune_dirs = self.param_prune_dirs.iter().map(helpers::path_to_string).join(", ");
        let source_prefix = self.param_src_prefix.as_ref().map(helpers::path_to_string).unwrap_or_else(String::new);

        let git_id = self.git_id.as_ref().or_else(|| git.map(|v| &v.head.id));
        let git_tag = self.git_tag.as_ref();
        let git_branch = self.git_branch.as_ref().or_else(|| git.map(|v| &v.branch));
        let git_author_name = self.git_author_name.as_ref().or_else(|| git.map(|v| &v.head.author_name));
        let git_author_email = self.git_author_email.as_ref().or_else(|| git.map(|v| &v.head.author_email));
        let git_committer_name = self.git_committer_name.as_ref().or_else(|| git.map(|v| &v.head.committer_name));
        let git_committer_email = self.git_committer_email.as_ref().or_else(|| git.map(|v| &v.head.committer_email));
        let git_message = self.git_message.as_ref().or_else(|| git.map(|v| &v.head.message));

        let git_remote_name = self.git_remote_name.as_ref()
            .or_else(|| git.and_then(|v| v.remotes.get(0).map(|v| &v.name)));

        let git_remote_url = self.git_remote_url.as_ref()
            .or_else(|| git.and_then(|v| v.remotes.get(0).map(|v| &v.url)));

        println!("Parameters:");
        println!("Prune absolute paths:  {}", self.param_prune_absolutes);
        println!("Prune directories: ... [{prune_dirs}]");
        println!("Source prefix: ....... [{source_prefix}]");
        println!();

        println!("Configuration:");
        println!("Service name: ........ {}", self.service.get_name());
        println!("Repo token: .......... [{}]", self.repo_token.as_ref().unwrap_or(&empty));
        println!("Repo name: ........... [{}]", self.service_repo_name.as_ref().unwrap_or(&empty));
        println!("Flag name: ........... [{}]", self.flag_name.as_ref().unwrap_or(&empty));
        println!("Service build ID: .... [{}]", self.service_build_id.as_ref().unwrap_or(&empty));
        println!("Service build number:  [{}]", self.service_build_number.as_ref().unwrap_or(&empty));
        println!("Service build version: [{}]", self.service_build_version.as_ref().unwrap_or(&empty));
        println!("Service build URL: ... [{}]", self.service_build_url.as_ref().unwrap_or(&empty));
        println!("Service pull request:  [{}]", self.service_pull_request.as_ref().unwrap_or(&empty));
        println!("Service job ID: ...... [{}]", self.service_job_id.as_ref().unwrap_or(&empty));
        println!("Service job name: .... [{}]", self.service_job_name.as_ref().unwrap_or(&empty));
        println!("Service job number: .. [{}]", self.service_job_number.as_ref().unwrap_or(&empty));

        println!("Git ID: .............. [{}]", git_id.unwrap_or(&empty));
        println!("Git branch: .......... [{}]", git_branch.unwrap_or(&empty));
        println!("Git tag: ............. [{}]", git_tag.unwrap_or(&empty));
        println!("Git author name: ..... [{}]", git_author_name.unwrap_or(&empty));
        println!("Git author email: .... [{}]", git_author_email.unwrap_or(&empty));
        println!("Git committer name: .. [{}]", git_committer_name.unwrap_or(&empty));
        println!("Git committer email: . [{}]", git_committer_email.unwrap_or(&empty));
        println!("Git remote name: ..... [{}]", git_remote_name.unwrap_or(&empty));
        println!("Git remote URL: ...... [{}]", git_remote_url.unwrap_or(&empty));
        println!("Git message: ......... [{}]", git_message.unwrap_or(&empty));
        println!();
    }
}
