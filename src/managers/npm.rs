use std::collections::HashMap;
use std::process::Output;
use error_stack::{report, Result};
use which::which;
use crate::command_utils::{run_command_line};
use crate::errors::KeeperError;

pub fn is_available() -> bool {
    std::env::current_dir()
        .map(|dir| dir.join("package.json").exists())
        .unwrap_or(false)
}

pub fn is_command_available() -> bool {
    which("npm").is_ok() || which("yarn").is_ok()
}

pub fn get_task_command_map() -> HashMap<String, String> {
    let mut task_command_map = HashMap::new();
    task_command_map.insert("init".to_string(), "npm init".to_string());
    task_command_map.insert("install".to_string(), "npm install".to_string());
    task_command_map.insert("compile".to_string(), "npm run compile".to_string());
    task_command_map.insert("build".to_string(), "npm run build".to_string());
    task_command_map.insert("start".to_string(), "npm run start".to_string());
    task_command_map.insert("test".to_string(), "npm run test".to_string());
    task_command_map.insert("deps".to_string(), "npm list".to_string());
    task_command_map.insert("doc".to_string(), "npm run doc".to_string());
    task_command_map.insert("clean".to_string(), "npm run clean".to_string());
    if which::which("npm-check").is_ok() {
        task_command_map.insert("outdated".to_string(), "npm-check -u".to_string());
    } else {
        task_command_map.insert("outdated".to_string(), "npm outdated".to_string());
    }
    task_command_map.insert("update".to_string(), "npm update".to_string());
    task_command_map
}

pub fn run_task(task: &str, _task_args: &[&str], _global_args: &[&str], verbose: bool) -> Result<Output, KeeperError> {
    if let Some(command_line) = get_task_command_map().get(task) {
        run_command_line(command_line, verbose)
    } else {
        Err(report!(KeeperError::ManagerTaskNotFound(task.to_owned(), "npm".to_string())))
    }
}
