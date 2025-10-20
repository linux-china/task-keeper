use crate::command_utils::{is_command_available, run_command_with_env_vars, CommandOutput};
use crate::errors::KeeperError;
use crate::models::Task;
use crate::task;
use colored::Colorize;
use error_stack::{IntoReport, Report};
use jsonc_parser::parse_to_serde_value;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FleetRunJson {
    pub configurations: Vec<Configuration>,
}

// https://www.jetbrains.com/help/fleet/run-configurations.html
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Configuration {
    pub name: String,
    #[serde(rename = "type")]
    pub type_value: String,
    pub program: Option<String>,
    pub command: Option<String>,
    pub scripts: Option<String>,
    pub working_dir: Option<String>,
    pub environment: Option<HashMap<String, String>>,
    pub args: Option<Vec<String>>,
    pub tasks: Option<Vec<String>>,
    pub script_path: Option<String>,
    pub arguments: Option<Vec<String>>,
    pub command_arguments: Option<Vec<String>>,
    pub parameters: Option<Vec<String>>,
    pub app_options: Option<Vec<String>>,
    pub cargo_args: Option<Vec<String>>,
    pub cargo_extra_args: Option<Vec<String>>,
    pub go_exec_path: Option<String>,
    pub params: Option<Vec<String>>,
    pub build_params: Option<Vec<String>>,
    pub main_class: Option<String>,
    pub file: Option<String>,
    // docker
    pub image_id_or_name: Option<String>,
    pub run_options: Option<String>,
    // fastapi
    pub module: Option<String>,
    pub application: Option<String>,
}

impl Configuration {
    #[allow(dead_code)]
    pub fn new_command(name: &str, command: &str, args: &[String]) -> Self {
        Configuration {
            name: name.to_owned(),
            type_value: "command".to_owned(),
            program: Some(command.to_owned()),
            args: Some(args.to_vec()),
            ..Default::default()
        }
    }

    pub fn formatted_name(&self) -> String {
        str::replace(&self.name, " ", "-")
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
        if let Some(ref script_path) = self.file {
            full_args.push(script_path.to_string());
        }
        if let Some(ref arguments) = self.arguments {
            full_args.extend(arguments.iter().cloned());
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
    Ok(parse_run_json()
        .configurations
        .iter()
        .map(|configuration| {
            let description = if &configuration.type_value == "command" {
                configuration.program.clone().unwrap()
            } else {
                configuration.type_value.clone()
            };
            task!(&configuration.formatted_name(), "fleet", description)
        })
        .collect())
}

fn parse_run_json() -> FleetRunJson {
    std::env::current_dir()
        .map(|dir| dir.join(".fleet").join("run.json"))
        .map(|path| std::fs::read_to_string(path).unwrap_or("{}".to_owned()))
        .map(|data| {
            parse_to_serde_value(&data, &Default::default())
                .unwrap()
                .unwrap()
        })
        .map(|json_value| serde_json::from_value::<FleetRunJson>(json_value).unwrap())
        .unwrap()
}

pub fn run_task(
    task_name: &str,
    _task_args: &[&str],
    _global_args: &[&str],
    verbose: bool,
) -> Result<CommandOutput, Report<KeeperError>> {
    let run_json = parse_run_json();
    let result = run_json
        .configurations
        .iter()
        .find(|configuration| configuration.formatted_name() == task_name);
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
    let command_name = get_command_name(configuration);
    let args = get_command_args(configuration);
    let args: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    if is_command_available(&command_name) {
        Ok(run_command_with_env_vars(
            &command_name,
            &args,
            &configuration.working_dir,
            &configuration.environment,
            verbose,
        )?)
    } else {
        println!(
            "{}",
            format!("{} is not available", command_name).bold().red()
        );
        Err(KeeperError::CommandNotFound(command_name).into_report())
    }
}

//todo: add support for other types: spring-boot
fn get_command_name(configuration: &Configuration) -> String {
    match configuration.type_value.as_str() {
        "cargo" => "cargo".to_owned(),
        "maven" | "maven-run" => "mvn".to_owned(),
        "gradle" => "./gradlew".to_owned(),
        "docker-run" => "docker".to_owned(),
        "python" => "python".to_owned(),
        "flask" => "python".to_owned(),
        "fastapi" => "python".to_owned(),
        "node" => "node".to_owned(),
        "npm" => "npm".to_owned(),
        "php" => "php".to_owned(),
        "go" => configuration
            .go_exec_path
            .clone()
            .unwrap_or("go".to_owned()),
        "command" => configuration.program.clone().unwrap_or_default(),
        _ => "".to_owned(),
    }
}

fn get_command_args(configuration: &Configuration) -> Vec<String> {
    match configuration.type_value.as_str() {
        "command" => configuration.args.clone().unwrap_or_default(),
        "cargo" => configuration.cargo_full_args(),
        "maven" | "gradle" => configuration.tasks.clone().unwrap_or_default(),
        "maven-run" => {
            if let Some(args) = &configuration.args {
                let args_text = args.join(" ");
                vec![
                    "compile".to_owned(),
                    "exec:java".to_owned(),
                    format!(
                        "-Dexec.mainClass='{}'",
                        configuration.main_class.clone().unwrap_or_default()
                    ),
                    format!("-Dexec.args='{}'", args_text),
                ]
            } else {
                vec![
                    "compile".to_owned(),
                    "exec:java".to_owned(),
                    format!(
                        "-Dexec.mainClass={}",
                        configuration.main_class.clone().unwrap_or_default()
                    ),
                ]
            }
        }
        "docker-run" => {
            if let Some(run_options_text) = &configuration.run_options {
                let options = shlex::split(run_options_text).unwrap();
                let mut args = vec!["run".to_owned()];
                args.extend(options.iter().cloned());
                args.push(configuration.image_id_or_name.clone().unwrap_or_default());
                args
            } else {
                vec![
                    "run".to_owned(),
                    configuration.image_id_or_name.clone().unwrap_or_default(),
                ]
            }
        }
        "python" => configuration.python_full_args().clone(),
        "flask" => {
            let mut args = vec!["-m".to_owned(), "flask".to_owned(), "run".to_owned()];
            args.extend(configuration.arguments.clone().unwrap_or_default());
            args
        }
        "fastapi" => {
            let module_and_app = format!(
                "{}{}",
                &configuration.module.clone().unwrap_or("".to_owned()),
                &configuration.application.clone().unwrap_or("".to_owned())
            );
            let mut args = vec!["-m".to_owned(), "unicorn".to_owned(), module_and_app];
            args.extend(configuration.arguments.clone().unwrap_or_default());
            args
        }
        "php" => {
            let mut args = vec![];
            if let Some(file) = &configuration.file {
                args.push(file.clone());
            }
            args.extend(configuration.arguments.clone().unwrap_or_default());
            args
        }
        "go" => configuration.build_params.clone().unwrap_or_default(),
        "node" => {
            let mut args = vec![];
            if let Some(file) = &configuration.file {
                args.push(file.clone());
            }
            args.extend(configuration.app_options.clone().unwrap_or_default());
            args
        }
        "npm" => {
            let mut args = vec![];
            if let Some(command) = &configuration.command {
                args.push(command.clone());
                args.extend(configuration.command_arguments.clone().unwrap_or_default());
            } else if let Some(scripts) = &configuration.scripts {
                args.push("run".to_owned());
                args.push(scripts.clone());
            }
            args
        }
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
        run_task("my-ip", &[], &[], false).unwrap();
    }

    #[test]
    fn test_run_configuration() {
        let configuration =
            Configuration::new_command("my-ip", "curl", &["https://httpbin.org/ip".to_owned()]);
        run_configuration(&configuration, true).unwrap();
    }
}
