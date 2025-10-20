use crate::command_utils::{is_command_available, run_command_with_env_vars, CommandOutput};
use crate::errors::KeeperError;
use crate::models::Task;
use crate::task;
use colored::Colorize;
use error_stack::{IntoReport, Report};
use jsonc_parser::parse_to_serde_value;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::HashMap;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Configuration {
    pub label: String,
    pub command: String,
    pub args: Option<Vec<String>>,
    pub env: Option<HashMap<String, String>>,
    pub use_new_terminal: Option<bool>,
    pub allow_concurrent_runs: Option<bool>,
}

impl Configuration {
    #[allow(dead_code)]
    pub fn new_command(label: &str, command: &str, args: &[String]) -> Self {
        Configuration {
            label: label.to_string(),
            command: command.to_string(),
            args: Some(args.to_vec()),
            ..Default::default()
        }
    }

    pub fn command_line(&self) -> String {
        if self.args.is_none() {
            return self.command.clone();
        } else {
            let args = self
                .args
                .clone()
                .unwrap()
                .iter()
                .map(|s| shell_escape::escape(Cow::from(s)).to_string())
                .collect::<Vec<String>>();
            format!("{} {}", self.command, args.join(" "))
        }
    }
}

pub fn is_available() -> bool {
    std::env::current_dir()
        .map(|dir| dir.join(".zed").join("tasks.json").exists())
        .unwrap_or(false)
}

pub fn list_tasks() -> Result<Vec<Task>, Report<KeeperError>> {
    Ok(parse_tasks_json()
        .iter()
        .map(|configuration| task!(&configuration.label, "zed", configuration.command_line()))
        .collect())
}

fn parse_tasks_json() -> Vec<Configuration> {
    std::env::current_dir()
        .map(|dir| dir.join(".zed").join("tasks.json"))
        .map(|path| std::fs::read_to_string(path).unwrap_or("[]".to_owned()))
        .map(|data| {
            parse_to_serde_value(&data, &Default::default())
                .unwrap()
                .unwrap()
        })
        .map(|json_value| serde_json::from_value::<Vec<Configuration>>(json_value).unwrap())
        .unwrap()
}

pub fn run_task(
    task_name: &str,
    _task_args: &[&str],
    _global_args: &[&str],
    verbose: bool,
) -> Result<CommandOutput, Report<KeeperError>> {
    let configurations = parse_tasks_json();
    let result = configurations
        .iter()
        .find(|configuration| configuration.label == task_name);
    if let Some(configuration) = result {
        run_configuration(configuration, verbose)
    } else {
        Err(KeeperError::TaskNotFound(task_name.to_owned()).into_report())
    }
}

fn run_configuration(
    configuration: &Configuration,
    verbose: bool,
) -> Result<CommandOutput, Report<KeeperError>> {
    let command_name = &configuration.command;
    let args = configuration.args.clone().unwrap_or_default();
    let args: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    if is_command_available(&command_name) {
        Ok(
            run_command_with_env_vars(&command_name, &args, &None, &configuration.env, verbose)
                .unwrap(),
        )
    } else {
        println!(
            "{}",
            format!("{} is not available", command_name).bold().red()
        );
        Err(KeeperError::CommandNotFound(command_name.clone()).into_report())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        println!("exits: {}", is_available());
        if let Ok(tasks) = list_tasks() {
            println!("{:?}", tasks);
        }
    }

    #[test]
    fn test_run() {
        run_task("bash echo", &[], &[], false).unwrap();
    }

    #[test]
    fn test_run_configuration() {
        let configuration =
            Configuration::new_command("my-ip", "curl", &["https://httpbin.org/ip".to_owned()]);
        run_configuration(&configuration, true).unwrap();
    }
}
