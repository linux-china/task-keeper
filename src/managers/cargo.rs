use crate::command_utils::{run_command_line, CommandOutput};
use crate::errors::KeeperError;
use error_stack::{IntoReport, Report};
use std::collections::HashMap;
use which::which;
use crate::task;

pub fn is_available() -> bool {
    std::env::current_dir()
        .map(|dir| dir.join("Cargo.toml").exists())
        .unwrap_or(false)
}

pub fn is_command_available() -> bool {
    which("cargo").is_ok()
}

pub fn get_task_command_map() -> HashMap<String, String> {
    let mut task_command_map = HashMap::new();
    task_command_map.insert("install".to_string(), "cargo fetch".to_string());
    task_command_map.insert("compile".to_string(), "cargo build".to_string());
    task_command_map.insert("build".to_string(), "cargo build".to_string());
    task_command_map.insert("release".to_string(), "cargo build --release".to_string());
    task_command_map.insert("test".to_string(), "cargo test".to_string());
    task_command_map.insert("deps".to_string(), "cargo tree".to_string());
    task_command_map.insert("doc".to_string(), "cargo doc".to_string());
    task_command_map.insert("clean".to_string(), "cargo clean".to_string());
    task_command_map.insert("outdated".to_string(), "cargo outdated".to_string());
    task_command_map.insert("update".to_string(), "cargo update".to_string());
    let cargo_bin = std::env::current_dir()
        .map(|dir| dir.join("src").join("main.rs").exists())
        .unwrap_or(false);
    if cargo_bin {
        task_command_map.insert("start".to_string(), "cargo run".to_string());
    }
    task_command_map.insert("sbom".to_string(), "cargo-cyclonedx cyclonedx -v --format json".to_string());
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
            "cargo".to_string()
        ).into_report())
    }
}
