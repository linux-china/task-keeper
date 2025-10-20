use crate::command_utils::{run_command_line, CommandOutput};
use crate::errors::KeeperError;
use error_stack::{IntoReport, Report};
use std::collections::HashMap;
use which::which;

pub fn is_available() -> bool {
    std::env::current_dir()
        .map(|dir| dir.join("bld").exists() || dir.join("bld.bat").exists())
        .unwrap_or(false)
}

pub fn is_command_available() -> bool {
    which("./bld").is_ok() || which("./bld.bat").is_ok()
}

pub fn get_task_command_map() -> HashMap<String, String> {
    let mut task_command_map = HashMap::new();
    task_command_map.insert("install".to_string(), "./bld download".to_owned());
    task_command_map.insert("compile".to_string(), "./bld compile".to_owned());
    task_command_map.insert("build".to_string(), "./bld jar".to_owned());
    task_command_map.insert("release".to_string(), "./bld uberjar".to_owned());
    task_command_map.insert("start".to_string(), "./bld run".to_owned());
    task_command_map.insert("test".to_string(), "./bld test".to_owned());
    task_command_map.insert("deps".to_string(), "./bld dependency-tree".to_owned());
    task_command_map.insert("outdated".to_string(), "./bld updates".to_owned());
    task_command_map.insert("clean".to_string(), "./bld clean".to_owned());
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
            "bld".to_string()
        ).into_report())
    }
}
