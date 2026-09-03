use error_stack::Report;
use crate::command_utils::{run_command, CommandOutput};
use crate::common::pyproject::{get_uv_tool_path, PyProjectToml};
use crate::common::pyproject_toml_has_tool;
use crate::errors::KeeperError;
use crate::models::Task;
use crate::task;
use which::which;

pub fn is_available() -> bool {
    std::env::current_dir()
        .map(|_dir| pyproject_toml_has_tool("poetry"))
        .unwrap_or(false)
}

pub fn is_command_available() -> bool {
    get_uv_tool_path("poetry").is_some() || which("poetry").is_ok()
}

pub fn install() -> Result<CommandOutput, Report<KeeperError>> {
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
) -> Result<CommandOutput, Report<KeeperError>> {
    let mut args = vec![];
    args.extend(global_args);
    args.push("run");
    args.push(task);
    args.extend(task_args);
    if let Some(poetry) = get_uv_tool_path("poetry") {
        run_command(&poetry, &args, verbose)
    } else {
        run_command("poetry", &args, verbose)
    }
}
