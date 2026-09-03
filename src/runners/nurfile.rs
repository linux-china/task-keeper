use crate::command_utils::{capture_command_output, run_command, CommandOutput};
use crate::errors::KeeperError;
use crate::models::Task;
use crate::task;
use serde::{Deserialize, Serialize};
use std::env;
use std::io::{BufRead, BufReader};
use error_stack::Report;
use which::which;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArgcfileJson {
    pub subcommands: Vec<ArgcSubCommand>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArgcSubCommand {
    pub name: String,
    pub describe: Option<String>,
}

pub fn is_available() -> bool {
    env::current_dir()
        .map(|dir| dir.join("nurfile").exists())
        .unwrap_or(false)
}

pub fn is_command_available() -> bool {
    which("nur").is_ok()
}

pub fn list_tasks() -> Result<Vec<Task>, Report<KeeperError>> {
    let nur_list = capture_command_output("nur", &["--quiet", "--list"])
        .map(|output| String::from_utf8(output.stdout).unwrap_or("".to_owned()))?;
    let tasks: Vec<Task> = BufReader::new(nur_list.as_bytes())
        .lines()
        .map(|line| line.unwrap())
        .filter(|line| !line.is_empty())
        .map(|line| {
            let name = line.trim();
            let command = format!("nur {}", name);
            task!(name, "nur", command)
        })
        .collect();
    Ok(tasks)
}

pub fn run_task(
    task: &str,
    task_args: &[&str],
    global_args: &[&str],
    verbose: bool,
) -> Result<CommandOutput, Report<KeeperError>> {
    let mut args = vec![];
    args.extend(global_args);
    args.push("--quiet");
    args.push(task);
    args.extend(task_args);
    let tk_args = env::args().skip(2).collect::<Vec<String>>();
    args.extend(tk_args.iter().map(|s| s.as_str()));
    run_command("nur", &args, verbose)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        if let Ok(tasks) = list_tasks() {
            println!("{:?}", tasks);
        }
    }

    #[test]
    fn test_run() {
        if let Ok(output) = run_task("build1", &[], &[], true) {
            let status_code = output.status.code().unwrap_or(0);
            println!("exit code: {}", status_code);
        }
    }
}
