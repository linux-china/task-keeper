use crate::command_utils::{run_command_line, CommandOutput};
use crate::errors::KeeperError;
use error_stack::{IntoReport, Report};
use std::collections::HashMap;
use which::which;

pub fn is_available() -> bool {
    std::env::current_dir()
        .map(|dir| dir.join("WORKSPACE").is_file() || dir.join("BUILD").is_file())
        .unwrap_or(false)
}

pub fn is_command_available() -> bool {
    which("bazel").is_ok()
}

pub fn get_task_command_map() -> HashMap<String, String> {
    let mut task_command_map = HashMap::new();
    task_command_map.insert("install".to_string(), "bazel fetch //...".to_string());
    task_command_map.insert("build".to_string(), "bazel build //...".to_string());
    task_command_map.insert(
        "deps".to_string(),
        "bazel query --notool_deps --noimplicit_deps 'deps(//...)'".to_string(),
    );
    task_command_map.insert("clean".to_string(), "bazel clean".to_string());
    task_command_map.insert("test".to_string(), "bazel test //...".to_string());
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
            "bazel".to_string()
        ).into_report())
    }
}
