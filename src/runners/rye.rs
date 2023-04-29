use std::process::Output;
use crate::errors::KeeperError;
use crate::models::Task;
use crate::command_utils::{run_command, capture_command_output};
use crate::task;
use error_stack::{Result};
use which::which;
use crate::common::pyproject::PyProjectToml;
use crate::common::pyproject_toml_has_tool;

pub fn is_available() -> bool {
    std::env::current_dir()
        .map(|dir| dir.join("requirements.lock").exists() || pyproject_toml_has_tool("rye"))
        .unwrap_or(false)
}

pub fn is_command_available() -> bool {
    which("rye").is_ok()
}

pub fn list_tasks() -> Result<Vec<Task>, KeeperError> {
    let mut tasks = vec![];
    if let Ok(pyproject) = PyProjectToml::get_default_project() {
        if pyproject.rye_available() {
            if let Some(scripts) = pyproject.get_rye_scripts() {
                scripts.iter().for_each(|(name, description)| {
                    tasks.push(task!(name, "rye", description));
                });
            }
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
