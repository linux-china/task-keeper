use std::process::Output;
use crate::errors::KeeperError;
use crate::models::Task;
use crate::command_utils::{run_command, capture_command_output};
use crate::task;
use error_stack::{Result};
use which::which;

pub fn is_available() -> bool {
    std::env::current_dir()
        .map(|dir| dir.join("requirements.lock").exists())
        .unwrap_or(false)
}

pub fn is_command_available() -> bool {
    which("rye").is_ok()
}

pub fn list_tasks() -> Result<Vec<Task>, KeeperError> {
    let tasks_text = capture_command_output("rye", &["run", "-l"])
        .map(|output| {
            String::from_utf8(output.stdout).unwrap_or("".to_owned())
        })?;
    let mut tasks = vec![];
    for line in tasks_text.lines() {
        if line.contains("(") && line.ends_with(")") {
            let offset = line.find("(").unwrap();
            let name = line[..offset].trim();
            let description = line[offset + 1..line.len() - 1].trim();
            tasks.push(task!(name, "rye", description));
        }
    }
    Ok(tasks)
}

pub fn run_task(task: &str, task_args: &[&str], global_args: &[&str], verbose: bool) -> Result<Output, KeeperError> {
    let mut args = vec![];
    args.extend(global_args);
    args.push("run");
    args.push(task);
    args.extend(task_args);
    run_command("rye", &args, verbose)
}
