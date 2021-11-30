use super::Env;
use std::io::Result;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Service {
    AppVeyor,
    BuildKite,
    CircleCI,
    GithubActions,
    Jenkins,
    Semaphore,
    Travis,
}

impl Service {
    pub fn from_name(name: &str) -> Option<Service> {
        match name {
            "circleci" => Some(Service::CircleCI),
            "travis-ci" => Some(Service::Travis),
            "appveyor" => Some(Service::AppVeyor),
            "jenkins" => Some(Service::Jenkins),
            "semaphore-ci" => Some(Service::Semaphore),
            "github-actions" => Some(Service::GithubActions),
            "buildkite" => Some(Service::BuildKite),
            _ => None
        }
    }

    pub fn from_env(env: &Env) -> Result<Option<Service>> {
        if env.get_var("CIRCLECI")?.is_some() {
            return Ok(Some(Service::CircleCI));
        }

        if env.get_var("TRAVIS")?.is_some() {
            return Ok(Some(Service::Travis));
        }

        if env.get_var("GITHUB_ACTIONS")?.is_some() {
            return Ok(Some(Service::GithubActions));
        }

        if env.get_var("JENKINS_HOME")?.is_some() {
            return Ok(Some(Service::Jenkins));
        }

        if env.get_var("APPVEYOR")?.is_some() {
            return Ok(Some(Service::AppVeyor));
        }

        if env.get_var("SEMAPHORE")?.is_some() {
            return Ok(Some(Service::Semaphore));
        }

        if env.get_var("BUILDKITE")?.is_some() {
            return Ok(Some(Service::BuildKite));
        }

        Ok(None)
    }

    pub fn get_name(&self) -> &'static str {
        match self {
            Self::CircleCI => "circleci",
            Self::AppVeyor => "appveyor",
            Self::BuildKite => "buildkite",
            Self::GithubActions => "github-actions",
            Self::Jenkins => "jenkins",
            Self::Semaphore => "semaphore-ci",
            Self::Travis => "travis-ci",
        }
    }
}