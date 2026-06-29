use crate::{cli_args::CliArgs, coverage::Coverage, coveralls::CoverallsManager, config::Config, env::Env};
use simple_error::SimpleError;
use log::{debug, info, warn};
use clap::Parser;
use std::{
    io::{Result, ErrorKind, Error, copy, stdin},
    fs::File,
};

/// Run the complete `coveralls` workflow, as the command line binary does.
///
/// This is the single entry point that ties every stage together:
///
/// 1. parse the command line arguments,
/// 2. build a [`Config`] from the selected subcommand or, failing that, from the environment,
/// 3. read the coverage report from the input file or the standard input as a [`Coverage`],
/// 4. enrich the report and prune unwanted source files through a [`CoverallsManager`],
/// 5. when `--output` was passed, write the resulting payload to that file,
/// 6. unless `--no-send` was passed, upload the job to <https://coveralls.io>.
///
/// Logging is performed through the [`log`] crate, so initialize a logger (for instance
/// [`env_logger`](https://docs.rs/env_logger)) beforehand to see the progress messages.
///
/// # Errors
///
/// Returns an [`std::io::Error`] if no CI service could be determined, if the report cannot be read
/// or parsed, if the mandatory repository token is missing, or if the upload is rejected by the
/// Coveralls API.
pub fn work() -> Result<()> {
    let args = CliArgs::parse();
    let env = Env::new();

    run(args, env)
}

/// Run the workflow on already-resolved inputs.
///
/// This is the body of [`work`] with the command line arguments and the environment passed in
/// rather than read from the process, so the whole workflow can be driven from tests.
fn run(args: CliArgs, env: Env) -> Result<()> {
    let do_send = !args.no_send;

    debug!(
        "Arguments parsed (send: {do_send}, force git fetch: {})",
        args.force_fetch_git_infos
    );

    let config = {
        let config = match Config::load_from_command(&args, &env)? {
            Some(v) => Some(v),
            None => Config::load_from_environment(&env)?,
        };

        match config {
            Some(v) => v.init_parameters(&args),
            None => {
                return Err(Error::new(ErrorKind::Other, SimpleError::new("No service name found")));
            }
        }
    };

    info!("Using service `{}`", config.service.get_name());

    let manager = CoverallsManager::new();

    let mut coverage = if let Some(input) = &args.input {
        info!("Reading coverage report from file `{}`", input.display());

        Coverage::from_reader(File::open(input)?)?
    } else {
        info!("Reading coverage report from standard input");

        Coverage::from_reader(stdin())?
    };

    manager.apply_config(&config, &mut coverage, args.force_fetch_git_infos)?;
    config.show(coverage.git());

    if let Some(output) = &args.output {
        info!("Writing coverage payload to file `{}`", output.display());

        let mut reader = coverage.new_reader()?;
        let mut file = File::create(output)?;

        copy(&mut reader, &mut file)?;
    }

    if do_send {
        manager.send(&coverage)?;
    } else {
        warn!("Sending is disabled (--no-send): coverage will not be uploaded to Coveralls");
    }

    debug!("Coverage processing finished");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs, process};

    /// End-to-end dry run: read a report, enrich it and write the payload without uploading it.
    ///
    /// The Git metadata is fetched from the crate's own repository (the working directory during
    /// the tests), which is why the test relies on `cargo test` being run from a checkout.
    #[test]
    fn run_dry_run_writes_payload_without_sending() {
        let dir = std::env::temp_dir().join(format!("coveralls-work-test-{}", process::id()));
        fs::create_dir_all(&dir).expect("create the temporary directory");

        let input = dir.join("coverage.json");
        let output = dir.join("payload.json");

        fs::write(&input, r#"{"source_files":[]}"#).expect("write the coverage fixture");

        let token = "secret-token-1234567890";
        let args = CliArgs::try_parse_from([
            "coveralls",
            "--no-send",
            "--output",
            output.to_str().unwrap(),
            input.to_str().unwrap(),
            "circleci",
            "--repo-token",
            token,
        ])
        .expect("parse the command line arguments");

        let result = run(args, Env::new());

        // Clean up before asserting so a failure does not leave the temporary files behind.
        let payload = fs::read_to_string(&output);
        fs::remove_dir_all(&dir).ok();

        result.expect("the dry-run workflow should succeed");

        let json: serde_json::Value =
            serde_json::from_str(&payload.expect("the payload should have been written")).expect("a valid JSON payload");

        assert_eq!(json["service_name"], "circleci");
        assert_eq!(json["repo_token"], token);
    }
}
