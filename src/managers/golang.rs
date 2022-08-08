use std::collections::HashMap;
use std::process::Output;
use error_stack::{IntoReport, report, Result, ResultExt};
use which::which;
use crate::command_utils::run_command_line;
use crate::errors::KeeperError;

pub fn is_available() -> bool {
    std::env::current_dir()
        .map(|dir| dir.join("go.mod").exists())
        .unwrap_or(false)
}

pub fn is_command_available() -> bool {
    which("go").is_ok()
}

pub fn get_task_command_map() -> HashMap<String, String> {
    let mut task_command_map = HashMap::new();
    task_command_map.insert("init".to_string(), "go mod init {name}".to_string());
    task_command_map.insert("install".to_string(), "go get -u".to_string());
    task_command_map.insert("compile".to_string(), "go build".to_string());
    task_command_map.insert("build".to_string(), "go build".to_string());
    task_command_map.insert("start".to_string(), "go run main.go".to_string());
    task_command_map.insert("test".to_string(), "go test".to_string());
    task_command_map.insert("deps".to_string(), "go list -m all".to_string());
    task_command_map.insert("doc".to_string(), "go doc".to_string());
    task_command_map.insert("clean".to_string(), "go clean".to_string());
    task_command_map.insert("outdated".to_string(), "go list -u -m all".to_string());
    task_command_map.insert("update".to_string(), "go get -u".to_string());
    task_command_map
}

pub fn run_task(task: &str, extra_args: &[&str], verbose: bool) -> Result<Output, KeeperError> {
    if let Some(command_line) = get_task_command_map().get(task) {
        run_command_line(command_line, verbose)
    } else {
        Err(report!(KeeperError::ManagerTaskNotFound(task.to_owned(), "go".to_string())))
    }
}
