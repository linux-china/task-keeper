use crate::command_utils::{CommandOutput, run_command_with_env_vars};
use crate::common::pyproject::{PyProjectToml, python_command};
use crate::common::pyproject_toml_has_tool;
use crate::errors::KeeperError;
use crate::models::Task;
use crate::task;
use error_stack::{IntoReport, Report, ResultExt};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use toml::Value;
use which::which;

/// implement feature from https://rye.astral.sh/guide/pyproject/#toolryescripts
pub fn is_available() -> bool {
    pyproject_toml_has_tool("rye")
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
    task_args: &[&str],
    global_args: &[&str],
    verbose: bool,
) -> Result<CommandOutput, Report<KeeperError>> {
    let project = PyProjectToml::get_default_project()
        .change_context(KeeperError::FailedToRunTasks("failed to parse pyproject.toml".to_owned()))?;
    let script = find_script(&project, task)?;
    invoke_script(&project, &script, &[task.to_owned()], task_args, global_args, verbose)
}

/// Find script by name from `[tool.rye.scripts]`
fn find_script(pyproject: &PyProjectToml, script_name: &str) -> Result<Script, Report<KeeperError>> {
    let script_value = pyproject
        .get_uv_script(script_name)
        .ok_or_else(|| KeeperError::TaskNotFound(script_name.to_owned()).into_report())?;
    get_script_cmd(&script_value).ok_or_else(|| {
        KeeperError::FailedToRunTasks(format!("invalid script definition: {}", script_name))
            .into_report()
    })
}

fn script_error(message: &str) -> Report<KeeperError> {
    KeeperError::FailedToRunTasks(message.to_owned()).into_report()
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
            let command_and_args = shlex::split(cmd_text)?;
            Some(Script::Cmd(command_and_args, HashMap::new(), None))
        }
        Value::Array(arr) => {
            let command_and_args: Vec<String> = arr
                .iter()
                .map(|item| item.to_string().trim_matches(&['"', '\'']).to_string())
                .collect();
            Some(Script::Cmd(command_and_args, HashMap::new(), None))
        }
        Value::Table(table) => {
            // relative to the project root, and loaded when the script runs
            let env_file: EnvFile = table.get("env-file").and_then(Value::as_str).map(PathBuf::from);
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
                        let command_and_args = shlex::split(cmd_text)?;
                        Some(Script::Cmd(command_and_args, env_hash_map, env_file))
                    }
                    Value::Array(arr) => {
                        let command_and_args: Vec<String> = arr
                            .iter()
                            .map(|item| item.to_string().trim_matches(&['"', '\'']).to_string())
                            .collect();
                        Some(Script::Cmd(command_and_args, env_hash_map, env_file))
                    }
                    _ => None,
                }
            } else if let Some(call) = table.get("call") {
                match call {
                    Value::String(call_text) => {
                        let callable = call_text.to_string();
                        return Some(Script::Call(callable, env_hash_map, env_file));
                    }
                    _ => None,
                }
            } else if let Some(chain) = table.get("chain") {
                match chain {
                    Value::Array(chain_arr) => {
                        let commands: Vec<Vec<String>> = chain_arr
                            .iter()
                            .filter_map(|item| match item {
                                Value::Array(arr) => Some(
                                    arr.iter()
                                        .map(|v| v.as_str().map(str::to_owned).unwrap_or_else(|| v.to_string()))
                                        .collect::<Vec<String>>(),
                                ),
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

/// Environment variables for the script's process only, so they never leak into the next tasks:
/// variables from `env-file`, overridden by `env`, the same as rye.
fn script_env_vars(env_vars: &EnvVars, env_file: &EnvFile) -> Result<Option<EnvVars>, Report<KeeperError>> {
    let mut all_env_vars = EnvVars::new();
    if let Some(env_file_path) = env_file {
        let env_file_path = std::env::current_dir()
            .map(|dir| dir.join(env_file_path))
            .unwrap_or_else(|_| env_file_path.clone());
        let entries = dotenvx_rs::dotenvx::from_path_iter(&env_file_path).change_context(
            KeeperError::FailedToRunTasks(format!("failed to load env file: {}", env_file_path.display())),
        )?;
        all_env_vars.extend(entries);
    }
    all_env_vars.extend(env_vars.iter().map(|(k, v)| (k.clone(), v.clone())));
    Ok(if all_env_vars.is_empty() { None } else { Some(all_env_vars) })
}

/// `chain` holds the names of the scripts being invoked, outermost first, to detect a recursive chain.
fn invoke_script(
    pyproject: &PyProjectToml,
    script: &Script,
    chain: &[String],
    task_args: &[&str],
    global_args: &[&str],
    verbose: bool,
) -> Result<CommandOutput, Report<KeeperError>> {
    match script {
        Script::Call(entry, env_vars, env_file) => {
            let args: Vec<String> = if let Some((module, func)) = entry.split_once(':') {
                if module.is_empty() || func.is_empty() {
                    return Err(script_error(
                        "Python callable must be in the form <module_name>:<callable_name> or <module_name>",
                    ));
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
            let env_vars = script_env_vars(env_vars, env_file)?;
            let real_args: Vec<&str> = args.iter().map(String::as_str).collect();
            let py = pyproject.venv_python_path();
            run_command_with_env_vars(&py.to_string_lossy(), &real_args, &None, &env_vars, verbose)
        }
        Script::Cmd(script_args, env_vars, env_file) => {
            if script_args.is_empty() {
                return Err(script_error("script has no arguments"));
            }
            let env_vars = script_env_vars(env_vars, env_file)?;
            let script_target = std::env::current_dir().unwrap().join(&script_args[0]);
            if script_target.exists() && script_target.is_file() {
                let args: Vec<&str> = script_args.into_iter().map(String::as_str).collect();
                let mut real_args: Vec<&str> = vec![];
                real_args.extend(global_args);
                real_args.extend(args);
                real_args.extend(task_args);
                run_command_with_env_vars(python_command(), &real_args, &None, &env_vars, verbose)
            } else {
                let args: Vec<&str> = script_args[1..].iter().map(String::as_str).collect();
                let mut real_args: Vec<&str> = vec![];
                real_args.extend(global_args);
                real_args.extend(args);
                real_args.extend(task_args);
                let command_name = &script_args[0];
                run_command_with_env_vars(command_name, &real_args, &None, &env_vars, verbose)
            }
        }
        Script::Chain(commands) => {
            if commands.is_empty() {
                return Err(script_error("Please supply at least one command to chain"));
            }
            let mut last_output = None;
            for command_and_args in commands {
                let script_name = command_and_args
                    .first()
                    .ok_or_else(|| script_error("empty command in chain"))?;
                if chain.contains(script_name) {
                    return Err(script_error(&format!(
                        "recursive chain: {} -> {}",
                        chain.join(" -> "),
                        script_name
                    )));
                }
                let script_cmd = find_script(pyproject, script_name)?;
                let mut sub_chain = chain.to_vec();
                sub_chain.push(script_name.clone());
                let output = invoke_script(pyproject, &script_cmd, &sub_chain, task_args, global_args, verbose)?;
                // stop the chain on first failure, and let caller report the exit code
                if !output.status.success() {
                    return Ok(output);
                }
                if verbose {
                    if let Some(stdout) = &output.stdout {
                        println!("{}", stdout);
                    }
                }
                last_output = Some(output);
            }
            Ok(last_output.unwrap())
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
        invoke_script(&project, &script.unwrap(), &[], &[], &[], true).unwrap();
    }

    #[test]
    #[ignore]
    fn test_invoke_call_script() {
        let project = PyProjectToml::get_default_project().unwrap();
        let script_value = project.get_uv_script("hello-world").unwrap();
        let script = get_script_cmd(&script_value);
        println!("Script: {:?}", script);
        invoke_script(&project, &script.unwrap(), &[], &[], &[], true).unwrap();
    }

    #[test]
    #[ignore]
    fn test_invoke_chain_script() {
        let project = PyProjectToml::get_default_project().unwrap();
        let script_value = project.get_uv_script("all").unwrap();
        let script = get_script_cmd(&script_value);
        println!("Script: {:?}", script);
        invoke_script(&project, &script.unwrap(), &[], &[], &[], true).unwrap();
    }

    fn parse_project(text: &str) -> PyProjectToml {
        toml::from_str(text).unwrap()
    }

    #[test]
    fn test_parse_env_file() {
        let project = parse_project(
            r#"
[tool.rye.scripts]
serve = { cmd = "flask run", env-file = ".env.dev", env = { PORT = "8000" } }
"#,
        );
        match get_script_cmd(&project.get_uv_script("serve").unwrap()) {
            Some(Script::Cmd(_, env_vars, env_file)) => {
                assert_eq!(env_file, Some(PathBuf::from(".env.dev")));
                assert_eq!(env_vars.get("PORT").map(String::as_str), Some("8000"));
            }
            other => panic!("unexpected script: {:?}", other),
        }
    }

    #[test]
    fn test_script_env_vars() {
        let dir = tempfile::tempdir().unwrap();
        let env_file = dir.path().join(".env.test");
        std::fs::write(&env_file, "FROM_FILE=file\nPORT=1\n").unwrap();
        let env_vars = EnvVars::from([("PORT".to_owned(), "8000".to_owned())]);
        // an absolute path is kept by `join`
        let all_env_vars = script_env_vars(&env_vars, &Some(env_file)).unwrap().unwrap();
        assert_eq!(all_env_vars.get("FROM_FILE").map(String::as_str), Some("file"));
        // `env` overrides the env file
        assert_eq!(all_env_vars.get("PORT").map(String::as_str), Some("8000"));
        // nothing is set on tk's own process
        assert!(std::env::var("FROM_FILE").is_err());
        assert!(script_env_vars(&EnvVars::new(), &None).unwrap().is_none());
        assert!(script_env_vars(&EnvVars::new(), &Some(dir.path().join("missing"))).is_err());
    }

    #[test]
    #[cfg(unix)]
    fn test_env_not_leaked() {
        let project = parse_project(
            r#"
[tool.rye.scripts]
with-env = { cmd = ["sh", "-c", "test \"$TK_UV_SCRIPT_ENV\" = yes"], env = { TK_UV_SCRIPT_ENV = "yes" } }
"#,
        );
        let script = get_script_cmd(&project.get_uv_script("with-env").unwrap()).unwrap();
        let output = invoke_script(&project, &script, &[], &[], &[], false).unwrap();
        assert!(output.status.success());
        assert!(std::env::var("TK_UV_SCRIPT_ENV").is_err());
    }

    #[test]
    fn test_recursive_chain() {
        let project = parse_project(
            r#"
[tool.rye.scripts]
self-loop = { chain = ["self-loop"] }
a = { chain = ["b"] }
b = { chain = ["a"] }
"#,
        );
        for (name, expected) in [("self-loop", "self-loop -> self-loop"), ("a", "a -> b -> a")] {
            let script = get_script_cmd(&project.get_uv_script(name).unwrap()).unwrap();
            let Err(err) = invoke_script(&project, &script, &[name.to_owned()], &[], &[], false) else {
                panic!("recursive chain {} should fail", name);
            };
            let message = format!("{:?}", err);
            assert!(message.contains(expected), "{}", message);
        }
    }

    #[test]
    fn test_run_task() {
        let task_name = "hello";
        run_task(task_name, &[], &[], true).unwrap();
    }
}
