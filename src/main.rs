use coveralls::{cli_args, CoverallsManager, Coverage, Env, Config};
use simple_error::SimpleError;
use std::{io::{Result, Error, stdin}, process::exit, fs::File};
use std::io::ErrorKind;

fn work() -> Result<()> {
    let args = cli_args::make_args();
    let env = Env::new();
    let config = {
        let mut config = None;

        if let Some(cmd_name) = args.subcommand_name() {
            if let Some(args) = args.subcommand_matches(cmd_name) {
                config.replace(Config::load_from_command(cmd_name, args, &env)?);
            }
        }

        if config.is_none() {
            config = Config::load_from_environment(&env)?;
        }

        match config {
            Some(v) => v.init_parameters(&args),
            None => {
                return Err(Error::new(ErrorKind::Other, SimpleError::new("No service name found")));
            }
        }
    };

    let manager = CoverallsManager::new();

    let mut coverage = if let Some(input) = args.value_of("input") {
        Coverage::from_reader(File::open(input)?)?
    } else {
        Coverage::from_reader(stdin())?
    };

    config.show();
    manager.apply_config(&config, &mut coverage)?;
    manager.send(&coverage)
}

fn main() {
    if let Err(err) = work() {
        eprintln!("{}", err);
        exit(1);
    }
}
