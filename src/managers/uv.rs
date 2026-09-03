use crate::command_utils::{run_command_line, CommandOutput};
use crate::common::pyproject_toml_has_tool;
use crate::errors::KeeperError;
use error_stack::{IntoReport, Report};
use std::collections::HashMap;
use which::which;

pub fn is_available() -> bool {
    std::env::current_dir()
        .map(|dir| {
            dir.join("uv.lock").exists()
                || dir.join("uv.toml").exists()
                || pyproject_toml_has_tool("uv")
        })
        .unwrap_or(false)
}

pub fn is_command_available() -> bool {
    which("uv").is_ok()
}

pub fn get_task_command_map() -> HashMap<String, String> {
    let mut task_command_map = HashMap::new();
    task_command_map.insert("install".to_string(), "uv sync".to_string());
    task_command_map.insert("deps".to_string(), "uv tree".to_string());
    task_command_map.insert("outdated".to_string(), "uv pip list --outdated".to_string());
    task_command_map.insert("update".to_string(), "uv sync -U".to_string());
    task_command_map.insert(
        "build".to_string(),
        "uvx --from build pyproject-build --installer uv".to_string(),
    );
    task_command_map.insert(
        "sbom".to_string(),
        "uvx --from cyclonedx-bom cyclonedx-py environment .venv -o application.cdx.json"
            .to_string(),
    );
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
        Err(KeeperError::ManagerTaskNotFound(task.to_owned(), "uv".to_string()).into_report())
    }
}
