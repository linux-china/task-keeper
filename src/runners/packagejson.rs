use std::collections::HashMap;
use error_stack::{IntoReport, Result, ResultExt};
use serde::{Deserialize, Serialize};
use crate::errors::KeeperError;
use crate::models::Task;
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
                    scripts.iter().map(|(name, command)| task!(name, "npm", command)).collect()
                })
                .unwrap_or_else(|| vec![])
        })
        .report()
        .change_context(KeeperError::InvalidPackageJson)
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
}
