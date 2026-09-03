use crate::command_utils::{run_command_line, CommandOutput};
use crate::errors::KeeperError;
use error_stack::{IntoReport, Report};
use std::collections::HashMap;
use which::which;

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
    task_command_map.insert("install".to_string(), "go get -u".to_string());
    task_command_map.insert("compile".to_string(), "go build".to_string());
    task_command_map.insert("build".to_string(), "go build".to_string());
    if std::env::current_dir()
        .map(|dir| dir.join(".goreleaser.yaml").exists())
        .is_ok()
    {
        task_command_map.insert(
            "release".to_string(),
            "goreleaser release --clean".to_string(),
        );
    } else {
        task_command_map.insert(
            "release".to_string(),
            "go build -ldflags '-s -w'".to_string(),
        );
    }
    if std::env::current_dir()
        .map(|dir| dir.join("main.go").exists())
        .is_ok()
    {
        task_command_map.insert("start".to_string(), "go run main.go".to_string());
    }
    task_command_map.insert("test".to_string(), "go test".to_string());
    task_command_map.insert("deps".to_string(), "go list -m all".to_string());
    task_command_map.insert("doc".to_string(), "go doc".to_string());
    task_command_map.insert("clean".to_string(), "go clean".to_string());
    task_command_map.insert("outdated".to_string(), "go list -u -m all".to_string());
    task_command_map.insert("update".to_string(), "go get -u".to_string());
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
            "go".to_string()
        ).into_report())
    }
}
