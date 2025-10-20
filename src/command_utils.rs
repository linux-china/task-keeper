use crate::errors::KeeperError;
use colored::Colorize;
use error_stack::{IntoReport, Report, ResultExt};
use std::collections::HashMap;
use std::io;
use std::io::{Read, Write};
use std::process::{Command, ExitStatus, Output, Stdio};
use which::which;

pub struct CommandOutput {
    pub status: ExitStatus,
    pub stdout: Option<String>,
    pub stderr: Option<String>,
}

impl CommandOutput {
    pub fn from(output: Output) -> Self {
        CommandOutput {
            status: output.status,
            stdout: if output.stdout.len() == 0 {
                None
            } else {
                String::from_utf8(output.stdout).ok()
            },
            stderr: if output.stderr.len() == 0 {
                None
            } else {
                String::from_utf8(output.stderr).ok()
            },
        }
    }
}

pub fn is_command_available(command_name: &str) -> bool {
    which(command_name).is_ok()
}

pub fn run_command(
    command_name: &str,
    args: &[&str],
    verbose: bool,
) -> Result<CommandOutput, Report<KeeperError>> {
    run_command_with_env_vars(command_name, args, &None, &None, verbose)
}

pub fn run_command_line(command_line: &str, verbose: bool) -> Result<CommandOutput, Report<KeeperError>> {
    let command_and_args = shlex::split(command_line).unwrap();
    // command line contains pipe or not
    if command_and_args
        .iter()
        .any(|arg| arg == "|" || arg == "|&" || arg == ">" || arg == ">>")
    {
        return run_command_by_shell(command_line, verbose);
    }
    let command_name = &command_and_args[0];
    let args: Vec<&str> = command_and_args[1..].iter().map(AsRef::as_ref).collect();
    if is_command_available(&command_name) {
        run_command(&command_name, &args, verbose)
    } else {
        println!(
            "{}",
            format!(
                "{} is not available to run '{}'",
                command_name, command_line
            )
            .bold()
            .red()
        );
        Err(KeeperError::CommandNotFound(command_name.to_string()).into_report())
    }
}

pub fn run_command_line_from_stdin(
    command_line: &str,
    input: &str,
    verbose: bool,
) -> Result<CommandOutput, Report<KeeperError>> {
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
            .change_context(KeeperError::FailedToRunTasks(format!("{:?}", command_name)))?;
        child
            .stdin
            .as_mut()
            .ok_or("Child process stdin has not been captured!")
            .unwrap()
            .write_all(input.as_bytes())
            .unwrap();
        child
            .wait_with_output()
            .map(CommandOutput::from)
            .change_context(KeeperError::FailedToRunTasks(format!("{:?}", command_name)))
    } else {
        println!(
            "{}",
            format!(
                "{} is not available to run '{}'",
                command_name, command_line
            )
            .bold()
            .red()
        );
        Err(KeeperError::CommandNotFound(command_name.to_string()).into_report())
    }
}

pub fn run_command_with_env_vars(
    command_name: &str,
    args: &[&str],
    working_dir: &Option<String>,
    env_vars: &Option<HashMap<String, String>>,
    verbose: bool,
) -> Result<CommandOutput, Report<KeeperError>> {
    let mut command = Command::new(command_name);
    command.envs(std::env::vars());
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
    if std::env::var("TK_TASK_NAME").is_ok() {
        return intercept_output(&mut command);
    }
    command
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .map(CommandOutput::from)
        .change_context(KeeperError::FailedToRunTasks(format!("{:?}", command)))
}

pub fn run_command_by_shell(
    command_line: &str,
    verbose: bool,
) -> Result<CommandOutput, Report<KeeperError>> {
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
    command
        .envs(std::env::vars())
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .map(CommandOutput::from)
        .change_context(KeeperError::FailedToRunTasks(format!("{:?}", command)))
}

pub fn intercept_output(command: &mut Command) -> Result<CommandOutput, Report<KeeperError>> {
    let mut child = command
        .stdin(Stdio::inherit())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .change_context(KeeperError::FailedToRunTasks(format!("{:?}", command)))?;
    // Create threads to handle both streams
    let mut stdout = child.stdout.take().unwrap();
    let mut stderr = child.stderr.take().unwrap();

    let stdout_thread = std::thread::spawn(move || {
        let mut stdout_bytes = Vec::new();
        let mut buffer = [0; 32];
        while let Ok(n) = stdout.read(&mut buffer) {
            if n == 0 {
                break;
            }
            // Print to console
            let content = &buffer[..n];
            io::stdout().write_all(content).unwrap();
            // Collect output
            stdout_bytes.extend_from_slice(content);
        }
        String::from_utf8_lossy(&stdout_bytes).to_string()
    });

    let stderr_thread = std::thread::spawn(move || {
        let mut stderr_bytes = Vec::new();
        let mut buffer = [0; 32];
        while let Ok(n) = stderr.read(&mut buffer) {
            if n == 0 {
                break;
            }
            // Print to console
            let content = &buffer[..n];
            io::stderr().write_all(content).unwrap();
            // You can also process the error output here
            stderr_bytes.extend_from_slice(content);
        }
        String::from_utf8_lossy(&stderr_bytes).to_string()
    });

    let output = stdout_thread.join().unwrap();
    let error = stderr_thread.join().unwrap();

    let status = child.wait().unwrap();
    Ok(CommandOutput {
        status,
        stdout: if output.is_empty() {
            None
        } else {
            Some(output)
        },
        stderr: if error.is_empty() { None } else { Some(error) },
    })
}

pub fn capture_command_output(command_name: &str, args: &[&str]) -> Result<Output, Report<KeeperError>> {
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

    #[test]
    fn test_intercept_output() {
        let mut command = Command::new("java");
        command.args(["-version"]).envs(std::env::vars());
        let output = intercept_output(&mut command).unwrap();
        println!("{}", output.stderr.unwrap())
    }
}
