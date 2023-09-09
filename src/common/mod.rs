pub mod pyproject;

use crate::common::pyproject::PyProjectToml;
use crate::errors::KeeperError;
use error_stack::Result;
use error_stack::{IntoReport, ResultExt};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::Path};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct PackageJson {
    pub scripts: Option<HashMap<String, String>>,
    /// (npm|pnpm|yarn)@\d+\.\d+\.\d+(-.+)?
    pub package_manager: Option<String>,
}

pub fn parse_package_json() -> Result<PackageJson, KeeperError> {
    std::env::current_dir()
        .map(|dir| dir.join("package.json"))
        .map(|path| std::fs::read_to_string(path).unwrap_or("{}".to_owned()))
        .map(|data| serde_json::from_str::<PackageJson>(&data).unwrap())
        .into_report()
        .change_context(KeeperError::InvalidPackageJson)
}

pub fn get_npm_command(package_json: &PackageJson) -> &'static str {
    if let Some(package_manager) = &package_json.package_manager {
        return if package_manager.starts_with("yarn") {
            "yarn"
        } else if package_manager.starts_with("pnpm") {
            "pnpm"
        } else if package_manager.starts_with("bun") {
            "bun"
        }else {
            "npm"
        };
    } else {
        if let Ok(dir) = std::env::current_dir() {
            if dir.join("bun.lockb").exists() {
                return "bun";
            } else if dir.join("pnpm-lock.yaml").exists() {
                return "pnpm";
            } else if dir.join("yarn.lock").exists() {
                return "yarn";
            }
        }
    }
    "npm"
}

pub fn pyproject_toml_has_tool(tool_name: &str) -> bool {
    std::env::current_dir()
        .map(|dir| {
            let pyproject_file = dir.join("pyproject.toml");
            pyproject_file.exists()
                && std::fs::read_to_string(pyproject_file)
                .unwrap_or("".to_owned())
                .contains(&format!("[tool.{}", tool_name))
        })
        .unwrap_or(false)
}
