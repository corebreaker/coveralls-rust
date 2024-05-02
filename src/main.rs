use coveralls::{cli_args::CliArgs, CoverallsManager, Coverage, Env, Config};
use simple_error::SimpleError;
use clap::Parser;
use std::{io::{Result, ErrorKind, Error, stdin}, process::exit, fs::File};

fn work() -> Result<()> {
    let args = CliArgs::parse();
    let env = Env::new();
    let do_send = !args.no_send;
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

    let manager = CoverallsManager::new();

    let mut coverage = if let Some(input) = &args.input {
        Coverage::from_reader(File::open(input)?)?
    } else {
        Coverage::from_reader(stdin())?
    };

    config.show();
    manager.apply_config(&config, &mut coverage)?;

    if do_send {
        manager.send(&coverage)?;
    }

    Ok(())
}

fn main() {
    if let Err(err) = work() {
        eprintln!("{}", err);
        exit(1);
    }
}
