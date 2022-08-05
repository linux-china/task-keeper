use std::collections::HashMap;
use std::process::Output;
use clap::CommandFactory;
use serde::{Deserialize, Serialize};
use crate::errors::KeeperError;
use crate::models::Task;
use crate::task;
use error_stack::{IntoReport, Result, ResultExt};
use crate::runners::{run_command, run_command_with_env_vars};


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FleetRunJson {
    pub configurations: Vec<Configuration>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Configuration {
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub program: Option<String>,
    pub environment: Option<HashMap<String, String>>,
    pub args: Option<Vec<String>>,
    pub tasks: Option<Vec<String>>,
    pub python_executable_path: Option<String>,
    pub script_path: Option<String>,
    pub arguments: Option<Vec<String>>,
    pub parameters: Option<Vec<String>>,
    pub cargo_args: Option<Vec<String>>,
    pub cargo_extra_args: Option<Vec<String>>,
    pub go_exec_path: Option<String>,
    pub params: Option<Vec<String>>,
}

impl Configuration {
    pub fn new_command(name: &str, command: &str, args: &[String]) -> Self {
        Configuration {
            name: name.to_owned(),
            type_field: "command".to_owned(),
            program: Some(command.to_owned()),
            args: Some(args.to_vec()),
            ..Default::default()
        }
    }

    pub fn cargo_full_args(&self) -> Vec<String> {
        let mut full_args = vec![];
        if let Some(ref args) = self.cargo_args {
            full_args.extend(args.iter().cloned());
        }
        if let Some(ref args) = self.cargo_extra_args {
            full_args.extend(args.iter().cloned());
        }
        full_args
    }

    pub fn python_full_args(&self) -> Vec<String> {
        let mut full_args = vec![];
        if let Some(ref script_path) = self.script_path {
            full_args.push(script_path.to_string());
        }
        if let Some(ref parameters) = self.parameters {
            full_args.extend(parameters.iter().cloned());
        }
        full_args
    }
}

pub fn is_available() -> bool {
    std::env::current_dir()
        .map(|dir| dir.join(".fleet").join("run.json").exists())
        .unwrap_or(false)
}

pub fn list_tasks() -> Result<Vec<Task>, KeeperError> {
    Ok(parse_run_json().configurations.iter().map(|configuration| task!(configuration.name, "fleet")).collect())
}

fn parse_run_json() -> FleetRunJson {
    std::env::current_dir()
        .map(|dir| dir.join(".fleet").join("run.json"))
        .map(|path| std::fs::read_to_string(path).unwrap_or("{}".to_owned()))
        .map(|data| serde_jsonrc::from_str::<FleetRunJson>(&data).unwrap())
        .unwrap()
}

pub fn run_task(task_name: &str, _extra_args: &[&str], verbose: bool) -> Result<Output, KeeperError> {
    parse_run_json().configurations.iter()
        .find(|configuration| configuration.name == task_name)
        .map(|configuration| {
            let args = get_command_args(configuration);
            let args: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
            let command_name = get_command_name(configuration);
            run_command_with_env_vars(&command_name, &args, &configuration.environment, verbose).unwrap()
        })
        .ok_or_else(|| KeeperError::TaskNotFound(task_name.to_owned()))
        .report()
}

fn run_configuration(configuration: &Configuration, verbose: bool) -> Result<Output, KeeperError> {
    let command_name = get_command_name(configuration);
    let args = configuration.args.clone().unwrap_or_default();
    let args: Vec<&str> = args.iter()
        .map(|arg| arg.as_str())
        .collect();
    run_command(&command_name, &args, verbose)
}

//todo: add support for other types
fn get_command_name(configuration: &Configuration) -> String {
    match configuration.type_field.as_str() {
        "cargo" => "cargo".to_owned(),
        "maven" => "mvn".to_owned(),
        "gradle" => "./gradlew".to_owned(),
        "python" => configuration.python_executable_path.clone().unwrap_or("python".to_owned()),
        "go" => configuration.go_exec_path.clone().unwrap_or("go".to_owned()),
        "command" => configuration.program.clone().unwrap_or_default(),
        _ => "".to_owned(),
    }
}

fn get_command_args(configuration: &Configuration) -> Vec<String> {
    match configuration.type_field.as_str() {
        "command" => configuration.args.clone().unwrap_or_default(),
        "cargo" => configuration.cargo_full_args(),
        "maven" | "gradle" => configuration.tasks.clone().unwrap_or_default(),
        "python" => configuration.python_full_args().clone(),
        "go" => configuration.params.clone().unwrap_or_default(),
        _ => vec![],
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
        run_task("my-ip", &[], false).unwrap();
    }

    #[test]
    fn test_run_configuration() {
        let mut configuration = Configuration::new_command("my-ip", "curl", &["https://httpbin.org/ip".to_owned()]);
        run_configuration(&configuration, true).unwrap();
    }
}
