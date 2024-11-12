use std::collections::HashMap;
use std::process::Output;
use error_stack::{report, Result};
use which::which;
use crate::command_utils::run_command_line;
use crate::errors::KeeperError;

pub fn is_available() -> bool {
    std::env::current_dir()
        .map(|dir| dir.join("build.zig").exists())
        .unwrap_or(false)
}

pub fn is_command_available() -> bool {
    which("zig").is_ok()
}

pub fn get_task_command_map() -> HashMap<String, String> {
    let mut task_command_map = HashMap::new();
    task_command_map.insert("install".to_string(), "zig build install".to_string());
    task_command_map.insert("compile".to_string(), "zig build".to_string());
    task_command_map.insert("build".to_string(), "zig build".to_string());
    task_command_map.insert("release".to_string(), "zig build -Drelease-safe".to_string());
    task_command_map.insert("start".to_string(), "zig build run".to_string());
    task_command_map.insert("test".to_string(), "zig build test".to_string());
    task_command_map.insert("clean".to_string(), "zig build uninstall".to_string());
    task_command_map
}

pub fn run_task(task: &str, _task_args: &[&str], _global_args: &[&str], verbose: bool) -> Result<Output, KeeperError> {
    if let Some(command_line) = get_task_command_map().get(task) {
        run_command_line(command_line, verbose)
    } else {
        Err(report!(KeeperError::ManagerTaskNotFound(task.to_owned(), "zig".to_string())))
    }
}
