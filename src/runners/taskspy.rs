use crate::command_utils::{capture_command_output, run_command, CommandOutput};
use crate::common::pyproject::get_uv_tool_path;
use crate::errors::KeeperError;
use crate::models::Task;
use crate::task;
use error_stack::{Report, ResultExt};
use serde::{Deserialize, Serialize};
use which::which;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TasksPy {
    pub tasks: Option<Vec<PyTask>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PyTask {
    pub name: String,
    pub help: Option<String>,
}

pub fn is_available() -> bool {
    std::env::current_dir()
        .map(|dir| dir.join("tasks.py").exists())
        .unwrap_or(false)
}

pub fn is_command_available() -> bool {
    get_uv_tool_path("invoke").is_some() || which("invoke").is_ok()
}

pub fn install() -> Result<CommandOutput, Report<KeeperError>> {
    run_command(
        "uv",
        &["tool", "install", "--python", "3.11", "invoke"],
        true,
    )
}

pub fn list_tasks() -> Result<Vec<Task>, Report<KeeperError>> {
    let json_text = capture_command_output("invoke", &["--list", "--list-format=json"])
        .map(|output| String::from_utf8(output.stdout).unwrap_or("{}".to_owned()))?;
    serde_json::from_str::<TasksPy>(&json_text)
        .map(|tasks_py| {
            tasks_py
                .tasks
                .map(|tasks| {
                    tasks
                        .iter()
                        .map(|task| {
                            task!(
                                task.name,
                                "invoke",
                                task.help.clone().unwrap_or("".to_owned())
                            )
                        })
                        .collect()
                })
                .unwrap_or_else(Vec::new)
        })
        .change_context(KeeperError::InvalidJustfile)
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
    if let Some(invoke) = get_uv_tool_path("invoke") {
        run_command(&invoke, &args, verbose)
    } else {
        run_command("invoke", &args, verbose)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        if let Ok(tasks) = list_tasks() {
            println!("{:?}", tasks);
        }
    }

    #[test]
    fn test_run() {
        if let Ok(output) = run_task("build", &["--verbose"], &[], true) {
            let status_code = output.status.code().unwrap_or(0);
            println!("exit code: {}", status_code);
        }
    }
}
