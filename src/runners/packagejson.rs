use error_stack::Report;
use crate::command_utils::{run_command, CommandOutput};
use crate::common::{get_npm_command, parse_package_json};
use crate::errors::KeeperError;
use crate::models::Task;
use crate::task;
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

pub fn list_tasks() -> Result<Vec<Task>, Report<KeeperError>> {
    parse_package_json().map(|package_json| {
        package_json
            .scripts
            .map(|scripts| {
                scripts
                    .iter()
                    .filter(|(name, _)| !name.starts_with("pre") && !name.starts_with("post"))
                    .map(|(name, command)| task!(name, "npm", command))
                    .collect()
            })
            .unwrap_or_else(|| vec![])
    })
}

pub fn run_task(
    task: &str,
    task_args: &[&str],
    global_args: &[&str],
    verbose: bool,
) -> Result<CommandOutput, Report<KeeperError>> {
    let package_json = parse_package_json()?;
    let command_name = get_npm_command(&package_json);
    let mut args = vec![];
    args.extend(global_args);
    args.push("run");
    args.push(task);
    args.extend(task_args);
    run_command(command_name, &args, verbose)
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
        if let Ok(output) = run_task("start", &["--verbose"], &[], true) {
            let status_code = output.status.code().unwrap_or(0);
            println!("exit code: {}", status_code);
        }
    }
}
