use super::{Config, Env, Result};

pub(super) fn load_config(config: &mut Config, env: &Env) -> Result<()> {
    if let Some(v) = env.get_github_actions_branch()? {
        config.git_branch.replace(v);
    }

    if let Some(v) = env.get_var("GITHUB_JOB")? {
        config.service_job_id.replace(v);
    }

    if let Some(v) = env.get_var("GITHUB_RUN_ID")? {
        config.service_build_number.replace(v);
    }

    if let Some(v) = env.get_github_actions_pull_request()? {
        config.service_pull_request.replace(v);
    }

    if let Some(v) = env.get_var("GITHUB_RUN_NUMBER")? {
        config.service_job_number.replace(v);
    }

    Ok(())
}
