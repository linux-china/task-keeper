use std::process::{Output};
use crate::errors::KeeperError;
use error_stack::{Result};
use crate::models::Task;
use crate::command_utils::{run_command_by_shell, run_command_line};
use crate::task;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
struct TasksJson {
    pub version: String,
    pub tasks: Option<Vec<VSTask>>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct VSTask {
    pub label: Option<String>,
    #[serde(rename = "type")]
    pub task_type: String,
    pub command: Option<String>,
}

pub fn is_available() -> bool {
    std::env::current_dir()
        .map(|dir| dir.join(".vscode").join("tasks.json").exists())
        .unwrap_or(false)
}

pub fn list_tasks() -> Result<Vec<Task>, KeeperError> {
    Ok(parse_run_json().tasks
        .map(|tasks| {
            tasks.into_iter().map(|task| {
                if task.task_type == "shell" {
                    Some(task!(&task.label.clone().unwrap(), "vscode", "shell", &task.command.clone().unwrap()))
                } else if let Some(cmd) = task.command {
                    Some(task!(&task.label.clone().unwrap(), "vscode", &cmd))
                } else {None}
            }).flatten().collect()
        })
        .unwrap_or_else(|| vec![])
    )
}


fn parse_run_json() -> TasksJson {
    std::env::current_dir()
        .map(|dir| dir.join(".vscode").join("tasks.json"))
        .map(|path| std::fs::read_to_string(path).unwrap_or("{}".to_owned()))
        .map(|data| serde_jsonrc::from_str::<TasksJson>(&data).unwrap())
        .unwrap()
}

pub fn run_task(task: &str, _task_args: &[&str], _global_args: &[&str], verbose: bool) -> Result<Output, KeeperError> {
    let tasks = list_tasks()?;
    let task = tasks.iter().find(|t| t.name == task).ok_or_else(|| {
        KeeperError::TaskNotFound(task.to_string())
    })?;
    if let Some(_runner2) = &task.runner2 {
        run_command_by_shell(&task.description, verbose)
    } else {
        run_command_line(&task.description, verbose)
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
        if let Ok(output) = run_task("run-tests", &[], &[], true) {
            let status_code = output.status.code().unwrap_or(0);
            println!("exit code: {}", status_code);
        }
        if let Ok(output) = run_task("node-version", &[], &[], true) {
            let status_code = output.status.code().unwrap_or(0);
            println!("exit code: {}", status_code);
        }
    }
}
