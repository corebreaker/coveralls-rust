use super::{Config, Env, Result};

pub(super) fn load_config(config: &mut Config, env: &Env) -> Result<()> {
    if let Some(v) = env.get_var("COVERALLS_SERVICE_NUMBER")? {
        config.service_build_number.replace(v);
    }

    if let Some(v) = env.get_var("COVERALLS_BUILD_URL")? {
        config.service_build_url.replace(v);
    }

    if let Some(v) = env.get_var("COVERALLS_SERVICE_JOB_ID")? {
        config.service_job_id.replace(v);
    }

    if let Some(v) = env.get_var("COVERALLS_SERVICE_JOB_NUMBER")? {
        config.service_job_number.replace(v);
    }

    if let Some(v) = env.get_var("COVERALLS_PULL_REQUEST")? {
        config.service_pull_request.replace(v);
    }

    if let Some(v) = env.get_var("COVERALLS_BRANCH")? {
        config.git_branch.replace(v);
    }

    Ok(())
}
