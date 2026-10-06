use crate::command_utils::{
    is_command_available, run_command_line_with_env_vars, run_command_with_env_vars, CommandOutput,
};
use crate::errors::KeeperError;
use crate::models::Task;
use crate::task;
use colored::Colorize;
use error_stack::{IntoReport, Report, ResultExt};
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
    Ok(parse_tasks_json()?
        .iter()
        .map(|configuration| task!(&configuration.label, "zed", configuration.command_line()))
        .collect())
}

fn parse_tasks_json() -> Result<Vec<Configuration>, Report<KeeperError>> {
    let data = std::env::current_dir()
        .map(|dir| dir.join(".zed").join("tasks.json"))
        .map(|path| std::fs::read_to_string(path).unwrap_or("[]".to_owned()))
        .change_context(KeeperError::InvalidZedTasksJson)?;
    let json_value = parse_to_serde_value::<serde_json::Value>(&data, &Default::default())
        .change_context(KeeperError::InvalidZedTasksJson)?;
    serde_json::from_value::<Vec<Configuration>>(json_value)
        .change_context(KeeperError::InvalidZedTasksJson)
}

pub fn run_task(
    task_name: &str,
    _task_args: &[&str],
    _global_args: &[&str],
    verbose: bool,
) -> Result<CommandOutput, Report<KeeperError>> {
    let configurations = parse_tasks_json()?;
    let result = configurations
        .iter()
        .find(|configuration| configuration.label == task_name);
    if let Some(configuration) = result {
        run_configuration(configuration, verbose)
    } else {
        Err(KeeperError::TaskNotFound(task_name.to_owned()).into_report())
    }
}

/// Zed runs a task by shell, and `command` is often a whole command line, such as `cargo run --release`,
/// so only a single program with `args` is executed directly, which keeps argument boundaries.
fn run_configuration(
    configuration: &Configuration,
    verbose: bool,
) -> Result<CommandOutput, Report<KeeperError>> {
    let command_name = configuration.command.trim();
    let args = configuration.args.clone().unwrap_or_default();
    if args.is_empty() {
        return run_command_line_with_env_vars(command_name, &configuration.env, verbose);
    }
    if command_name.contains(char::is_whitespace) {
        return run_command_line_with_env_vars(&configuration.command_line(), &configuration.env, verbose);
    }
    let args: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    if is_command_available(command_name) {
        run_command_with_env_vars(command_name, &args, &None, &configuration.env, verbose)
    } else {
        println!(
            "{}",
            format!("{} is not available", command_name).bold().red()
        );
        Err(KeeperError::CommandNotFound(command_name.to_owned()).into_report())
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
    fn test_run_command_line_without_args() {
        let configuration = Configuration::new_command("version", "cargo --version", &[]);
        let configuration = Configuration { args: None, ..configuration };
        assert!(run_configuration(&configuration, false).unwrap().status.success());
    }

    #[cfg(unix)]
    #[test]
    fn test_run_command_line_with_env() {
        let mut env = HashMap::new();
        env.insert("TK_ZED_ENV".to_owned(), "yes".to_owned());
        let configuration = Configuration {
            label: "env".to_owned(),
            command: "test \"$TK_ZED_ENV\" = yes".to_owned(),
            env: Some(env),
            ..Default::default()
        };
        assert!(run_configuration(&configuration, false).unwrap().status.success());
    }

    #[test]
    fn test_run_command_with_spaces_and_args() {
        let configuration = Configuration::new_command("version", "cargo --color never", &["--version".to_owned()]);
        assert!(run_configuration(&configuration, false).unwrap().status.success());
    }

    #[test]
    fn test_run_unavailable_command() {
        let configuration = Configuration::new_command("missing", "tk-no-such-command", &["x".to_owned()]);
        assert!(run_configuration(&configuration, false).is_err());
    }

    #[test]
    fn test_run_configuration() {
        let configuration =
            Configuration::new_command("my-ip", "curl", &["https://httpbin.org/ip".to_owned()]);
        run_configuration(&configuration, true).unwrap();
    }
}
