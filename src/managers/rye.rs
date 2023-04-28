use std::collections::HashMap;
use std::process::Output;
use error_stack::{report, Result};
use which::which;
use crate::command_utils::run_command_line;
use crate::common::pyproject_toml_has_tool;
use crate::errors::KeeperError;

pub fn is_available() -> bool {
    std::env::current_dir()
        .map(|dir| dir.join("requirements.lock").exists() || pyproject_toml_has_tool("rye"))
        .unwrap_or(false)
}

pub fn is_command_available() -> bool {
    which("rye").is_ok()
}

pub fn get_task_command_map() -> HashMap<String, String> {
    let mut task_command_map = HashMap::new();
    task_command_map.insert("install".to_string(), "rye sync".to_string());
    task_command_map.insert("deps".to_string(), "rye show --installed-deps".to_string());
    task_command_map.insert("outdated".to_string(), "rye run pip3 list --outdated".to_string());
    task_command_map.insert("update".to_string(), "rye sync --update-all".to_string());
    task_command_map.insert("build".to_string(), "rye build".to_string());
    task_command_map
}

pub fn run_task(task: &str, _task_args: &[&str], _global_args: &[&str], verbose: bool) -> Result<Output, KeeperError> {
    if let Some(command_line) = get_task_command_map().get(task) {
        run_command_line(command_line, verbose)
    } else {
        Err(report!(KeeperError::ManagerTaskNotFound(task.to_owned(), "rye".to_string())))
    }
}
