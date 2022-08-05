use std::collections::HashMap;
use std::process::Output;
use error_stack::{IntoReport, Result, ResultExt};
use serde::{Deserialize, Serialize};
use crate::errors::KeeperError;
use crate::models::Task;
use crate::runners::run_command;
use crate::task;


#[derive(Serialize, Deserialize, Debug, Default)]
struct PackageJson {
    pub scripts: Option<HashMap<String, String>>,
}

pub fn is_available() -> bool {
    std::env::current_dir()
        .map(|dir| dir.join("package.json").exists())
        .unwrap_or(false)
}

pub fn list_tasks() -> Result<Vec<Task>, KeeperError> {
    std::env::current_dir()
        .map(|dir| dir.join("package.json"))
        .map(|path| std::fs::read_to_string(path).unwrap_or("{}".to_owned()))
        .map(|data| serde_json::from_str::<PackageJson>(&data).unwrap())
        .map(|package_json| {
            package_json.scripts
                .map(|scripts| {
                    scripts.iter()
                        .filter(|(name, _)| !name.starts_with("pre") && !name.starts_with("post"))
                        .map(|(name, command)| task!(name, "npm", command))
                        .collect()
                })
                .unwrap_or_else(|| vec![])
        })
        .report()
        .change_context(KeeperError::InvalidPackageJson)
}

pub fn run_task(task: &str, extra_args: &[&str], verbose: bool) -> Result<Output, KeeperError> {
    let mut args = vec!["run"];
    args.extend(extra_args);
    args.push(task);
    run_command("npm", &args, verbose)
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
        if let Ok(output) = run_task("start", &["--verbose"], true) {
            let status_code = output.status.code().unwrap_or(0);
            println!("exit code: {}", status_code);
        }
    }
}
