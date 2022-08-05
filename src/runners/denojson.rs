use std::collections::HashMap;
use std::process::Output;
use error_stack::{IntoReport, Result, ResultExt};
use serde::{Deserialize, Serialize};
use crate::errors::KeeperError;
use crate::models::Task;
use crate::runners::run_command;
use crate::task;

#[derive(Serialize, Deserialize, Debug, Default)]
struct DenoJson {
    pub tasks: Option<HashMap<String, String>>,
}

pub fn is_available() -> bool {
    std::env::current_dir()
        .map(|dir| dir.join("deno.json").exists())
        .unwrap_or(false)
}

pub fn list_tasks() -> Result<Vec<Task>, KeeperError> {
    std::env::current_dir()
        .map(|dir| dir.join("deno.json"))
        .map(|path| std::fs::read_to_string(path).unwrap_or("{}".to_owned()))
        .map(|data| serde_json::from_str::<DenoJson>(&data).unwrap())
        .map(|deno_json| {
            deno_json.tasks
                .map(|scripts| {
                    scripts.iter()
                        .map(|(name, command)| task!(name, "deno", command))
                        .collect()
                })
                .unwrap_or_else(|| vec![])
        })
        .report()
        .change_context(KeeperError::InvalidPackageJson)
}

pub fn run_task(task: &str, extra_args: &[&str], verbose: bool) -> Result<Output, KeeperError> {
    let mut args = vec!["task"];
    args.extend(extra_args);
    args.push(task);
    run_command("deno", &args, verbose)
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
        if let Ok(output) = run_task("first", &[], true) {
            let status_code = output.status.code().unwrap_or(0);
            println!("exit code: {}", status_code);
        }
    }
}
