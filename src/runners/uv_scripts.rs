use crate::command_utils::{run_command, CommandOutput};
use crate::common::pyproject::PyProjectToml;
use crate::common::pyproject_toml_has_tool;
use crate::errors::KeeperError;
use crate::models::Task;
use crate::task;
use error_stack::{IntoReport, Report};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use toml::Value;
use which::which;

/// implement feature from https://rye.astral.sh/guide/pyproject/#toolryescripts
pub fn is_available() -> bool {
    pyproject_toml_has_tool("uv")
}

pub fn is_command_available() -> bool {
    which("uv").is_ok()
}

pub fn list_tasks() -> Result<Vec<Task>, Report<KeeperError>> {
    let mut tasks = vec![];
    if let Ok(pyproject) = PyProjectToml::get_default_project() {
        if let Some(scripts) = pyproject.get_uv_scripts() {
            scripts.iter().for_each(|(name, description)| {
                tasks.push(task!(name, "uvs", description));
            });
        }
    }
    Ok(tasks)
}

pub fn run_task(
    task: &str,
    _task_args: &[&str],
    _global_args: &[&str],
    verbose: bool,
) -> Result<CommandOutput, Report<KeeperError>> {
    let project = PyProjectToml::get_default_project().unwrap();
    if let Some(script_value) = project.get_uv_script(task) {
        let script = get_script_cmd(&script_value);
        invoke_script(&project, &script.unwrap(), verbose)
    } else {
        Err(KeeperError::TaskNotFound(task.to_owned()).into_report())
    }
}

type EnvVars = HashMap<String, String>;
type EnvFile = Option<PathBuf>;

/// A reference to a script
#[derive(Clone, Debug)]
pub enum Script {
    /// Call python module entry
    Call(String, EnvVars, EnvFile),
    /// A command alias
    Cmd(Vec<String>, EnvVars, EnvFile),
    /// A multi-script execution
    Chain(Vec<Vec<String>>),
}

pub fn get_script_cmd(tom_value: &Value) -> Option<Script> {
    match &tom_value {
        Value::String(cmd_text) => {
            let command_and_args = shlex::split(cmd_text).unwrap();
            Some(Script::Cmd(command_and_args, HashMap::new(), None))
        }
        Value::Array(arr) => {
            let command_and_args: Vec<String> = arr.iter().map(|item| item.to_string()).collect();
            Some(Script::Cmd(command_and_args, HashMap::new(), None))
        }
        Value::Table(table) => {
            let env_hash_map: HashMap<String, String> = if let Some(env) = table.get("env") {
                match env {
                    Value::Table(env_table) => env_table
                        .iter()
                        .filter_map(|(k, v)| {
                            if let Value::String(s) = v {
                                Some((k.clone(), s.clone()))
                            } else {
                                None
                            }
                        })
                        .collect(),
                    _ => HashMap::new(),
                }
            } else {
                HashMap::new()
            };
            if let Some(cmd) = table.get("cmd") {
                match cmd {
                    Value::String(cmd_text) => {
                        let command_and_args = shlex::split(cmd_text).unwrap();
                        Some(Script::Cmd(command_and_args, env_hash_map, None))
                    }
                    Value::Array(arr) => {
                        let command_and_args: Vec<String> =
                            arr.iter().map(|item| item.to_string()).collect();
                        Some(Script::Cmd(command_and_args, env_hash_map, None))
                    }
                    _ => None,
                }
            } else if let Some(call) = table.get("call") {
                match call {
                    Value::String(call_text) => {
                        let callable = call_text.to_string();
                        return Some(Script::Call(callable, env_hash_map, None));
                    }
                    _ => None,
                }
            } else if let Some(chain) = table.get("chain") {
                match chain {
                    Value::Array(chain_arr) => {
                        let commands: Vec<Vec<String>> = chain_arr
                            .iter()
                            .filter_map(|item| match item {
                                Value::Array(arr) => {
                                    Some(arr.iter().map(|v| v.to_string()).collect::<Vec<String>>())
                                }
                                Value::String(s) => Some(vec![s.to_string()]),
                                _ => None,
                            })
                            .collect();
                        Some(Script::Chain(commands))
                    }
                    _ => None,
                }
            } else {
                None
            }
        }
        _ => None,
    }
}

fn invoke_script(
    pyproject: &PyProjectToml,
    script: &Script,
    verbose: bool,
) -> Result<CommandOutput, Report<KeeperError>> {
    match script {
        Script::Call(entry, env_vars, env_file) => {
            let args: Vec<String> = if let Some((module, func)) = entry.split_once(':') {
                if module.is_empty() || func.is_empty() {
                    eprintln!("Python callable must be in the form <module_name>:<callable_name> or <module_name>");
                    std::process::exit(1)
                }
                let call = if !func.contains('(') {
                    format!("{func}()")
                } else {
                    func.to_string()
                };
                [
                    "-c".to_string(),
                    format!("import sys, {module} as _1; sys.exit(_1.{call})"),
                ]
            } else {
                ["-m".to_string(), entry.clone()]
            }
                .into_iter()
                .collect();
            // inject env variables
            if !env_vars.is_empty() {
                for (key, value) in env_vars {
                    unsafe {
                        std::env::set_var(key, value);
                    }
                }
            }
            if let Some(env_file_path) = env_file {
                dotenvx_rs::from_path(env_file_path).unwrap()
            }
            let real_args: Vec<&str> = args.iter().map(String::as_str).collect();
            let py = pyproject.venv_bin_path().join("python3");
            run_command(py.to_str().unwrap(), &real_args, verbose)
        }
        Script::Cmd(script_args, env_vars, env_file) => {
            if script_args.is_empty() {
                eprintln!("script has no arguments");
                std::process::exit(1);
            }
            // inject env variables
            if !env_vars.is_empty() {
                for (key, value) in env_vars {
                    unsafe {
                        std::env::set_var(key, value);
                    }
                }
            }
            if let Some(env_file_path) = env_file {
                dotenvx_rs::from_path(env_file_path).unwrap()
            }
            let script_target = std::env::current_dir().unwrap().join(&script_args[0]);
            if script_target.exists() && script_target.is_file() {
                let args: Vec<&str> = script_args.into_iter().map(String::as_str).collect();
                run_command("python3", &args, verbose)
            } else {
                let args: Vec<&str> = script_args[1..].iter().map(String::as_str).collect();
                let command_name = &script_args[0];
                run_command(command_name, &args, verbose)
            }
        }
        Script::Chain(commands) => {
            if commands.is_empty() {
                eprintln!("Please supply at least one command to chain");
                std::process::exit(1);
            }
            let mut index = 0;
            for command_and_args in commands {
                let script_name = &command_and_args[0];
                if let Some(script_value) = pyproject.get_uv_script(script_name) {
                    if let Some(script_cmd) = get_script_cmd(&script_value) {
                        let result = invoke_script(pyproject, &script_cmd, verbose);
                        if index == commands.len() - 1 {
                            return result;
                        }
                        index += 1;
                        if let Ok(result) = result {
                            if result.status.success() {
                                if let Some(stdout) = result.stdout {
                                    if verbose {
                                        println!("{}", stdout);
                                    }
                                }
                            } else {
                                std::process::exit(result.status.code().unwrap_or(1));
                            }
                        }
                    }
                }
            }
            std::process::exit(0);
        }
    }
}

pub fn is_executable(path: &Path) -> bool {
    #[cfg(unix)]
    {
        use std::os::unix::prelude::MetadataExt;
        path.metadata().is_ok_and(|x| x.mode() & 0o111 != 0)
    }
    #[cfg(windows)]
    {
        ["com", "exe", "bat", "cmd"]
            .iter()
            .any(|x| path.with_extension(x).is_file())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_task() {
        let tasks = list_tasks().unwrap();
        for task in tasks {
            println!("Task: {} - {}", task.name, task.runner);
        }
    }

    #[test]
    fn test_get_script() {
        let project = PyProjectToml::get_default_project().unwrap();
        let script_value = project.get_uv_script("hello-world").unwrap();
        println!("Script cmd: {:?}", script_value);
    }

    #[test]
    fn test_invoke_cmd_script() {
        let project = PyProjectToml::get_default_project().unwrap();
        let script_value = project.get_uv_script("python-version").unwrap();
        let script = get_script_cmd(&script_value);
        println!("script: {:?}", script);
        invoke_script(&project, &script.unwrap(), true).unwrap();
    }

    #[test]
    fn test_invoke_call_script() {
        let project = PyProjectToml::get_default_project().unwrap();
        let script_value = project.get_uv_script("hello-world").unwrap();
        let script = get_script_cmd(&script_value);
        println!("Script: {:?}", script);
        invoke_script(&project, &script.unwrap(), true).unwrap();
    }

    #[test]
    fn test_invoke_chain_script() {
        let project = PyProjectToml::get_default_project().unwrap();
        let script_value = project.get_uv_script("all").unwrap();
        let script = get_script_cmd(&script_value);
        println!("Script: {:?}", script);
        invoke_script(&project, &script.unwrap(), true).unwrap();
    }

    #[test]
    fn test_run_task() {
        let task_name = "hello";
        run_task(task_name, &[], &[], true).unwrap();
    }
}
