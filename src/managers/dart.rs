use std::collections::HashMap;
use std::process::Output;
use error_stack::{report, Result};
use which::which;
use crate::command_utils::{run_command_line};
use crate::errors::KeeperError;

pub fn is_available() -> bool {
    std::env::current_dir()
        .map(|dir| dir.join("pubspec.yaml").exists())
        .unwrap_or(false)
}

pub fn is_command_available() -> bool {
    which("dart").is_ok()
}

pub fn get_task_command_map() -> HashMap<String, String> {
    let mut task_command_map = HashMap::new();
    task_command_map.insert("install".to_string(), "dart pub get".to_string());
    task_command_map.insert("deps".to_string(), "dart pub deps".to_string());
    task_command_map.insert("outdated".to_string(), "dart pub outdated".to_string());
    task_command_map.insert("update".to_string(), "dart pub upgrade".to_string());
    task_command_map
}

pub fn run_task(task: &str, _task_args: &[&str], _global_args: &[&str], verbose: bool) -> Result<Output, KeeperError> {
    if let Some(command_line) = get_task_command_map().get(task) {
        run_command_line(command_line, verbose)
    } else {
        Err(report!(KeeperError::ManagerTaskNotFound(task.to_owned(), "dart".to_string())))
    }
}
