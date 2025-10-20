use crate::command_utils::{run_command_line, CommandOutput};
use crate::errors::KeeperError;
use error_stack::{IntoReport, Report};
use std::collections::HashMap;
use which::which;

pub fn is_available() -> bool {
    std::env::current_dir()
        .map(|dir| dir.join("rebar.config").exists())
        .unwrap_or(false)
}

pub fn is_command_available() -> bool {
    which("rebar3").is_ok()
}

pub fn get_task_command_map() -> HashMap<String, String> {
    // https://rebar3.readme.io/docs/getting-started
    let mut task_command_map = HashMap::new();
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
            "rebar3".to_string()
        ).into_report())
    }
}
