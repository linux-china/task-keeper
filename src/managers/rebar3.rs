use std::collections::HashMap;
use std::process::Output;
use error_stack::{report, Result};
use which::which;
use crate::command_utils::{run_command_line};
use crate::errors::KeeperError;

pub fn is_available() -> bool {
    std::env::current_dir()
        .map(|dir| dir.join("rebar.config").exists())
        .unwrap_or(false)
}

pub fn is_command_available() -> bool {
    which("rebar3").is_ok()
}

pub fn get_task_command_map() -> HashMap<String, String> {
    let mut task_command_map = HashMap::new();
    task_command_map.insert("init".to_string(), "rebar3 new app {name}".to_string());
    task_command_map.insert("install".to_string(), "rebar3 get-deps".to_string());
    task_command_map.insert("compile".to_string(), "rebar3 compile".to_string());
    task_command_map.insert("build".to_string(), "rebar3 release".to_string());
    task_command_map.insert("deps".to_string(), "rebar3 tree".to_string());
    task_command_map.insert("doc".to_string(), "rebar3 edoc".to_string());
    task_command_map.insert("clean".to_string(), "rebar3 clean".to_string());
    task_command_map.insert("test".to_string(), "rebar3 eunit".to_string());
    task_command_map.insert("update".to_string(), "rebar3 upgrade --all".to_string());
    task_command_map
}

pub fn run_task(task: &str, _task_args: &[&str], _global_args: &[&str], verbose: bool) -> Result<Output, KeeperError> {
    if let Some(command_line) = get_task_command_map().get(task) {
        run_command_line(command_line, verbose)
    } else {
        Err(report!(KeeperError::ManagerTaskNotFound(task.to_owned(), "rebar3".to_string())))
    }
}
