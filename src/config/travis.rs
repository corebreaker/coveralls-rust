use super::{Config, Env, Result};

pub(super) fn load_config(config: &mut Config, env: &Env) -> Result<()> {
    if let Some(v) = env.get_var("TRAVIS_BRANCH")? {
        config.git_branch.replace(v);
    }

    if let Some(v) = env.get_var("TRAVIS_JOB_ID")? {
        config.service_job_id.replace(v);
    }

    if let Some(v) = env.get_var("TRAVIS_BUILD_NUMBER")? {
        config.service_build_number.replace(v);
    }

    if let Some(v) = env.get_var("TRAVIS_PULL_REQUEST")? {
        config.service_pull_request.replace(v.split("/").last().unwrap_or(&v).to_string());
    }

    if let Some(v) = env.get_var("TRAVIS_BUILD_WEB_URL")? {
        config.service_build_url.replace(v);
    }

    if let Some(v) = env.get_var("TRAVIS_JOB_NUMBER")? {
        config.service_job_number.replace(v);
    }

    Ok(())
}
