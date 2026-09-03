use crate::command_utils::{capture_command_output, run_command, CommandOutput};
use crate::errors::KeeperError;
use crate::models::Task;
use crate::task;
use std::io::{BufRead, BufReader};
use error_stack::Report;

pub fn is_available() -> bool {
    std::env::current_dir()
        .map(|dir| dir.join("Cargo.toml").exists() && dir.join("xtask").exists())
        .unwrap_or(false)
}

pub fn list_tasks() -> Result<Vec<Task>, Report<KeeperError>> {
    let tasks_text = capture_command_output("cargo", &["-q", "xtask", "--help"])
        .map(|output| String::from_utf8(output.stdout).unwrap_or("".to_owned()))?;
    let tasks: Vec<Task> = BufReader::new(tasks_text.as_bytes())
        .lines()
        .filter(|line| line.is_ok() && line.as_ref().unwrap().starts_with("  "))
        .map(|line| line.unwrap().trim().to_string())
        .map(|line| {
            let offset = line.find(' ').unwrap_or(0);
            if offset > 0 {
                let task_name = line[..offset].trim();
                let mut description = line[offset + 1..].trim();
                if description.starts_with('-') {
                    description = description[1..].trim();
                }
                task!(task_name, "xtask", description)
            } else {
                task!(line, "xtask")
            }
        })
        .collect::<Vec<Task>>();
    Ok(tasks)
}

pub fn run_task(
    task: &str,
    task_args: &[&str],
    global_args: &[&str],
    verbose: bool,
) -> Result<CommandOutput, Report<KeeperError>> {
    let mut args = vec![];
    args.push("-q");
    args.push("xtask");
    args.extend(global_args);
    args.push(task);
    args.extend(task_args);
    run_command("cargo", &args, verbose)
}
