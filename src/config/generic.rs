use super::{Config, Env, Result};

pub(super) fn load_config(config: &mut Config, env: &Env) -> Result<()> {
    if let Some(v) = env.get_var("CI_PROJECT_ID")? {
        config.service_project_id.replace(v);
    }

    if let Some(v) = env.get_var("CI_BUILD_NUMBER")? {
        config.service_build_number.replace(v);
    }

    if let Some(v) = env.get_var("CI_BUILD_ID")? {
        config.service_build_id.replace(v);
    }

    if let Some(v) = env.get_var("CI_BUILD_URL")? {
        config.service_build_url.replace(v);
    }

    if let Some(v) = env.get_var("CI_BUILD_VERSION")? {
        config.service_build_version.replace(v);
    }

    if let Some(v) = env.get_var("CI_JOB_ID")? {
        config.service_job_id.replace(v);
    }

    if let Some(v) = env.get_var("CI_JOB_NAME")? {
        config.service_job_name.replace(v);
    }

    if let Some(v) = env.get_var("CI_JOB_NUMBER")? {
        config.service_job_number.replace(v);
    }

    if let Some(v) = env.get_var("CI_PULL_REQUEST")? {
        config.service_pull_request.replace(v);
    }

    if let Some(v) = env.get_var("CI_REPO_NAME")? {
        config.service_repo_name.replace(v);
    }

    if let Some(v) = env.get_var("CI_COMMIT")? {
        config.git_id.replace(v);
    }

    if let Some(v) = env.get_var("CI_REMOTE")? {
        config.git_remote_name.replace(v);
    }

    if let Some(v) = env.get_var("CI_REMOTE_URL")? {
        config.git_remote_url.replace(v);
    }

    if let Some(v) = env.get_var("CI_AUTHOR_NAME")? {
        config.git_author_name.replace(v);
    }

    if let Some(v) = env.get_var("CI_AUTHOR_EMAIL")? {
        config.git_author_email.replace(v);
    }

    if let Some(v) = env.get_var("CI_COMMITER_NAME")? {
        config.git_committer_name.replace(v);
    }

    if let Some(v) = env.get_var("CI_COMMITTER_EMAIL")? {
        config.git_committer_email.replace(v);
    }

    if let Some(v) = env.get_var("CI_BRANCH")? {
        config.git_branch.replace(v);
    }

    if let Some(v) = env.get_var("CI_TAG")? {
        config.git_tag.replace(v);
    }

    Ok(())
}
