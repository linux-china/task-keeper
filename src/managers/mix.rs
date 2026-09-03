use crate::command_utils::{run_command_line, CommandOutput};
use crate::errors::KeeperError;
use error_stack::{IntoReport, Report};
use std::collections::HashMap;
use which::which;

pub fn is_available() -> bool {
    std::env::current_dir()
        .map(|dir| dir.join("mix.exs").exists())
        .unwrap_or(false)
}

pub fn is_command_available() -> bool {
    which("mix").is_ok()
}

pub fn get_task_command_map() -> HashMap<String, String> {
    // https://hexdocs.pm/mix/1.13/Mix.html
    // https://hexdocs.pm/hex/Mix.Tasks.Hex.html
    let mut task_command_map = HashMap::new();
    task_command_map.insert("install".to_string(), "mix deps.get".to_string());
    task_command_map.insert("compile".to_string(), "mix compile".to_string());
    task_command_map.insert("build".to_string(), "mix hex.build".to_string());
    task_command_map.insert("deps".to_string(), "mix deps.tree".to_string());
    task_command_map.insert("doc".to_string(), "mix docs".to_string());
    task_command_map.insert("clean".to_string(), "mix clean".to_string());
    task_command_map.insert("start".to_string(), "mix run".to_string());
    task_command_map.insert("test".to_string(), "mix test".to_string());
    task_command_map.insert("outdated".to_string(), "mix hex.outdated".to_string());
    task_command_map.insert("update".to_string(), "mix deps.update --all".to_string());
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
            "mix".to_string()
        ).into_report())
    }
}
