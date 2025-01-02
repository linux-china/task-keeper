use crate::command_utils::run_command;
use crate::common::pyproject::PyProjectToml;
use crate::common::pyproject_toml_has_tool;
use crate::errors::KeeperError;
use crate::models::Task;
use crate::task;
use error_stack::Result;
use std::process::Output;
use which::which;

pub fn is_available() -> bool {
    std::env::current_dir()
        .map(|_dir| pyproject_toml_has_tool("poetry"))
        .unwrap_or(false)
}

pub fn is_command_available() -> bool {
    let user_home = dirs::home_dir();
    if let Some(user_home) = user_home {
        let user_home = user_home.join(".local").join("bin").join("poetry");
        if user_home.exists() {
            return true;
        }
    }
    which("poetry").is_ok()
}

pub fn install() -> Result<Output, KeeperError> {
    run_command(
        "uv",
        &["tool", "install", "--python", "3.13", "poetry"],
        true,
    )
}

pub fn list_tasks() -> Result<Vec<Task>, KeeperError> {
    let mut tasks = vec![];
    if let Ok(pyproject) = PyProjectToml::get_default_project() {
        if pyproject.poetry_available() {
            if let Some(scripts) = pyproject.get_poetry_scripts() {
                scripts.iter().for_each(|(name, description)| {
                    tasks.push(task!(name, "poetry", description));
                });
            }
        }
    }
    Ok(tasks)
}

pub fn run_task(
    task: &str,
    task_args: &[&str],
    global_args: &[&str],
    verbose: bool,
) -> Result<Output, KeeperError> {
    let mut args = vec![];
    args.extend(global_args);
    args.push("run");
    args.push(task);
    args.extend(task_args);
    run_command("poetry", &args, verbose)
}
