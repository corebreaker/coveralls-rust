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
