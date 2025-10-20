use crate::command_utils::{run_command_line, CommandOutput};
use crate::errors::KeeperError;
use error_stack::{IntoReport, Report};
use std::collections::HashMap;
use which::which;

pub fn is_available() -> bool {
    std::env::current_dir()
        .map(|dir| dir.join("build.sbt").exists())
        .unwrap_or(false)
}

pub fn is_command_available() -> bool {
    which("sbt").is_ok()
}

pub fn get_task_command_map() -> HashMap<String, String> {
    let mut task_command_map = HashMap::new();
    task_command_map.insert("install".to_string(), "sbt update".to_string());
    task_command_map.insert("compile".to_string(), "sbt compile".to_string());
    task_command_map.insert("build".to_string(), "sbt package".to_string());
    task_command_map.insert("start".to_string(), "sbt run".to_string());
    task_command_map.insert("test".to_string(), "sbt test".to_string());
    task_command_map.insert("deps".to_string(), "sbt dependencyTree".to_string());
    task_command_map.insert("doc".to_string(), "sbt doc".to_string());
    task_command_map.insert("clean".to_string(), "sbt clean".to_string());
    task_command_map.insert("outdated".to_string(), "sbt dependencyUpdates".to_string());
    task_command_map.insert("update".to_string(), "sbt update".to_string());
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
            "sbt".to_string()
        ).into_report())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute() {
        run_task("compile", &[], &[], false).unwrap();
    }
}
