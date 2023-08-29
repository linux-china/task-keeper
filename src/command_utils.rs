use std::collections::HashMap;
use std::io::Write;
use std::process::{Command, Output, Stdio};
use colored::Colorize;
use error_stack::{IntoReport, report, Result, ResultExt};
use crate::errors::KeeperError;
use which::which;

pub fn is_command_available(command_name: &str) -> bool {
    which(command_name).is_ok()
}

pub fn run_command(command_name: &str, args: &[&str], verbose: bool) -> Result<Output, KeeperError> {
    run_command_with_env_vars(command_name, args, &None, &None, verbose)
}

pub fn run_command_line(command_line: &str, verbose: bool) -> Result<Output, KeeperError> {
    let command_and_args = shlex::split(command_line).unwrap();
    // command line contains pipe or not
    if command_and_args.iter().any(|arg| arg == "|" || arg == "|&" || arg == ">" || arg == ">>") {
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

pub fn run_command_line_from_stdin(command_line: &str, input: &str, verbose: bool) -> Result<Output, KeeperError> {
    let command_and_args = shlex::split(command_line).unwrap();
    let command_name = &command_and_args[0];
    let args: Vec<&str> = if command_and_args.len() > 1 {
        command_and_args[1..].iter().map(AsRef::as_ref).collect()
    } else {
        vec![]
    };
    if verbose {
        println!("[tk] command line:  {:?}", command_line);
    }
    if is_command_available(&command_name) {
        let mut child = Command::new(command_name)
            .args(&args)
            .envs(std::env::vars())
            .stdin(Stdio::piped())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .into_report()
            .change_context(KeeperError::FailedToRunTasks(format!("{:?}", command_name)))?;
        child.stdin
            .as_mut()
            .ok_or("Child process stdin has not been captured!").unwrap()
            .write_all(input.as_bytes()).unwrap();
        child.wait_with_output().into_report()
            .change_context(KeeperError::FailedToRunTasks(format!("{:?}", command_name)))
    } else {
        println!("{}", format!("{} is not available to run '{}'", command_name, command_line).bold().red());
        Err(report!(KeeperError::CommandNotFound(command_name.to_string())))
    }
}

pub fn run_command_with_env_vars(command_name: &str, args: &[&str], working_dir: &Option<String>, env_vars: &Option<HashMap<String, String>>, verbose: bool) -> Result<Output, KeeperError> {
    let mut command = Command::new(command_name);
    if args.len() > 0 {
        command.args(args);
    }
    if let Some(current_dir) = working_dir {
        command.current_dir(current_dir);
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
        .envs(std::env::vars())
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .into_report()
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
        .envs(std::env::vars())
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .into_report()
        .change_context(KeeperError::FailedToRunTasks(format!("{:?}", command)))?;
    Ok(output)
}

pub fn capture_command_output(command_name: &str, args: &[&str]) -> Result<Output, KeeperError> {
    let mut command = Command::new(command_name);
    if args.len() > 0 {
        command.args(args);
    }
    let output = command
        .envs(std::env::vars())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .into_report()
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
    fn test_run_command_line_from_stdin() {
        run_command_line_from_stdin("deno run -", "console.log('hello world')", true).unwrap();
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
