use clap::{Parser, Args, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[clap(
    version,
    author,
    about,
    subcommand_required = true,
    arg_required_else_help = true,
    propagate_version = true,
    after_help = "\
        The sub-command name is the service name (i.e.: circleci for Circle-CI), \
        except the subcommand `env` for guessing the service name from environment variables, \
        and the subcommand `help` for printring this help.\n\
        \n\
        For each sub-command, command line arguments override environment variables \
        (except for the subcommand `env` which has no argument).\n\
        \n\
        Common environment variables:\n\
        - COVERALLS_REPO_TOKEN:    Coveralls repo token\n\
        - COVERALLS_FLAG_NAME:     Coveralls flag name\n\
        - GIT_ID:                  Git ID\n\
        - GIT_MESSAGE:             Git message\n\
        - GIT_AUTHOR_NAME:         Git author name\n\
        - GIT_AUTHOR_EMAIL:        Git author email\n\
        - GIT_COMMITTER_NAME:      Git committer name\n\
        - GIT_COMMITTER_EMAIL:     Git committer email\n\
        - GIT_REMOTE:              Git remote name\n\
        - GIT_URL:                 Git remote URL\n\
        - GIT_BRANCH, BRANCH_NAME: Git branch\n\
        \n\
    "
)]
pub struct CliArgs {
    /// Use `file_name` as input file instead of standard input
    #[clap(value_name = "file_name", global = true)]
    pub input: Option<PathBuf>,

    /// Use `file` as output file for writing what should be sent to Coveralls
    #[clap(short='O', long, value_name = "file", global = true)]
    pub output: Option<PathBuf>,

    /// Add a prefix to all files
    #[clap(short='P', long, value_name = "prefix", global = true)]
    pub source_prefix: Option<PathBuf>,

    /// Prune directory
    #[clap(short='D', long, value_name = "dir", global = true)]
    pub prune_dir: Option<Vec<PathBuf>>,

    /// Force fetching of repository informations from Git
    #[clap(short='F', long, value_name = "dir", global = true)]
    pub force_fetch_git_infos: bool,

    /// Prune absolute paths
    #[clap(short='X', long, global = true)]
    pub prune_absolutes: bool,

    /// Don't send to Coveralls
    #[clap(short='z', long, global = true)]
    pub no_send: bool,

    #[clap(subcommand)]
    pub service: CliService,
}

#[derive(Subcommand)]
pub enum CliService {
    /// Service Circle-CI
    #[clap(name = "circleci", after_help = "\
        Used environment variables for Circle-CI:\n\
        - CIRCLE_PULL_REQUEST:    Service pull request\n\
        - CIRCLE_BUILD_URL:       Service build url\n\
        - CIRCLE_PROJECT_ID:      Service project ID\n\
        - CIRCLE_WORKFLOW_JOB_ID: Service job ID\n\
        - CIRCLE_JOB:             Service job name\n\
        - CIRCLE_BUILD_NUM:       Service build number\n\
        - CIRCLE_BRANCH:          Service branch\n\
        - CIRCLE_TAG:             Service tag\n\
        - CIRCLE_SHA1:            Service commit ID\n\
        \n\
        Common environment variables:\n\
        - GIT_ID:                  Git ID\n\
        - GIT_MESSAGE:             Git message\n\
        - GIT_AUTHOR_NAME:         Git author name\n\
        - GIT_AUTHOR_EMAIL:        Git author email\n\
        - GIT_COMMITTER_NAME:      Git committer name\n\
        - GIT_COMMITTER_EMAIL:     Git committer email\n\
        - GIT_REMOTE:              Git remote name\n\
        - GIT_URL:                 Git remote URL\n\
        - GIT_BRANCH, BRANCH_NAME: Git branch\n\
        - GIT_TAG:                 Git tag\n\
        \n\
    ")]
    CircleCI(CliServiceArgs),

    /// Service Github-Actions
    #[clap(name = "actions", after_help = "\
        Used environment variables for GitHub Actions:\n\
        - GITHUB_REF, GITHUB_HEAD_REF: Git branch\n\
        - GITHUB_RUN_ID:               Service number\n\
        - GITHUB_REF:                  Service pull request\n\
        - GITHUB_JOB:                  Service job id\n\
        - GITHUB_RUN_NUMBER:           Service job number\n\
        \n\
        Common environment variables:\n\
        - GIT_ID:                  Git ID\n\
        - GIT_MESSAGE:             Git message\n\
        - GIT_AUTHOR_NAME:         Git author name\n\
        - GIT_AUTHOR_EMAIL:        Git author email\n\
        - GIT_COMMITTER_NAME:      Git committer name\n\
        - GIT_COMMITTER_EMAIL:     Git committer email\n\
        - GIT_REMOTE:              Git remote name\n\
        - GIT_URL:                 Git remote URL\n\
        - GIT_BRANCH, BRANCH_NAME: Git branch\n\
        - GIT_TAG:                 Git tag\n\
        \n\
    ")]
    Actions(CliServiceArgs),

    /// Service AppVeyor
    #[clap(name = "appveyor", after_help = "\
        Used environment variables for AppVeyor:\n\
        - APPVEYOR_PROJECT_ID:               Service project ID\n\
        - APPVEYOR_BUILD_ID:                 Service build id\n\
        - APPVEYOR_BUILD_NUMBER:             Service number\n\
        - APPVEYOR_BUILD_VERSION:            Service build version\n\
        - APPVEYOR_PULL_REQUEST_NUMBER:      Service pull request\n\
        - APPVEYOR_JOB_ID:                   Service job id\n\
        - APPVEYOR_JOB_NUMBER:               Service job number\n\
        - APPVEYOR_JOB_NAME:                 Service job name\n\
        - APPVEYOR_REPO_BRANCH:              Service repo branch\n\
        - APPVEYOR_REPO_NAME:                Service repo name\n\
        - APPVEYOR_REPO_TAG_NAME:            Service repo tag name\n\
        - APPVEYOR_REPO_COMMIT:              Service repo commit ID\n\
        - APPVEYOR_REPO_COMMIT_MESSAGE:      Service repo commit message\n\
        - APPVEYOR_REPO_COMMIT_AUTHOR:       Service repo commit author\n\
        - APPVEYOR_REPO_COMMIT_AUTHOR_EMAIL: Service repo commit email\n\
        \n\
        Common environment variables:\n\
        - GIT_ID:                  Git ID\n\
        - GIT_MESSAGE:             Git message\n\
        - GIT_AUTHOR_NAME:         Git author name\n\
        - GIT_AUTHOR_EMAIL:        Git author email\n\
        - GIT_COMMITTER_NAME:      Git committer name\n\
        - GIT_COMMITTER_EMAIL:     Git committer email\n\
        - GIT_REMOTE:              Git remote name\n\
        - GIT_URL:                 Git remote URL\n\
        - GIT_BRANCH, BRANCH_NAME: Git branch\n\
        - GIT_TAG:                 Git tag\n\
        \n\
    ")]
    AppVeyor(CliServiceArgs),

    /// Service BuildKite
    #[clap(name = "buildkite", after_help = "\
        Used environment variables for BuildKite:\n\
        - BUILDKITE_COMMIT:              Service commit ID\n\
        - BUILDKITE_MESSAGE:             Service message\n\
        - BUILDKITE_BRANCH:              Service branch\n\
        - BUILDKITE_TAG:                 Service tag\n\
        - BUILDKITE_PULL_REQUEST:        Service pull request\n\
        - BUILDKITE_JOB_ID:              Service job id\n\
        - BUILDKITE_BUILD_ID :           Service build ID\n\
        - BUILDKITE_BUILD_URL:           Service build url\n\
        - BUILDKITE_BUILD_NUMBER:        Service number\n\
        - BUILDKITE_BUILD_AUTHOR:        Service build author\n\
        - BUILDKITE_BUILD_AUTHOR_EMAIL:  Service build author email\n\
        - BUILDKITE_BUILD_CREATOR:       Service build creator\n\
        - BUILDKITE_BUILD_CREATOR_EMAIL: Service build creator email\n\
        \n\
        Common environment variables:\n\
        - GIT_ID:                  Git ID\n\
        - GIT_MESSAGE:             Git message\n\
        - GIT_AUTHOR_NAME:         Git author name\n\
        - GIT_AUTHOR_EMAIL:        Git author email\n\
        - GIT_COMMITTER_NAME:      Git committer name\n\
        - GIT_COMMITTER_EMAIL:     Git committer email\n\
        - GIT_REMOTE:              Git remote name\n\
        - GIT_URL:                 Git remote URL\n\
        - GIT_BRANCH, BRANCH_NAME: Git branch\n\
        - GIT_TAG:                 Git tag\n\
        \n\
    ")]
    BuildKite(CliServiceArgs),

    /// Service Travis
    #[clap(name = "travis", after_help = "\
        Used environment variables for Travis-CI:\n\
        - TRAVIS_BRANCH:        Git branch\n\
        - TRAVIS_BUILD_NUMBER:  Service number\n\
        - TRAVIS_PULL_REQUEST:  Service pull request\n\
        - TRAVIS_BUILD_WEB_URL: Service build url\n\
        - TRAVIS_JOB_ID:        Service job id\n\
        - TRAVIS_JOB_NUMBER:    Service job number\n\
        \n\
        Common environment variables:\n\
        - GIT_ID:                  Git ID\n\
        - GIT_MESSAGE:             Git message\n\
        - GIT_AUTHOR_NAME:         Git author name\n\
        - GIT_AUTHOR_EMAIL:        Git author email\n\
        - GIT_COMMITTER_NAME:      Git committer name\n\
        - GIT_COMMITTER_EMAIL:     Git committer email\n\
        - GIT_REMOTE:              Git remote name\n\
        - GIT_URL:                 Git remote URL\n\
        - GIT_BRANCH, BRANCH_NAME: Git branch\n\
        - GIT_TAG:                 Git tag\n\
        \n\
    ")]
    Travis(CliServiceArgs),

    /// Service Semaphore-CI
    #[clap(name = "semaphore", after_help = "\
        Used environment variables for Semaphore-CI:\n\
        - SEMAPHORE_GIT_BRANCH:                             Git branch\n\
        - SEMAPHORE_EXECUTABLE_UUID, SEMAPHORE_WORKFLOW_ID: Service number\n\
        - SEMAPHORE_BRANCH_ID, SEMAPHORE_GIT_PR_NUMBER:     Service pull request\n\
        - SEMAPHORE_JOB_UUID, SEMAPHORE_JOB_ID:             Service job id\n\
        - SEMAPHORE_WORKFLOW_NUMBER:                        Service job number\n\
        \n\
        Common environment variables:\n\
        - GIT_ID:                  Git ID\n\
        - GIT_MESSAGE:             Git message\n\
        - GIT_AUTHOR_NAME:         Git author name\n\
        - GIT_AUTHOR_EMAIL:        Git author email\n\
        - GIT_COMMITTER_NAME:      Git committer name\n\
        - GIT_COMMITTER_EMAIL:     Git committer email\n\
        - GIT_REMOTE:              Git remote name\n\
        - GIT_URL:                 Git remote URL\n\
        - GIT_BRANCH, BRANCH_NAME: Git branch\n\
        - GIT_TAG:                 Git tag\n\
        \n\
    ")]
    Semaphore(CliServiceArgs),

    /// Service Jenkins
    #[clap(name = "jenkins", after_help = "\
        Used environment variables for Jenkins:\n\
        - BUILD_NUMBER:         Service number\n\
        - CI_PULL_REQUEST:      Service pull request\n\
        - BUILD_URL:            Service build url\n\
        - BUILD_ID:             Service job id\n\
        \n\
        Common environment variables:\n\
        - GIT_ID:                  Git ID\n\
        - GIT_MESSAGE:             Git message\n\
        - GIT_AUTHOR_NAME:         Git author name\n\
        - GIT_AUTHOR_EMAIL:        Git author email\n\
        - GIT_COMMITTER_NAME:      Git committer name\n\
        - GIT_COMMITTER_EMAIL:     Git committer email\n\
        - GIT_REMOTE:              Git remote name\n\
        - GIT_URL:                 Git remote URL\n\
        - GIT_BRANCH, BRANCH_NAME: Git branch\n\
        - GIT_TAG:                 Git tag\n\
        \n\
    ")]
    Jenkins(CliServiceArgs),

    /// Guess service from environment
    #[clap(name = "env", after_help = "\
        Used environment variables in a generic context:\n\
        - CI_NAME:            Service name: \
            circleci, travis-ci, appveyor, jenkins, semaphore-ci, github-actions, buildkite\n\
        - CI_JOB_ID:          Service job ID\n\
        - CI_JOB_NUMBER:      Service job number\n\
        - CI_PULL_REQUEST:    Service pull request\n\
        - CI_PROJECT_ID:      Service project ID\n\
        - CI_BUILD_ID:        Service build ID\n\
        - CI_BUILD_VERSION:   Service build version\n\
        - CI_BUILD_NUMBER:    Service number\n\
        - CI_BUILD_URL:       Service build URL\n\
        - CI_JOB_NAME:        Service job name\n\
        - CI_REPO_NAME:       Service repo name\n\
        - CI_COMMIT:          Service commit ID\n\
        - CI_REMOTE:          Service remote name\n\
        - CI_REMOTE_URL:      Service remote URL\n\
        - CI_AUTHOR_NAME:     Service author name\n\
        - CI_AUTHOR_EMAIL:    Service author email\n\
        - CI_COMMITER_NAME:   Service committer name\n\
        - CI_COMMITTER_EMAIL: Service committer email\n\
        - CI_BRANCH:          Service branch\n\
        - CI_TAG:             Service tag\n\
        \n\
        Used environment variables with Coveralls variables:
        - COVERALLS_REPO_TOKEN:         Coveralls repo token\n\
        - COVERALLS_SERVICE_NAME:       Service name: \
            circleci, travis-ci, appveyor, jenkins, semaphore-ci, github-actions, buildkite\n\
        - COVERALLS_SERVICE_NUMBER:     Service number\n\
        - COVERALLS_BUILD_URL:          Service build URL\n\
        - COVERALLS_SERVICE_JOB_ID:     Service job ID\n\
        - COVERALLS_SERVICE_JOB_NUMBER: Service job number\n\
        - COVERALLS_PULL_REQUEST:       Service pull request\n\
        - COVERALLS_BRANCH:             Git branch\n\n\
        \n\
        Common environment variables:\n\
        - COVERALLS_REPO_TOKEN:    Coveralls repo token\n\
        - COVERALLS_FLAG_NAME:     Coveralls flag name\n\
        - GIT_ID:                  Git ID\n\
        - GIT_MESSAGE:             Git message\n\
        - GIT_AUTHOR_NAME:         Git author name\n\
        - GIT_AUTHOR_EMAIL:        Git author email\n\
        - GIT_COMMITTER_NAME:      Git committer name\n\
        - GIT_COMMITTER_EMAIL:     Git committer email\n\
        - GIT_REMOTE:              Git remote name\n\
        - GIT_URL:                 Git remote URL\n\
        - GIT_BRANCH, BRANCH_NAME: Git branch\n\
        - GIT_TAG:                 Git tag\n\
        \n\
    ")]
    Env,
}

#[derive(Args)]
pub struct CliServiceArgs {
    /// Repo token
    #[clap(short='t', long, value_name = "token")]
    pub repo_token: Option<String>,

    /// Flag name
    #[clap(short='f', long, value_name = "flag_name")]
    pub flag_name: Option<String>,

    /// Project ID
    #[clap(short='P', long, value_name = "id")]
    pub project_id: Option<String>,

    /// Service build ID
    #[clap(short='b', long, value_name = "build_id")]
    pub service_build_id: Option<String>,

    /// Service build number
    #[clap(short='s', long, value_name = "build_number")]
    pub service_build_number: Option<String>,

    /// Service build version
    #[clap(short='v', long, value_name = "build_version")]
    pub service_build_version: Option<String>,

    /// Service build URL
    #[clap(short='u', long, value_name = "url")]
    pub service_build_url: Option<String>,

    /// Service pull request
    #[clap(short='p', long, value_name = "pull_request")]
    pub service_pull_request: Option<String>,

    /// Service job ID
    #[clap(short='j', long, value_name = "job_id")]
    pub service_job_id: Option<String>,

    /// Service job name
    #[clap(short='n', long, value_name = "job_name")]
    pub service_job_name: Option<String>,

    /// Service job number
    #[clap(short='i', long, value_name = "job_number")]
    pub service_job_number: Option<String>,

    /// Service repo name
    #[clap(short='r', long, value_name = "name")]
    pub service_repo_name: Option<String>,

    /// Git ID
    #[clap(short='K', long, value_name = "id")]
    pub git_id: Option<String>,

    /// Git branch
    #[clap(short='B', long, value_name = "branch")]
    pub git_branch: Option<String>,

    /// Git tag
    #[clap(short='T', long, value_name = "tag")]
    pub git_tag: Option<String>,

    /// Git message
    #[clap(short='M', long, value_name = "message")]
    pub git_message: Option<String>,

    /// Git author name
    #[clap(short='a', long, value_name = "name")]
    pub git_author_name: Option<String>,

    /// Git author email
    #[clap(short='A', long, value_name = "email")]
    pub git_author_email: Option<String>,

    /// Git committer name
    #[clap(short='c', long, value_name = "name")]
    pub git_committer_name: Option<String>,

    /// Git committer email
    #[clap(short='C', long, value_name = "email")]
    pub git_committer_email: Option<String>,

    /// Git remote name
    #[clap(short='N', long, value_name = "name")]
    pub git_remote_name: Option<String>,

    /// Git remote URL
    #[clap(short='R', long, value_name = "url")]
    pub git_remote_url: Option<String>,
}
