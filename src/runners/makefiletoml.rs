use std::collections::HashMap;
use std::process::Output;
use error_stack::{IntoReport, Result, ResultExt};
use serde::{Deserialize, Serialize};
use crate::errors::KeeperError;
use crate::models::Task;
use crate::command_utils::run_command;
use crate::task;
use which::which;

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
struct MakefileToml {
    pub tasks: Option<HashMap<String, TaskToml>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskToml {
    pub description: Option<String>,
    pub category: Option<String>,
    pub command: Option<String>,
    pub script: Option<Vec<String>>,
}

pub fn is_available() -> bool {
    std::env::current_dir()
        .map(|dir| dir.join("Makefile.toml").exists())
        .unwrap_or(false)
}

pub fn is_command_available() -> bool {
    which("cargo-make").is_ok()
}

pub fn list_tasks() -> Result<Vec<Task>, KeeperError> {
    std::env::current_dir()
        .map(|dir| dir.join("Makefile.toml"))
        .map(|path| std::fs::read_to_string(path).unwrap_or("{}".to_owned()))
        .map(|data| toml::from_str::<MakefileToml>(&data).unwrap())
        .map(|makefile_toml| {
            makefile_toml.tasks
                .map(|tasks| {
                    tasks.iter()
                        .map(|(name, task)| task!(name, "cargo-make", task.description.clone().unwrap_or("".to_owned())))
                        .collect()
                })
                .unwrap_or_else(|| vec![])
        })
        .report()
        .change_context(KeeperError::InvalidMakefileToml)
}

pub fn run_task(task: &str, task_args: &[&str], global_args: &[&str], verbose: bool) -> Result<Output, KeeperError> {
    let mut args = vec!["make", "-t", task];
    args.extend(global_args);
    args.push("make");
    args.push("-t");
    args.push(task);
    args.extend(task_args);
    run_command("cargo", &args, verbose)
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
        if let Ok(output) = run_task("my-ip2", &[], &[],true) {
            let status_code = output.status.code().unwrap_or(0);
            println!("exit code: {}", status_code);
        }
    }
}
