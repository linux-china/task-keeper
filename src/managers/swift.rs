use crate::command_utils::{run_command_line, CommandOutput};
use crate::errors::KeeperError;
use error_stack::{IntoReport, Report};
use std::collections::HashMap;
use which::which;

pub fn is_available() -> bool {
    std::env::current_dir()
        .map(|dir| dir.join("Package.swift").exists())
        .unwrap_or(false)
}

pub fn is_command_available() -> bool {
    which("swift").is_ok()
}

pub fn get_task_command_map() -> HashMap<String, String> {
    let mut task_command_map = HashMap::new();
    task_command_map.insert("build".to_string(), "swift build".to_string());
    task_command_map.insert("release".to_string(), "swift build -c release".to_string());
    task_command_map.insert("start".to_string(), "swift run".to_string());
    task_command_map.insert("test".to_string(), "swift test".to_string());
    task_command_map.insert(
        "deps".to_string(),
        "swift package show-dependencies".to_string(),
    );
    task_command_map.insert("doc".to_string(), "bundle open".to_string());
    task_command_map.insert("clean".to_string(), "swift package clean".to_string());
    // swift-outdated
    if which::which("swift-outdated").is_ok() {
        task_command_map.insert("outdated".to_string(), "swift outdated".to_string());
    }
    task_command_map.insert("update".to_string(), "swift package update".to_string());
    task_command_map
}

pub fn run_task(
    task: &str,
    _task_args: &[&str],
    _global_args: &[&str],
    verbose: bool,
) -> Result<CommandOutput, Report<KeeperError>> {
    if let Some(command_line) = get_task_command_map().get(task) {
        run_command_line(command_line, verbose)
    } else {
        Err(KeeperError::ManagerTaskNotFound(
            task.to_owned(),
            "swift".to_string()
        ).into_report())
    }
}
