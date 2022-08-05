use std::collections::HashMap;
use std::process::{Output};
use colored::Colorize;
use error_stack::{report, Result};
use crate::errors::KeeperError;
use crate::models::Task;
use crate::runners;
use crate::runners::RUNNERS;

pub fn run_tasks(cli_runner: &str, task_names: &[&str], extra_args: &[&str], verbose: bool) -> Result<i32, KeeperError> {
    let mut task_count = 0;
    let all_tasks = list_tasks();
    if let Ok(tasks_hashmap) = all_tasks {
        if !cli_runner.is_empty() { //runner is specified
            if let Some(tasks) = tasks_hashmap.get(cli_runner) {
                for task_name in task_names {
                    tasks.iter()
                        .for_each(|task| {
                            if task.name.as_str() == *task_name {
                                task_count += 1;
                                run_task(cli_runner, task, extra_args, verbose).unwrap();
                            }
                        });
                }
            }
        } else { //unknown runner
            for task_name in task_names {
                RUNNERS.iter().for_each(|runner| {
                    if let Some(tasks) = tasks_hashmap.get(*runner) {
                        tasks.iter()
                            .for_each(|task| {
                                if task.name.as_str() == *task_name {
                                    task_count += 1;
                                    run_task(runner, task, extra_args, verbose).unwrap();
                                }
                            });
                    }
                });
            }
        }
    }
    Ok(task_count)
}

pub fn run_task(runner: &str, task: &Task, extra_args: &[&str], verbose: bool) -> Result<Output, KeeperError> {
    let task_name = task.name.as_str();
    println!("{}", format!("[tk] execute {} from {}", task_name, runner).bold().blue());
    match runner {
        "npm" => runners::packagejson::run_task(task_name, extra_args, verbose),
        "just" => runners::justfile::run_task(task_name, extra_args, verbose),
        "fleet" => runners::fleet::run_task(task_name, extra_args, verbose),
        "deno" => runners::denojson::run_task(task_name, extra_args, verbose),
        "make" => runners::makefile::run_task(task_name, extra_args, verbose),
        "rake" => runners::rakefile::run_task(task_name, extra_args, verbose),
        "task" => runners::taskfileyml::run_task(task_name, extra_args, verbose),
        "cargo-make" => runners::makefiletoml::run_task(task_name, extra_args, verbose),
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
    if runners::denojson::is_available() {
        tasks.insert("deno".to_string(), runners::denojson::list_tasks().unwrap());
    }
    if runners::makefile::is_available() {
        tasks.insert("make".to_string(), runners::makefile::list_tasks().unwrap());
    }
    if runners::rakefile::is_available() {
        tasks.insert("rake".to_string(), runners::rakefile::list_tasks().unwrap());
    }
    if runners::taskfileyml::is_available() {
        tasks.insert("task".to_string(), runners::taskfileyml::list_tasks().unwrap());
    }
    if runners::makefiletoml::is_available() {
        tasks.insert("cargo-make".to_string(), runners::makefiletoml::list_tasks().unwrap());
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
