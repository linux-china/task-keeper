use crate::command_utils::{run_command_line, CommandOutput};
use crate::common::{get_npm_command, parse_package_json};
use crate::errors::KeeperError;
use error_stack::{IntoReport, Report};
use std::collections::HashMap;
use which::which;

pub fn is_available() -> bool {
    std::env::current_dir()
        .map(|dir| dir.join("package.json").exists())
        .unwrap_or(false)
}

pub fn is_command_available() -> bool {
    let package_json = parse_package_json().unwrap();
    let package_manager = get_npm_command(&package_json);
    which(package_manager).is_ok()
}

pub fn get_task_command_map() -> HashMap<String, String> {
    let package_json = parse_package_json().unwrap();
    let package_manager_raw = package_json
        .package_manager
        .clone()
        .unwrap_or("npm".to_owned());
    let package_manager = get_npm_command(&package_json);
    let scripts = &package_json.scripts.unwrap_or_else(|| HashMap::new());
    let mut task_command_map = HashMap::new();
    task_command_map.insert(
        "install".to_string(),
        format!("{} install", package_manager),
    );
    if scripts.contains_key("compile") {
        task_command_map.insert(
            "compile".to_string(),
            format!("{} run compile", package_manager),
        );
    }
    if scripts.contains_key("build") {
        task_command_map.insert(
            "build".to_string(),
            format!("{} run build", package_manager),
        );
    }
    if scripts.contains_key("start") {
        task_command_map.insert(
            "start".to_string(),
            format!("{} run start", package_manager),
        );
    }
    if scripts.contains_key("test") {
        task_command_map.insert("test".to_string(), format!("{} run test", package_manager));
    }
    if scripts.contains_key("doc") {
        task_command_map.insert("doc".to_string(), format!("{} run doc", package_manager));
    }
    if scripts.contains_key("clean") {
        task_command_map.insert(
            "clean".to_string(),
            format!("{} run clean", package_manager),
        );
    }
    if package_manager == "bun" {
        task_command_map.insert("deps".to_string(), format!("bun pm ls"));
        if which::which("npm-check").is_ok() {
            task_command_map.insert("outdated".to_string(), "bun outdated".to_string());
        } else {
            task_command_map.insert("outdated".to_string(), format!("npm outdated"));
        }
    } else {
        task_command_map.insert("deps".to_string(), format!("{} list", package_manager));
        task_command_map.insert(
            "outdated".to_string(),
            format!("{} outdated", package_manager),
        );
    }
    task_command_map.insert("update".to_string(), format!("{} update", package_manager));

    if package_manager_raw.starts_with("yarn@3") || package_manager_raw.starts_with("yarn@2") {
        task_command_map.insert("deps".to_string(), "yarn info --dependents".to_string());
        task_command_map.insert(
            "outdated".to_string(),
            "yarn upgrade-interactive".to_string(),
        );
        task_command_map.insert("update".to_string(), "yarn up".to_string());
    } else if package_manager_raw.starts_with("yarn@1") {
        task_command_map.insert("update".to_string(), "yarn upgrade".to_string());
    } else if package_manager == "npm" {
        if which::which("npm-check").is_ok() {
            task_command_map.insert("outdated".to_string(), "npm-check -u".to_string());
        }
    }
    task_command_map.insert("sbom".to_string(), "npx @cyclonedx/cyclonedx-npm -o application.cdx.json".to_string());
    task_command_map
}

pub fn run_task(
    task: &str,
    _task_args: &[&str],
    _global_args: &[&str],
    verbose: bool,
) -> Result<CommandOutput, Report<KeeperError>> {
    if let Some(command_line) = get_task_command_map().get(task) {
        run_command_line(command_line, verbose)
    } else {
        Err(KeeperError::ManagerTaskNotFound(
            task.to_owned(),
            "npm".to_string()
        ).into_report())
    }
}
