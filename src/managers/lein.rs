use crate::command_utils::{run_command_line, CommandOutput};
use crate::errors::KeeperError;
use error_stack::{IntoReport, Report};
use std::collections::HashMap;
use which::which;

pub fn is_available() -> bool {
    std::env::current_dir()
        .map(|dir| dir.join("project.clj").exists())
        .unwrap_or(false)
}

pub fn is_command_available() -> bool {
    which("lein").is_ok()
}

pub fn get_task_command_map() -> HashMap<String, String> {
    let mut task_command_map = HashMap::new();
    task_command_map.insert("install".to_string(), "lein deps".to_string());
    task_command_map.insert("compile".to_string(), "lein compile".to_string());
    task_command_map.insert("build".to_string(), "lein uberjar".to_string());
    task_command_map.insert("start".to_string(), "lein run".to_string());
    task_command_map.insert("test".to_string(), "lein test".to_string());
    task_command_map.insert("deps".to_string(), "lein deps :tree".to_string());
    task_command_map.insert("clean".to_string(), "lein do clean".to_string());
    task_command_map.insert("outdated".to_string(), "lein outdated".to_string());
    task_command_map.insert("update".to_string(), "lein outdated --upgrade".to_string());
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
            "lein".to_string()
        ).into_report())
    }
}
