use super::{Config, Env, Result};

pub(super) fn load_config(config: &mut Config, env: &Env) -> Result<()> {
    if let Some(v) = env.get_var("BUILDKITE_JOB_ID")? {
        config.service_job_id.replace(v);
    }

    if let Some(v) = env.get_var("BUILDKITE_BUILD_ID ")? {
        config.service_build_id.replace(v);
    }

    if let Some(v) = env.get_var("BUILDKITE_PULL_REQUEST")? {
        config.service_pull_request.replace(v);
    }

    if let Some(v) = env.get_var("BUILDKITE_BUILD_NUMBER")? {
        config.service_build_number.replace(v);
    }

    if let Some(v) = env.get_var("BUILDKITE_BUILD_URL")? {
        config.service_build_url.replace(v);
    }

    if let Some(v) = env.get_var("BUILDKITE_COMMIT")? {
        config.git_id.replace(v);
    }

    if let Some(v) = env.get_var("BUILDKITE_MESSAGE")? {
        config.git_message.replace(v);
    }

    if let Some(v) = env.get_var("BUILDKITE_BRANCH")? {
        config.git_branch.replace(v);
    }

    if let Some(v) = env.get_var("BUILDKITE_TAG")? {
        config.git_tag.replace(v);
    }

    if let Some(v) = env.get_var("BUILDKITE_BUILD_AUTHOR")? {
        config.git_author_name.replace(v);
    }

    if let Some(v) = env.get_var("BUILDKITE_BUILD_AUTHOR_EMAIL")? {
        config.git_author_email.replace(v);
    }

    if let Some(v) = env.get_var("BUILDKITE_BUILD_CREATOR")? {
        config.git_committer_name.replace(v);
    }

    if let Some(v) = env.get_var("BUILDKITE_BUILD_CREATOR_EMAIL")? {
        config.git_committer_email.replace(v);
    }

    Ok(())
}
