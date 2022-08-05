use std::collections::HashMap;
use std::process::{Command, Output, Stdio};
use error_stack::{IntoReport, report, Result, ResultExt};
use crate::errors::KeeperError;
use crate::models::Task;
use crate::runners;

pub const RUNNERS: &'static [&'static str] = &["make", "npm", "deno", "just"];

pub fn run_tasks(cli_runner: &str, task_names: &[&str], extra_args: &[&str], verbose: bool) -> Result<(), KeeperError> {
    let all_tasks = list_tasks();
    if let Ok(tasks_hashmap) = all_tasks {
        if !cli_runner.is_empty() { //runner is specified
            if let Some(tasks) = tasks_hashmap.get(cli_runner) {
                tasks.iter()
                    .for_each(|task| {
                        let task_name = task.name.as_str();
                        if task_names.contains(&task_name) {
                            run_task(cli_runner, task, extra_args, verbose).unwrap();
                        }
                    });
            }
        } else { //unknown runner
            RUNNERS.iter().for_each(|runner| {
                if let Some(tasks) = tasks_hashmap.get(*runner) {
                    tasks.iter()
                        .for_each(|task| {
                            let task_name = task.name.as_str();
                            if task_names.contains(&task_name) {
                                run_task(runner, task, extra_args, verbose).unwrap();
                            }
                        });
                }
            });
        }
    } else {
        println!("[tk] no tasks found");
    }
    Ok(())
}

pub fn run_task(runner: &str, task: &Task, extra_args: &[&str], verbose: bool) -> Result<Output, KeeperError> {
    let task_name = task.name.as_str();
    match runner {
        "npm" => runners::packagejson::run_task(task_name, extra_args, verbose),
        "just" => runners::justfile::run_task(task_name, extra_args, verbose),
        "fleet" => runners::fleet::run_task(task_name, extra_args, verbose),
        _ => Err(report!(KeeperError::FailedToRunTasks(format!("unknown runner: {}", runner)))),
    }
}

pub fn list_tasks() -> Result<HashMap<String, Vec<Task>>, KeeperError> {
    let mut tasks = HashMap::new();
    if runners::fleet::is_available() {
        tasks.insert("fleet".to_string(), runners::fleet::list_tasks().unwrap());
    }
    if runners::justfile::is_available() {
        tasks.insert("just".to_string(), runners::justfile::list_tasks().unwrap());
    }
    if runners::packagejson::is_available() {
        tasks.insert("npm".to_string(), runners::packagejson::list_tasks().unwrap());
    }
    Ok(tasks)
}

#[cfg(test)]
mod tests {
    use crate::task;
    use super::*;

    #[test]
    fn test_run_task() {
        let task = task!("start", "npm");
        if let Ok(output) = run_task("npm", &task, &[], true) {
            let status_code = output.status.code().unwrap_or(0);
            println!("exit code: {}", status_code);
        }
    }
}
