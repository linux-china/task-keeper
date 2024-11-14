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
        .map(|dir| pyproject_toml_has_tool("poe"))
        .unwrap_or(false)
}

pub fn is_command_available() -> bool {
    which("poe").is_ok()
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
