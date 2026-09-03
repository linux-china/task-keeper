use error_stack::Report;
use crate::command_utils::{capture_command_output, run_command, CommandOutput};
use crate::errors::KeeperError;
use crate::models::Task;
use crate::task;
use regex::Regex;
use which::which;

pub fn is_available() -> bool {
    std::env::current_dir()
        .map(|dir| dir.join("Taskfile.yml").exists() || dir.join("Taskfile.yaml").exists())
        .unwrap_or(false)
}

pub fn is_command_available() -> bool {
    get_go_task_command().is_some()
}

pub fn list_tasks() -> Result<Vec<Task>, Report<KeeperError>> {
    let makefile_meta_text =
        capture_command_output(&get_go_task_command().unwrap(), &["--list-all"])
            .map(|output| String::from_utf8(output.stdout).unwrap_or("{}".to_owned()))?;
    let re = Regex::new(r"^\* [a-zA-Z0-9._-]+:.*").unwrap();
    let tasks: Vec<Task> = makefile_meta_text
        .lines()
        .filter(|line| re.is_match(line))
        .flat_map(|line| {
            let mut parts = line.splitn(2, ":");
            let mut task_name = parts.next().unwrap().trim().to_owned();
            if task_name.starts_with("* ") {
                task_name = task_name.split_off(2);
            }
            let description = parts.next().unwrap_or("").trim().to_owned();
            let mut tasks: Vec<Task> = vec![];
            if description.contains("(aliases:") {
                let offset = description.find("(aliases:").unwrap();
                let task_description = description[..offset].trim().to_owned();
                tasks.push(task!(task_name, "task", task_description));
                let aliases = description[offset + 9..description.len() - 1].trim();
                for alias in aliases.split(",") {
                    tasks.push(task!(
                        alias.trim(),
                        "task",
                        format!("Alias for {}", task_name)
                    ));
                }
            } else {
                tasks.push(task!(task_name, "task", description));
            }
            tasks
        })
        .collect();
    Ok(tasks)
}

pub fn run_task(
    task: &str,
    task_args: &[&str],
    global_args: &[&str],
    verbose: bool,
) -> Result<CommandOutput, Report<KeeperError>> {
    let mut args = vec![];
    args.extend(global_args);
    args.push(task);
    args.extend(task_args);
    run_command(&get_go_task_command().unwrap(), &args, verbose)
}

fn get_go_task_command() -> Option<String> {
    if let Ok(path) = which("go-task") {
        Some(path.to_str().unwrap().to_owned())
    } else if let Ok(path) = which("task") {
        Some(path.to_str().unwrap().to_owned())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_which_task() {
        if let Some(path) = get_go_task_command() {
            println!("{:?}", path);
        }
    }

    #[test]
    fn test_parse() {
        if let Ok(tasks) = list_tasks() {
            println!("{:?}", tasks);
        }
    }

    #[test]
    fn test_run() {
        if let Ok(output) = run_task("hello", &[], &[], true) {
            let status_code = output.status.code().unwrap_or(0);
            println!("exit code: {}", status_code);
        }
    }
}
