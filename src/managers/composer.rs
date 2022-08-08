use std::collections::HashMap;
use std::process::Output;
use error_stack::{IntoReport, report, Result, ResultExt};
use which::which;
use crate::command_utils::{run_command_line};
use crate::errors::KeeperError;

pub fn is_available() -> bool {
    std::env::current_dir()
        .map(|dir| dir.join("composer.json").exists())
        .unwrap_or(false)
}

pub fn is_command_available() -> bool {
    which("composer").is_ok()
}

pub fn get_task_command_map() -> HashMap<String, String> {
    let mut task_command_map = HashMap::new();
    task_command_map.insert("init".to_string(), "composer init".to_string());
    task_command_map.insert("install".to_string(), "composer install".to_string());
    task_command_map.insert("compile".to_string(), "composer check-platform-reqs".to_string());
    task_command_map.insert("build".to_string(), "composer run-script build".to_string());
    task_command_map.insert("start".to_string(), "composer run-script start".to_string());
    task_command_map.insert("test".to_string(), "composer run-script test".to_string());
    task_command_map.insert("deps".to_string(), "composer depends".to_string());
    task_command_map.insert("doc".to_string(), "composer doc".to_string());
    task_command_map.insert("clean".to_string(), "composer clear-cache".to_string());
    task_command_map.insert("outdated".to_string(), "composer outdated".to_string());
    task_command_map.insert("update".to_string(), "composer update".to_string());
    task_command_map
}

pub fn run_task(task: &str, extra_args: &[&str], verbose: bool) -> Result<Output, KeeperError> {
    if let Some(command_line) = get_task_command_map().get(task) {
        run_command_line(command_line, verbose)
    } else {
        Err(report!(KeeperError::ManagerTaskNotFound(task.to_owned(), "composer".to_string())))
    }
}
