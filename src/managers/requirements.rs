use std::collections::HashMap;
use std::process::Output;
use error_stack::{report, Result};
use which::which;
use crate::command_utils::run_command_line;
use crate::errors::KeeperError;

pub fn is_available() -> bool {
    std::env::current_dir()
        .map(|dir| dir.join("requirements.txt").exists()
            && !dir.join("pyproject.toml").exists()
            && !dir.join("Pipfile").exists()
        )
        .unwrap_or(false)
}

pub fn is_command_available() -> bool {
    which("pip").is_ok()
}

pub fn get_task_command_map() -> HashMap<String, String> {
    let mut task_command_map = HashMap::new();
    let mut command = "pip".to_owned();
    let uv_env = std::env::current_dir()
        .map(|dir| dir.join("requirements.txt").exists()
            && dir.join(".venv").exists()
        ).unwrap_or(false);
    if uv_env && which("uv").is_ok() {
        command = "uv pip".to_string();
    }
    task_command_map.insert("install".to_string(), format!("{} install -r requirements.txt", command));
    task_command_map.insert("deps".to_string(), "deptree".to_string());
    task_command_map.insert("outdated".to_string(), "pip list --outdated".to_string());
    task_command_map.insert("update".to_string(), format!("{}  install -U -r requirements.txt", command));
    task_command_map
}

pub fn run_task(task: &str, _task_args: &[&str], _global_args: &[&str], verbose: bool) -> Result<Output, KeeperError> {
    if let Some(command_line) = get_task_command_map().get(task) {
        run_command_line(command_line, verbose)
    } else {
        Err(report!(KeeperError::ManagerTaskNotFound(task.to_owned(), "pip".to_string())))
    }
}
