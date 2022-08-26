mod cli;
mod commands;
mod error;
mod github;
mod operations;
mod runtime;

use clap::StructOpt;
use cli::Command;
use colored::Colorize;
use error::UserError;
use runtime::Outcome;
use std::env;
use std::process::ExitCode;

fn main() -> ExitCode {
    match inner() {
        Ok(_) => ExitCode::SUCCESS,
        Err(err) => {
            println!("\n{}{}\n", "Error: ".red().bold(), err.to_string().red());
            err.exit_code()
        }
    }
}

fn inner() -> Result<(), UserError> {
    let args = cli::Arguments::parse();
    let persisted_steps = runtime::load()?;
    let initial_dir = env::current_dir().expect("cannot determine the current directory");
    let current_steps = match args.command {
        Command::Abort => commands::abort(&persisted_steps)?,
        Command::Clone { org } => commands::clone(&org),
        Command::Exec { cmd, args } => commands::exec(&cmd, &args, &initial_dir)?,
        Command::Ignore => commands::ignore(persisted_steps)?,
        Command::Retry => commands::retry(persisted_steps)?,
    };
    let result = match runtime::execute(current_steps) {
        Outcome::Success => {
            runtime::forget()?;
            Ok(())
        }
        Outcome::StepFailed {
            exit_code,
            failed_step,
            remaining_steps,
        } => {
            runtime::persist(&remaining_steps)?;
            Err(UserError::StepFailed {
                step: failed_step,
                exit_code,
            })
        }
    };
    env::set_current_dir(initial_dir).expect("cannot return to initial directory");
    result
}
