use super::{Config, Env, Result};

pub(super) fn load_config(config: &mut Config, env: &Env) -> Result<()> {
    if let Some(v) = env.get_var("SEMAPHORE_GIT_BRANCH")? {
        config.git_branch.replace(v);
    }

    // Classic
    if let Some(v) = env.get_var("SEMAPHORE_JOB_UUID")? {
        config.service_job_id.replace(v);
    }

    // 2.0
    if let Some(v) = env.get_var("SEMAPHORE_JOB_ID")? {
        config.service_job_id.replace(v);
    }

    // Classic
    if let Some(v) = env.get_var("SEMAPHORE_EXECUTABLE_UUID")? {
        config.service_build_number.replace(v);
    }

    // 2.0
    if let Some(v) = env.get_var("SEMAPHORE_WORKFLOW_ID")? {
        config.service_build_number.replace(v);
    }

    // Classic
    if let Some(v) = env.get_var("SEMAPHORE_BRANCH_ID")? {
        config.service_pull_request.replace(v);
    }

    // 2.0
    if let Some(v) = env.get_var("SEMAPHORE_GIT_PR_NUMBER")? {
        config.service_pull_request.replace(v);
    }

    if let Some(v) = env.get_var("SEMAPHORE_WORKFLOW_NUMBER")? {
        config.service_job_number.replace(v);
    }

    Ok(())
}
