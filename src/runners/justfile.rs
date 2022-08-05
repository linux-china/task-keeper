use std::collections::HashMap;
use std::process::Output;
use crate::errors::KeeperError;
use crate::models::Task;
use crate::runners::{run_command, capture_command_output};
use crate::task;
use error_stack::{IntoReport, Result, ResultExt};
use serde::{Deserialize, Serialize};


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JustfileJson {
    pub recipes: HashMap<String, JustRecipe>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JustRecipe {
    pub name: String,
    pub doc: Option<String>,
}

pub fn is_available() -> bool {
    std::env::current_dir()
        .map(|dir| dir.join("justfile").exists() || dir.join("Justfile").exists())
        .unwrap_or(false)
}

pub fn list_tasks() -> Result<Vec<Task>, KeeperError> {
    let json_text = capture_command_output("just", &["--unstable", "--dump", "--dump-format=json"])
        .map(|output| {
            String::from_utf8(output.stdout).unwrap_or("{}".to_owned())
        })
        .unwrap();
    serde_json::from_str::<JustfileJson>(&json_text)
        .map(|justfile_json| {
            justfile_json.recipes
                .iter()
                .map(|(name, recipe)| task!(name, "just", &recipe.doc.clone().unwrap_or("".to_owned())))
                .collect()
        })
        .report()
        .change_context(KeeperError::InvalidJustfile)
}

pub fn run_task(task: &str, extra_args: &[&str], verbose: bool) -> Result<Output, KeeperError> {
    let mut args = vec![];
    args.extend(extra_args);
    args.push(task);
    run_command("just", &args, verbose)
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
        if let Ok(output) = run_task("hello", &["--verbose"], true) {
            let status_code = output.status.code().unwrap_or(0);
            println!("exit code: {}", status_code);
        }
    }
}