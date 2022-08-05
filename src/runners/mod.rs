pub mod fleet;
pub mod justfile;
pub mod packagejson;
pub mod denojson;
pub mod makefile;

use std::collections::HashMap;
use std::process::{Command, Output, Stdio};
use error_stack::{IntoReport, Result, ResultExt};
use crate::errors::KeeperError;
use crate::{runners, task};

pub const RUNNERS: &'static [&'static str] = &["make", "npm", "deno", "just","fleet"];

pub fn run_command(command_name: &str, args: &[&str], verbose: bool) -> Result<Output, KeeperError> {
    run_command_with_env_vars(command_name, args, &None, verbose)
}

pub fn run_command_with_env_vars(command_name: &str, args: &[&str], env_vars: &Option<HashMap<String, String>>, verbose: bool) -> Result<Output, KeeperError> {
    let mut command = Command::new(command_name);
    if args.len() > 0 {
        command.args(args);
    }
    if let Some(vars) = env_vars {
        for (key, value) in vars {
            command.env(key, value);
        }
    }
    if verbose {
        println!("[tk] command line:  {:?}", command);
    }
    let output = command
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .report()
        .change_context(KeeperError::FailedToRunTasks(format!("{:?}", command)))?;
    Ok(output)
}

pub fn capture_command_output(command_name: &str, args: &[&str]) -> Result<Output, KeeperError> {
    let mut command = Command::new(command_name);
    if args.len() > 0 {
        command.args(args);
    }
    let output = command
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .report()
        .change_context(KeeperError::FailedToRunTasks(format!("{:?}", command)))?;
    Ok(output)
}
