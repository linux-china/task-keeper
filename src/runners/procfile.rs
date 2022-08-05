use std::io::{BufRead, BufReader};
use std::process::{Output, Termination};
use clap::arg;
use colored::Colorize;
use error_stack::{IntoReport, report, Result, ResultExt};
use crate::errors::KeeperError;
use crate::models::Task;
use crate::runners::{is_command_available, run_command};
use crate::task;
use which::which;

pub fn is_available() -> bool {
    std::env::current_dir()
        .map(|dir| dir.join("Procfile").exists())
        .unwrap_or(false)
}

pub fn list_tasks() -> Result<Vec<Task>, KeeperError> {
    let procfile_text = std::env::current_dir()
        .map(|dir| dir.join("Procfile"))
        .map(|path| std::fs::read_to_string(path).unwrap())
        .report()
        .change_context(KeeperError::InvalidProcfile)?;
    let tasks: Vec<Task> = BufReader::new(procfile_text.as_bytes())
        .lines()
        .filter(|line| line.is_ok() && line.as_ref().unwrap().contains(":"))
        .map(|line| line.unwrap())
        .map(|line| {
            let mut parts = line.splitn(2, ':');
            let name = parts.next().unwrap().trim();
            let command = parts.next().unwrap().trim();
            task!(name, "proc", command)
        })
        .collect();
    Ok(tasks)
}

pub fn run_task(task: &str, _extra_args: &[&str], verbose: bool) -> Result<Output, KeeperError> {
    let tasks = list_tasks()?;
    let task = tasks.iter().find(|t| t.name == task).ok_or_else(|| {
        KeeperError::TaskNotFound(task.to_string())
    })?;
    let command_and_args = shlex::split(&task.description).unwrap();
    let command_name = &command_and_args[0];
    let args: Vec<&str> = command_and_args[1..].iter().map(AsRef::as_ref).collect();
    if is_command_available(&command_name) {
        run_command(&command_name, &args, verbose)
    } else {
        println!("{}", format!("{} is not available to run '{}'", command_name, task.description).bold().red());
        Err(report!(KeeperError::CommandNotFound(command_name.clone())))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command() {
        let args = shlex::split("java -jar demo.jar \"demo dh\"").unwrap();
        println!("{:?}", args);
    }

    #[test]
    fn test_parse() {
        if let Ok(tasks) = list_tasks() {
            println!("{:?}", tasks);
        }
    }

    #[test]
    fn test_run() {
        if let Ok(output) = run_task("my-ip", &[], true) {
            let status_code = output.status.code().unwrap_or(0);
            println!("exit code: {}", status_code);
        }
    }
}
