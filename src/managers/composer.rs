use crate::command_utils::{run_command_line, CommandOutput};
use crate::errors::KeeperError;
use error_stack::{IntoReport, Report};
use std::collections::HashMap;
use std::env::current_dir;
use which::which;

pub fn is_available() -> bool {
    current_dir()
        .map(|dir| dir.join("composer.json").exists())
        .unwrap_or(false)
}

pub fn is_command_available() -> bool {
    which("composer").is_ok()
}

pub fn get_task_command_map() -> HashMap<String, String> {
    let mut task_command_map = HashMap::new();
    task_command_map.insert("install".to_string(), "composer install".to_string());
    task_command_map.insert(
        "compile".to_string(),
        "composer check-platform-reqs".to_string(),
    );
    task_command_map.insert("build".to_string(), "composer run-script build".to_string());
    // laravel
    if current_dir()
        .map(|dir| dir.join("artisan").exists())
        .unwrap_or(false)
    {
        task_command_map.insert("start".to_string(), "php artisan serve".to_string());
    } else if current_dir()
        .map(|dir| dir.join("spark").exists())
        .unwrap_or(false)
    {
        // CodeIgniter4
        task_command_map.insert("start".to_string(), "php spark serve".to_string());
    } else {
        task_command_map.insert("start".to_string(), "composer run-script start".to_string());
    }
    task_command_map.insert("test".to_string(), "composer run-script test".to_string());
    task_command_map.insert("deps".to_string(), "composer depends".to_string());
    task_command_map.insert("doc".to_string(), "composer doc".to_string());
    task_command_map.insert("clean".to_string(), "composer clear-cache".to_string());
    task_command_map.insert("outdated".to_string(), "composer outdated".to_string());
    task_command_map.insert("update".to_string(), "composer update".to_string());
    task_command_map.insert("sbom".to_string(), "composer CycloneDX:make-sbom --output-format=json --output-file=application.cdx.json".to_string());
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
            "composer".to_string()
        ).into_report())
    }
}
