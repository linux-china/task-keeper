use std::collections::HashMap;
use std::process::Output;
use error_stack::{IntoReport, Result, ResultExt};
use serde::{Deserialize, Serialize};
use crate::errors::KeeperError;
use crate::models::Task;
use crate::task;
use which::which;
use crate::command_utils::run_command;

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
struct ComposerJson {
    pub scripts: Option<HashMap<String, serde_json::value::Value>>,
}

pub fn is_available() -> bool {
    std::env::current_dir()
        .map(|dir| dir.join("composer.json").exists())
        .unwrap_or(false)
}

pub fn is_command_available() -> bool {
    which("composer").is_ok()
}

pub fn list_tasks() -> Result<Vec<Task>, KeeperError> {
    parse_composer_json()
        .map(|composer_json| {
            composer_json.scripts
                .map(|scripts| {
                    scripts.iter()
                        .filter(|(name, _)| !name.starts_with("pre-") && !name.starts_with("post-"))
                        .map(|(name, command)| {
                            let mut desc = command.to_string();
                            if desc.starts_with('"') {
                                desc = desc[1..desc.len() - 1].to_string();
                            }
                            task!(name, "composer",desc)
                        })
                        .collect()
                })
                .unwrap_or_else(|| vec![])
        })
}

fn parse_composer_json() -> Result<ComposerJson, KeeperError> {
    std::env::current_dir()
        .map(|dir| dir.join("composer.json"))
        .map(|path| std::fs::read_to_string(path).unwrap_or("{}".to_owned()))
        .map(|data| serde_json::from_str::<ComposerJson>(&data).unwrap())
        .into_report()
        .change_context(KeeperError::InvalidComposerJson)
}

pub fn run_task(task: &str, task_args: &[&str], global_args: &[&str], verbose: bool) -> Result<Output, KeeperError> {
    let mut args = vec![];
    args.extend(global_args);
    args.push("run-script");
    args.push(task);
    args.extend(task_args);
    run_command("composer", &args, verbose)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_trim() {
        let s = "\"foo\"";
        let trim = s[1..s.len() - 1].to_string();
        println!("{}", trim);
    }

    #[test]
    fn test_parse() {
        if let Ok(tasks) = list_tasks() {
            println!("{:?}", tasks);
        }
    }

    #[test]
    fn test_run() {
        if let Ok(output) = run_task("my-ip", &[], &[],true) {
            let status_code = output.status.code().unwrap_or(0);
            println!("exit code: {}", status_code);
        }
    }
}
