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
use clap::ArgMatches;
use itertools::Itertools;
use simple_error::SimpleError;
use std::{io::{Result, Error, ErrorKind}, path::PathBuf};

pub struct Config {
    pub(crate) service: Service,
    pub(crate) flag_name: Option<String>,
    pub(crate) repo_token: Option<String>,
    pub(crate) service_number: Option<String>,
    pub(crate) service_build_url: Option<String>,
    pub(crate) service_pull_request: Option<String>,
    pub(crate) service_job_id: Option<String>,
    pub(crate) service_job_number: Option<String>,
    pub(crate) git_id: Option<String>,
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
            service_number: None,
            service_build_url: None,
            service_pull_request: None,
            service_job_id: None,
            service_job_number: None,
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
            git_branch: match env.get_var("GIT_BRANCH")? {
                Some(v) => Some(v),
                None => env.get_var("BRANCH_NAME")?,
            }
        })
    }

    pub fn load_from_command(cmd_name: &str, args: &ArgMatches, env: &Env) -> Result<Config> {
        let service = match cmd_name {
            "actions" => Service::GithubActions,
            "appveyor" => Service::AppVeyor,
            "buildkite" => Service::BuildKite,
            "circleci" => Service::CircleCI,
            "travis" => Service::Travis,
            "semaphore" => Service::Semaphore,
            "jenkins" => Service::Jenkins,
            name => {
                let msg = format!("Unknown service name: {}", name);

                return Err(Error::new(ErrorKind::Other, SimpleError::new(msg)));
            }
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
        Ok(config)
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

    fn configure(&mut self, args: &ArgMatches) {
        if let Some(v) = args.value_of("repo_token") {
            self.repo_token.replace(v.to_string());
        }

        if let Some(v) = args.value_of("flag_name") {
            self.flag_name.replace(v.to_string());
        }

        if let Some(v) = args.value_of("service_number") {
            self.service_number.replace(v.to_string());
        }

        if let Some(v) = args.value_of("service_build_url") {
            self.service_build_url.replace(v.to_string());
        }

        if let Some(v) = args.value_of("service_pull_request") {
            self.service_pull_request.replace(v.to_string());
        }

        if let Some(v) = args.value_of("service_job_id") {
            self.service_job_id.replace(v.to_string());
        }

        if let Some(v) = args.value_of("service_job_number") {
            self.service_job_number.replace(v.to_string());
        }

        if let Some(v) = args.value_of("git_id") {
            self.repo_token.replace(v.to_string());
        }

        if let Some(v) = args.value_of("git_branch") {
            self.git_id.replace(v.to_string());
        }

        if let Some(v) = args.value_of("git_message") {
            self.git_message.replace(v.to_string());
        }

        if let Some(v) = args.value_of("git_author_name") {
            self.git_author_name.replace(v.to_string());
        }

        if let Some(v) = args.value_of("git_author_email") {
            self.git_author_email.replace(v.to_string());
        }

        if let Some(v) = args.value_of("git_committer_name") {
            self.git_committer_name.replace(v.to_string());
        }

        if let Some(v) = args.value_of("git_committer_email") {
            self.git_committer_email.replace(v.to_string());
        }

        if let Some(v) = args.value_of("git_remote_name") {
            self.git_remote_name.replace(v.to_string());
        }

        if let Some(v) = args.value_of("git_remote_url") {
            self.git_remote_url.replace(v.to_string());
        }
    }

    pub fn init_parameters(mut self, args: &ArgMatches) -> Self {
        if args.is_present("prune_absolutes") {
            self.param_prune_absolutes = true;
        }

        if let Some(prefix) = args.value_of_os("source_prefix") {
            self.param_src_prefix.replace(PathBuf::from(prefix.to_os_string()));
        }

        if let Some(dirs) = args.values_of_os("prune_dir") {
            self.param_prune_dirs = dirs.into_iter().map(|v| PathBuf::from(v.to_os_string())).collect();
        }

        self
    }

    pub fn show(&self) {
        let empty = String::new();
        let prune_dirs = self.param_prune_dirs.iter().map(helpers::path_to_string).join(", ");
        let source_prefix = self.param_src_prefix.as_ref().map(helpers::path_to_string).unwrap_or_else(String::new);

        println!("Parameters:");
        println!("Prune absolute paths: {}", self.param_prune_absolutes);
        println!("Prune directories: .. [{}]", prune_dirs);
        println!("Source prefix: ...... [{}]", source_prefix);
        println!();
        println!("Configuration:");
        println!("Service name: ....... {}", self.service.get_name());
        println!("Repo token: ......... [{}]", self.repo_token.as_ref().unwrap_or(&empty));
        println!("Flag name: .......... [{}]", self.flag_name.as_ref().unwrap_or(&empty));
        println!("Service number: ..... [{}]", self.service_number.as_ref().unwrap_or(&empty));
        println!("Service build URL: .. [{}]", self.service_build_url.as_ref().unwrap_or(&empty));
        println!("Service pull request: [{}]", self.service_pull_request.as_ref().unwrap_or(&empty));
        println!("Service job id: ..... [{}]", self.service_job_id.as_ref().unwrap_or(&empty));
        println!("Service job number: . [{}]", self.service_job_number.as_ref().unwrap_or(&empty));
        println!("Git ID: ............. [{}]", self.git_id.as_ref().unwrap_or(&empty));
        println!("Git branch: ......... [{}]", self.git_branch.as_ref().unwrap_or(&empty));
        println!("Git author name: .... [{}]", self.git_author_name.as_ref().unwrap_or(&empty));
        println!("Git author email: ... [{}]", self.git_author_email.as_ref().unwrap_or(&empty));
        println!("Git committer name: . [{}]", self.git_committer_name.as_ref().unwrap_or(&empty));
        println!("Git committer email:  [{}]", self.git_committer_email.as_ref().unwrap_or(&empty));
        println!("Git remote name: .... [{}]", self.git_remote_name.as_ref().unwrap_or(&empty));
        println!("Git remote URL: ..... [{}]", self.git_remote_url.as_ref().unwrap_or(&empty));
        println!("Git message: ........ [{}]", self.git_message.as_ref().unwrap_or(&empty));
    }
}
