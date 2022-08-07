pub mod fleet;
pub mod justfile;
pub mod packagejson;
pub mod denojson;
pub mod makefile;
pub mod rakefile;
pub mod taskspy;
pub mod taskfileyml;
pub mod makefiletoml;
pub mod procfile;
pub mod markdown;
pub mod taskshell;
pub mod composer;

use std::collections::HashMap;
use std::process::{Command, Output, Stdio};
use colored::Colorize;
use error_stack::{IntoReport, report, Result, ResultExt};
use crate::errors::KeeperError;
use which::which;

pub const RUNNERS: &'static [&'static str] = &["rake", "invoke", "task", "cargo-make", "just", "make", "proc", "npm", "deno", "composer", "shell", "fleet", "markdown"];

pub fn get_runner_file_name(runner: &str) -> &'static str {
    match runner {
        "rake" => "Rakefile",
        "invoke" => "tasks.py",
        "task" => "Taskfile.yml",
        "cargo-make" => "Makefile.toml",
        "just" => "Justfile",
        "make" => "Makefile",
        "proc" => "Procfile",
        "npm" => "package.json",
        "deno" => "deno.json",
        "composer" => "composer.json",
        "fleet" => ".fleet/run.json",
        "shell" => "task.sh",
        "markdown" => "README.md",
        _ => "unknown",
    }
}

pub fn is_command_available(command_name: &str) -> bool {
    which(command_name).is_ok()
}

pub fn run_command(command_name: &str, args: &[&str], verbose: bool) -> Result<Output, KeeperError> {
    run_command_with_env_vars(command_name, args, &None, verbose)
}

pub fn run_command_line(command_line: &str, verbose: bool) -> Result<Output, KeeperError> {
    let command_and_args = shlex::split(command_line).unwrap();
    // command line contains pipe or not
    if command_and_args.iter().any(|arg| arg == "|") {
        return run_command_by_shell(command_line, verbose);
    }
    let command_name = &command_and_args[0];
    let args: Vec<&str> = command_and_args[1..].iter().map(AsRef::as_ref).collect();
    if is_command_available(&command_name) {
        run_command(&command_name, &args, verbose)
    } else {
        println!("{}", format!("{} is not available to run '{}'", command_name, command_line).bold().red());
        Err(report!(KeeperError::CommandNotFound(command_name.to_string())))
    }
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

pub fn run_command_by_shell(command_line: &str, verbose: bool) -> Result<Output, KeeperError> {
    let mut command = if cfg!(target_os = "windows") {
        Command::new("cmd")
    } else {
        let shell_name = std::env::var("SHELL").unwrap_or("sh".to_owned());
        Command::new(&shell_name)
    };
    if cfg!(target_os = "windows") {
        command.args(["/C", command_line])
    } else {
        command.arg("-c").arg(command_line)
    };
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_pipe_line() {
        run_command_line("ls -al | wc -l", true).unwrap();
    }

    #[test]
    fn test_function_alias() {
        let command_name = "lss";
        if let Ok(path) = which(command_name) {
            println!("{:?}", path);
        } else {
            println!("{} is a function", command_name);
        }
    }
}
