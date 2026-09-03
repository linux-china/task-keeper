use crate::command_utils::{run_command, CommandOutput};
use crate::errors::KeeperError;
use crate::models::Task;
use crate::task;
use error_stack::{Report, ResultExt};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use which::which;

#[derive(Serialize, Deserialize, Debug, Default)]
struct JbangCatalogJson {
    pub aliases: Option<HashMap<String, JbangAlias>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JbangAlias {
    #[serde(rename = "script-ref")]
    pub script_ref: String,
    pub description: Option<String>,
}

impl JbangAlias {
    pub fn desc_or_script_ref(&self) -> String {
        self.description.clone().unwrap_or(self.script_ref.clone())
    }
}

pub fn is_available() -> bool {
    std::env::current_dir()
        .map(|dir| dir.join("jbang-catalog.json").exists())
        .unwrap_or(false)
}

pub fn is_command_available() -> bool {
    which("jbang").is_ok()
}

pub fn list_tasks() -> Result<Vec<Task>, Report<KeeperError>> {
    std::env::current_dir()
        .map(|dir| dir.join("jbang-catalog.json"))
        .map(|path| std::fs::read_to_string(path).unwrap_or("{}".to_owned()))
        .map(|data| serde_json::from_str::<JbangCatalogJson>(&data).unwrap())
        .map(|jbang_catalog_json| {
            jbang_catalog_json
                .aliases
                .map(|aliases| {
                    aliases
                        .iter()
                        .map(|(name, alias)| task!(name, "jbang", alias.desc_or_script_ref()))
                        .collect()
                })
                .unwrap_or_else(|| vec![])
        })
        .change_context(KeeperError::InvalidJBangCatalogJson)
}

pub fn run_task(
    task: &str,
    task_args: &[&str],
    global_args: &[&str],
    verbose: bool,
) -> Result<CommandOutput, Report<KeeperError>> {
    let mut args = vec![];
    args.extend(global_args);
    args.push("run");
    args.push(task);
    args.extend(task_args);
    run_command("jbang", &args, verbose)
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
        if let Ok(output) = run_task("Hello", &[], &[], true) {
            let status_code = output.status.code().unwrap_or(0);
            println!("exit code: {}", status_code);
        }
    }
}
