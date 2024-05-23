use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Output;
use error_stack::{report, Result};
use which::which;
use crate::command_utils::{run_command_line};
use crate::errors::KeeperError;

pub fn is_available() -> bool {
    std::env::current_dir()
        .map(|dir| dir.join("meson").exists())
        .unwrap_or(false)
}

pub fn is_command_available() -> bool {
    which("meson").is_ok()
}

pub fn get_task_command_map() -> HashMap<String, String> {
    let mut task_command_map = HashMap::new();
    let build_dir = get_build_dir();
    task_command_map.insert("init".to_string(), "meson setup builddir".to_string());
    task_command_map.insert("compile".to_string(), format!("meson compile -C {}", build_dir));
    task_command_map.insert("build".to_string(), format!("meson dist -C {}", build_dir));
    task_command_map.insert("test".to_string(), format!("build_dir test -C {}", build_dir));
    task_command_map
}

pub fn run_task(task: &str, _task_args: &[&str], _global_args: &[&str], verbose: bool) -> Result<Output, KeeperError> {
    if let Some(command_line) = get_task_command_map().get(task) {
        if task != "init" {
            let build_dir = get_build_dir();
            if !PathBuf::from(&build_dir).join(build_dir).exists() {
                run_command_line(&format!("meson setup {}", build_dir), verbose);
            }
        }
        run_command_line(command_line, verbose)
    } else {
        Err(report!(KeeperError::ManagerTaskNotFound(task.to_owned(), "meson".to_string())))
    }
}


fn get_build_dir() -> String {
    let paths = std::fs::read_dir(".").unwrap();
    for path in paths {
        let child = path.unwrap().path();
        if child.is_dir() {
            if child.join("compile_commands.json").exists() {
                return child.to_str().unwrap().to_owned();
            }
        }
    }
    "builddir".to_owned()
}

