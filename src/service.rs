use crate::env::Env;
use log::{debug, trace};
use std::io::Result;

/// A continuous integration service supported by this crate.
///
/// Each variant maps to a Coveralls service name (see [`Service::get_name`]) and to a set of
/// environment variables that are read to build the configuration. A `Service` can be obtained from
/// its Coveralls name with [`Service::from_name`] or guessed from the environment with
/// [`Service::from_env`].
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Service {
    /// AppVeyor (`appveyor`).
    AppVeyor,

    /// BuildKite (`buildkite`).
    BuildKite,

    /// Circle-CI (`circleci`).
    CircleCI,

    /// GitHub Actions (`github-actions`).
    GithubActions,

    /// Jenkins (`jenkins`).
    Jenkins,

    /// Semaphore-CI (`semaphore-ci`).
    Semaphore,

    /// Travis-CI (`travis-ci`).
    Travis,
}

impl Service {
    /// Return the service matching its Coveralls service name, if any.
    ///
    /// The recognized names are `circleci`, `travis-ci`, `appveyor`, `jenkins`, `semaphore-ci`,
    /// `github-actions` and `buildkite`. Any other name yields `None`.
    pub fn from_name(name: &str) -> Option<Service> {
        match name {
            "circleci" => Some(Service::CircleCI),
            "travis-ci" => Some(Service::Travis),
            "appveyor" => Some(Service::AppVeyor),
            "jenkins" => Some(Service::Jenkins),
            "semaphore-ci" => Some(Service::Semaphore),
            "github-actions" => Some(Service::GithubActions),
            "buildkite" => Some(Service::BuildKite),
            _ => {
                trace!("Service name `{name}` is not recognized");

                None
            }
        }
    }

    /// Guess the service from the environment variables of the current process.
    ///
    /// Each supported service exposes a marker variable (for instance `CIRCLECI`, `TRAVIS` or
    /// `GITHUB_ACTIONS`); the first one found determines the service. Returns `Ok(None)` when no
    /// known marker is set.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if an environment variable holds non-Unicode data.
    pub fn from_env(env: &Env) -> Result<Option<Service>> {
        trace!("Guessing CI service from environment variables");

        if env.get_var("CIRCLECI")?.is_some() {
            debug!("Detected Circle-CI from the CIRCLECI variable");

            return Ok(Some(Service::CircleCI));
        }

        if env.get_var("TRAVIS")?.is_some() {
            debug!("Detected Travis-CI from the TRAVIS variable");

            return Ok(Some(Service::Travis));
        }

        if env.get_var("GITHUB_ACTIONS")?.is_some() {
            debug!("Detected GitHub Actions from the GITHUB_ACTIONS variable");

            return Ok(Some(Service::GithubActions));
        }

        if env.get_var("JENKINS_HOME")?.is_some() {
            debug!("Detected Jenkins from the JENKINS_HOME variable");

            return Ok(Some(Service::Jenkins));
        }

        if env.get_var("APPVEYOR")?.is_some() {
            debug!("Detected AppVeyor from the APPVEYOR variable");

            return Ok(Some(Service::AppVeyor));
        }

        if env.get_var("SEMAPHORE")?.is_some() {
            debug!("Detected Semaphore-CI from the SEMAPHORE variable");

            return Ok(Some(Service::Semaphore));
        }

        if env.get_var("BUILDKITE")?.is_some() {
            debug!("Detected BuildKite from the BUILDKITE variable");

            return Ok(Some(Service::BuildKite));
        }

        debug!("No known CI service detected from the environment");

        Ok(None)
    }

    /// Return the Coveralls service name, as expected in the uploaded job payload.
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
