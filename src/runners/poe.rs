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
    pyproject_toml_has_tool("poe")
}

pub fn is_command_available() -> bool {
    let user_home = dirs::home_dir();
    if let Some(user_home) = user_home {
        let user_home = user_home.join(".local").join("bin").join("poe");
        if user_home.exists() {
            return true;
        }
    }
    which("poe").is_ok()
}

pub fn install() -> Result<Output, KeeperError> {
    run_command(
        "uv",
        &["tool", "install", "--python", "3.13", "poethepoet"],
        true,
    )
}

pub fn list_tasks() -> Result<Vec<Task>, KeeperError> {
    let mut tasks = vec![];
    if let Ok(pyproject) = PyProjectToml::get_default_project() {
        if pyproject.poe_available() {
            if let Some(scripts) = pyproject.get_poe_tasks() {
                scripts.iter().for_each(|(name, description)| {
                    tasks.push(task!(name, "poe", description));
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
    args.push(task);
    args.extend(task_args);
    run_command("poe", &args, verbose)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_tasks() {
        let tasks = list_tasks().unwrap();
        for task in &tasks {
            println!("Task: {:?}", task);
        }
    }
}
