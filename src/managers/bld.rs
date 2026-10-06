use crate::command_utils::{CommandOutput, run_command_line};
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
    let bld_command = get_bld_command();
    let mut task_command_map = HashMap::new();
    task_command_map.insert(
        "install".to_string(),
        format!("{} download", bld_command).to_owned(),
    );
    task_command_map.insert(
        "compile".to_string(),
        format!("{} compile", bld_command).to_owned(),
    );
    task_command_map.insert(
        "build".to_string(),
        format!("{} jar", bld_command).to_owned(),
    );
    task_command_map.insert(
        "release".to_string(),
        format!("{} uberjar", bld_command).to_owned(),
    );
    task_command_map.insert(
        "start".to_string(),
        format!("{} run", bld_command).to_owned(),
    );
    task_command_map.insert(
        "test".to_string(),
        format!("{} test", bld_command).to_owned(),
    );
    task_command_map.insert(
        "deps".to_string(),
        format!("{} dependency-tree", bld_command).to_owned(),
    );
    task_command_map.insert(
        "outdated".to_string(),
        format!("{} updates", bld_command).to_owned(),
    );
    task_command_map.insert(
        "clean".to_string(),
        format!("{} clean", bld_command).to_owned(),
    );
    task_command_map
}

fn get_bld_command() -> &'static str {
    if cfg!(windows) {
        let wrapper_available = std::env::current_dir()
            .map(|dir| dir.join("bld.bat").exists())
            .unwrap_or(false);
        if wrapper_available {
            ".\\bld.bat"
        } else {
            "bld.bat"
        }
    } else {
        let wrapper_available = std::env::current_dir()
            .map(|dir| dir.join("bld").exists())
            .unwrap_or(false);
        if wrapper_available { "./bld" } else { "bld" }
    }
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
        Err(KeeperError::ManagerTaskNotFound(task.to_owned(), "bld".to_string()).into_report())
    }
}
