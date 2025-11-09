use crate::command_utils::{run_command_by_shell, run_command_with_env_vars, CommandOutput};
use crate::errors::KeeperError;
use crate::models::Task;
use crate::task;
use error_stack::Report;
use jsonc_parser::parse_to_serde_value;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;

#[derive(Serialize, Deserialize, Debug, Default)]
struct TasksJson {
    pub version: String,
    pub tasks: Option<Vec<VSTask>>,
}

impl TasksJson {
    pub fn find_task(&self, task_name: &str) -> Option<VSTask> {
        if let Some(tasks) = &self.tasks {
            let result = tasks.iter().find(|task| {
                if let Some(label) = &task.label {
                    label == task_name
                } else {
                    false
                }
            });
            if let Some(task) = result {
                return Some(task.clone());
            }
        }
        None
    }
}

// Command options
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
struct CommandOptions {
    // current working directory
    cwd: Option<String>,
    // environment variables passed to the task
    env: Option<HashMap<String, String>>,
}

// --- Platform-specific configuration ---
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
struct PlatformConfig {
    command: Option<String>,
    args: Option<Vec<String>>,
    options: Option<CommandOptions>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
struct VSTask {
    pub label: Option<String>,
    #[serde(rename = "type")]
    pub task_type: String,
    pub command: Option<String>,
    options: Option<CommandOptions>,
    args: Option<Vec<String>>,
    // Platform-specific configurations
    #[allow(dead_code)]
    windows: Option<PlatformConfig>,
    #[allow(dead_code)]
    linux: Option<PlatformConfig>,
    #[allow(dead_code)]
    osx: Option<PlatformConfig>,
}

impl VSTask {
    pub fn get_command(&self) -> Option<String> {
        // Determine current platform
        #[cfg(target_os = "windows")]
        let platform_config = &self.windows;

        #[cfg(target_os = "linux")]
        let platform_config = &self.linux;

        #[cfg(target_os = "macos")]
        let platform_config = &self.osx;
        if let Some(config) = platform_config {
            if let Some(cmd) = &config.command {
                return Some(cmd.clone());
            }
        }
        self.command.clone()
    }

    pub fn get_command_options(&self) -> Option<CommandOptions> {
        // Determine current platform
        #[cfg(target_os = "windows")]
        let platform_config = &self.windows;

        #[cfg(target_os = "linux")]
        let platform_config = &self.linux;

        #[cfg(target_os = "macos")]
        let platform_config = &self.osx;
        if let Some(config) = platform_config {
            if let Some(option) = &config.options {
                return Some(option.clone());
            }
        }
        self.options.clone()
    }
}

pub fn is_available() -> bool {
    std::env::current_dir()
        .map(|dir| dir.join(".vscode").join("tasks.json").exists())
        .unwrap_or(false)
}

pub fn list_tasks() -> Result<Vec<Task>, Report<KeeperError>> {
    Ok(parse_run_json()
        .tasks
        .map(|tasks| {
            tasks
                .into_iter()
                .map(|task| {
                    if task.task_type == "shell" {
                        Some(task!(
                            &task.label.clone().unwrap(),
                            "vscode",
                            "shell",
                            &task.command.clone().unwrap()
                        ))
                    } else if let Some(cmd) = task.command {
                        Some(task!(&task.label.clone().unwrap(), "vscode", &cmd))
                    } else {
                        None
                    }
                })
                .flatten()
                .collect()
        })
        .unwrap_or_else(|| vec![]))
}

fn parse_run_json() -> TasksJson {
    std::env::current_dir()
        .map(|dir| dir.join(".vscode").join("tasks.json"))
        .map(|path| std::fs::read_to_string(path).unwrap_or("{}".to_owned()))
        .map(|data| {
            parse_to_serde_value(&data, &Default::default())
                .unwrap()
                .unwrap()
        })
        .map(|json_value| {
            serde_json::from_value::<TasksJson>(json_value).expect(".vscode/tasks.json format")
        })
        .unwrap()
}

pub fn run_task(
    task_name: &str,
    _task_args: &[&str],
    _global_args: &[&str],
    verbose: bool,
) -> Result<CommandOutput, Report<KeeperError>> {
    let tasks_json = parse_run_json();
    let task = tasks_json
        .find_task(task_name)
        .ok_or_else(|| KeeperError::TaskNotFound(task_name.to_string()))?;
    let command = task.command.clone().unwrap();
    if task.task_type == "shell" {
        run_command_by_shell(&command, verbose)
    } else {
        let mut workspace_root = env::current_dir().unwrap().to_str().unwrap().to_string();
        let mut command_env_vars: Option<HashMap<String, String>> = None;
        let command = task.get_command().unwrap();
        let options = task.get_command_options();
        if let Some(options) = &options {
            if let Some(cwd) = &options.cwd {
                workspace_root = cwd.to_string();
            }
            if let Some(env_vars) = &options.env {
                command_env_vars = Some(env_vars.clone());
            }
        }
        let command_and_args = shlex::split(&command).unwrap();
        let command_name = command_and_args.get(0).unwrap();
        let mut command_args: Vec<&str> = command_and_args[1..].iter().map(AsRef::as_ref).collect();
        if let Some(args) = &task.args {
            for arg in args {
                command_args.push(arg);
            }
        }
        run_command_with_env_vars(
            command_name,
            &command_args,
            &Some(workspace_root),
            &command_env_vars,
            verbose,
        )
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
