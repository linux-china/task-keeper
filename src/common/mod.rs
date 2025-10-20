pub mod notification;
pub mod pyproject;

use crate::errors::KeeperError;
use error_stack::{Report, ResultExt};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct PackageJson {
    pub scripts: Option<HashMap<String, String>>,
    /// (npm|pnpm|yarn)@\d+\.\d+\.\d+(-.+)?
    pub package_manager: Option<String>,
}

pub fn parse_package_json() -> core::result::Result<PackageJson, Report<KeeperError>> {
    let content = fs::read_to_string(
        &std::env::current_dir()
            .expect("Failed to get current directory")
            .join("package.json"),
    )
    .unwrap_or_else(|_| "{}".to_owned()); // HACK: if file read fails, fallback to empty json

    serde_json::from_str::<PackageJson>(&content).change_context(KeeperError::InvalidPackageJson)
}

pub fn get_npm_command(package_json: &PackageJson) -> &'static str {
    if let Some(package_manager) = &package_json.package_manager {
        return if package_manager.starts_with("yarn") {
            "yarn"
        } else if package_manager.starts_with("pnpm") {
            "pnpm"
        } else if package_manager.starts_with("bun") {
            "bun"
        } else {
            "npm"
        };
    } else {
        if let Ok(dir) = std::env::current_dir() {
            if dir.join("bun.lockb").exists() || dir.join("bun.lock").exists() {
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
