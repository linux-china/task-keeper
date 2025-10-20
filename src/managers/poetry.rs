use crate::command_utils::{run_command_line, CommandOutput};
use crate::common::pyproject_toml_has_tool;
use crate::errors::KeeperError;
use error_stack::{IntoReport, Report};
use std::collections::HashMap;
use which::which;

pub fn is_available() -> bool {
    pyproject_toml_has_tool("poetry")
}

pub fn is_command_available() -> bool {
    which("poetry").is_ok()
}

pub fn get_task_command_map() -> HashMap<String, String> {
    let mut task_command_map = HashMap::new();
    task_command_map.insert("install".to_string(), "poetry install".to_string());
    task_command_map.insert("build".to_string(), "poetry build".to_string());
    task_command_map.insert("deps".to_string(), "poetry show --tree".to_string());
    task_command_map.insert(
        "clean".to_string(),
        "poetry cache clear --all .".to_string(),
    );
    task_command_map.insert("outdated".to_string(), "poetry show --outdated".to_string());
    task_command_map.insert("update".to_string(), "poetry update".to_string());
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
            "poetry".to_string()
        ).into_report())
    }
}
