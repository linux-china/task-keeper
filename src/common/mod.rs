use std::collections::HashMap;
use error_stack::{IntoReport, ResultExt};
use serde::{Deserialize, Serialize};
use crate::errors::KeeperError;
use error_stack::{Result};

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

pub fn get_package_command(package_json: &PackageJson) -> &'static str {
    if let Some(package_manager) = &package_json.package_manager {
        return if package_manager.starts_with("yarn") {
            "yarn"
        } else if package_manager.starts_with("pnpm") {
            "pnpm"
        } else {
            "npm"
        };
    }
    "npm"
}
