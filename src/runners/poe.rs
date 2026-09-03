use error_stack::Report;
use crate::command_utils::{run_command, CommandOutput};
use crate::common::pyproject::{get_uv_tool_path, PyProjectToml};
use crate::common::pyproject_toml_has_tool;
use crate::errors::KeeperError;
use crate::models::Task;
use crate::task;
use which::which;

pub fn is_available() -> bool {
    pyproject_toml_has_tool("poe")
}

pub fn is_command_available() -> bool {
    get_uv_tool_path("poe").is_some() || which("poe").is_ok()
}

pub fn install() -> Result<CommandOutput, Report<KeeperError>> {
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
) -> Result<CommandOutput, Report<KeeperError>> {
    let mut args = vec![];
    args.extend(global_args);
    args.push(task);
    args.extend(task_args);
    if let Some(poe) = get_uv_tool_path("poe") {
        run_command(&poe, &args, verbose)
    } else {
        run_command("poe", &args, verbose)
    }
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
