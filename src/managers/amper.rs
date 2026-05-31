use crate::command_utils::{CommandOutput, run_command_line};
use crate::errors::KeeperError;
use error_stack::{IntoReport, Report};
use std::collections::HashMap;

pub fn is_available() -> bool {
    std::env::current_dir()
        .map(|dir| {
            dir.join("amper").exists()
                && (dir.join("module.yaml").exists() || dir.join("project.yaml").exists())
        })
        .unwrap_or(false)
}

pub fn is_command_available() -> bool {
    std::env::current_dir()
        .map(|dir| dir.join("amper").exists())
        .unwrap_or(false)
}

pub fn get_task_command_map() -> HashMap<String, String> {
    let mut task_command_map = HashMap::new();
    let amper_command = get_amper_command();
    task_command_map.insert("compile".to_string(), format!("{} build", amper_command));
    task_command_map.insert("build".to_string(), format!("{} build", amper_command));
    task_command_map.insert("start".to_string(), format!("{} run", amper_command));
    task_command_map.insert("test".to_string(), format!("{} test", amper_command));
    task_command_map.insert("clean".to_string(), format!("{} clean", amper_command));
    if let Ok(code) = std::fs::read_to_string("./amper") {
        if !code.contains("amper_version=0.10.0") {
            task_command_map.insert(
                "self-update".to_string(),
                format!("{} update", amper_command),
            );
        }
    }
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
        Err(KeeperError::ManagerTaskNotFound(task.to_owned(), "amper".to_string()).into_report())
    }
}

fn get_amper_command() -> &'static str {
    let wrapper_available = std::env::current_dir()
        .map(|dir| dir.join("amper").exists())
        .unwrap_or(false);
    if wrapper_available {
        "./amper"
    } else {
        "amper"
    }
}
