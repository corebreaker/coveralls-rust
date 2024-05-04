use super::{Config, Env, Result};

pub(super) fn load_config(config: &mut Config, env: &Env) -> Result<()> {
    if let Some(v) = env.get_var("APPVEYOR_JOB_ID")? {
        config.service_job_id.replace(v);
    }

    if let Some(v) = env.get_var("APPVEYOR_BUILD_NUMBER")? {
        config.service_build_number.replace(v);
    }

    if let Some(v) = env.get_var("APPVEYOR_PULL_REQUEST_NUMBER")? {
        config.service_pull_request.replace(v);
    }

    if let Some(v) = env.get_var("APPVEYOR_JOB_NUMBER")? {
        config.service_job_number.replace(v);
    }

    if let Some(v) = env.get_var("APPVEYOR_PROJECT_ID")? {
        config.service_project_id.replace(v);
    }

    if let Some(v) = env.get_var("APPVEYOR_BUILD_ID")? {
        config.service_build_id.replace(v);
    }

    if let Some(v) = env.get_var("APPVEYOR_BUILD_VERSION")? {
        config.service_build_version.replace(v);
    }

    if let Some(v) = env.get_var("APPVEYOR_JOB_NAME")? {
        config.service_job_name.replace(v);
    }

    if let Some(v) = env.get_var("APPVEYOR_REPO_NAME")? {
        config.service_repo_name.replace(v);
    }

    if let Some(v) = env.get_var("APPVEYOR_REPO_BRANCH")? {
        config.git_branch.replace(v);
    }

    if let Some(v) = env.get_var("APPVEYOR_REPO_TAG_NAME")? {
        config.git_tag.replace(v);
    }

    if let Some(v) = env.get_var("APPVEYOR_REPO_COMMIT")? {
        config.git_id.replace(v);
    }

    if let Some(v) = env.get_var("APPVEYOR_REPO_COMMIT_MESSAGE")? {
        config.git_message.replace(v);
    }

    if let Some(v) = env.get_var("APPVEYOR_REPO_COMMIT_AUTHOR")? {
        config.git_author_name.replace(v);
    }

    if let Some(v) = env.get_var("APPVEYOR_REPO_COMMIT_AUTHOR_EMAIL")? {
        config.git_author_email.replace(v);
    }

    Ok(())
}
