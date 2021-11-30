use super::{Config, Env, Result};

pub(super) fn load_config(config: &mut Config, env: &Env) -> Result<()> {
    if let Some(v) = env.get_var("APPVEYOR_REPO_BRANCH")? {
        config.git_branch.replace(v);
    }

    if let Some(v) = env.get_var("APPVEYOR_BUILD_ID")? {
        config.service_job_id.replace(v);
    }

    if let Some(v) = env.get_var("APPVEYOR_BUILD_NUMBER")? {
        config.service_number.replace(v);
    }

    if let Some(v) = env.get_var("APPVEYOR_PULL_REQUEST_NUMBER")? {
        config.service_pull_request.replace(v);
    }

    if let Some(v) = env.get_var("APPVEYOR_JOB_NUMBER")? {
        config.service_job_number.replace(v);
    }

    Ok(())
}
