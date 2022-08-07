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
        "invoke" => runners::taskspy::run_task(task_name, extra_args, verbose),
        "cargo-make" => runners::makefiletoml::run_task(task_name, extra_args, verbose),
        "procfile" => runners::procfile::run_task(task_name, extra_args, verbose),
        "composer" => runners::composer::run_task(task_name, extra_args, verbose),
        "markdown" => runners::markdown::run_task(task_name, extra_args, verbose),
        "shell" => runners::taskshell::run_task(task_name, extra_args, verbose),
        _ => Err(report!(KeeperError::FailedToRunTasks(format!("unknown runner: {}", runner)))),
    }
}

pub fn list_tasks() -> Result<HashMap<String, Vec<Task>>, KeeperError> {
    let mut tasks = HashMap::new();
    if runners::fleet::is_available() {
        tasks.insert("fleet".to_string(), runners::fleet::list_tasks().unwrap());
    }
    if runners::procfile::is_available() {
        tasks.insert("procfile".to_string(), runners::procfile::list_tasks().unwrap());
    }
    if runners::markdown::is_available() {
        tasks.insert("markdown".to_string(), runners::markdown::list_tasks().unwrap());
    }
    if runners::taskshell::is_available() {
        tasks.insert("shell".to_string(), runners::taskshell::list_tasks().unwrap());
    }
    if runners::justfile::is_available() {
        if runners::justfile::is_command_available() {
            tasks.insert("just".to_string(), runners::justfile::list_tasks().unwrap());
        } else {
            println!("{}", format!("[tk] just(https://github.com/casey/just) command not available for justfile").bold().red());
        }
    }
    if runners::packagejson::is_available() {
        if runners::packagejson::is_command_available() {
            tasks.insert("npm".to_string(), runners::packagejson::list_tasks().unwrap());
        } else {
            println!("{}", format!("[tk] npm(https://nodejs.org) command not available for package.json").bold().red());
        }
    }
    if runners::denojson::is_available() {
        if runners::denojson::is_command_available() {
            tasks.insert("deno".to_string(), runners::denojson::list_tasks().unwrap());
        } else {
            println!("{}", format!("[tk] deno(https://deno.land) command not available for deno.json").bold().red());
        }
    }
    if runners::makefile::is_available() {
        if runners::makefile::is_command_available() {
            tasks.insert("make".to_string(), runners::makefile::list_tasks().unwrap());
        } else {
            println!("{}", format!("[tk] make(https://www.gnu.org/software/make) command not available for makefile").bold().red());
        }
    }
    if runners::rakefile::is_available() {
        if runners::rakefile::is_command_available() {
            tasks.insert("rake".to_string(), runners::rakefile::list_tasks().unwrap());
        } else {
            println!("{}", format!("[tk] rake(https://ruby.github.io/rake/) command not available for rakefile").bold().red());
        }
    }
    if runners::taskfileyml::is_available() {
        if runners::taskfileyml::is_command_available() {
            tasks.insert("task".to_string(), runners::taskfileyml::list_tasks().unwrap());
        } else {
            println!("{}", format!("[tk] task(https://taskfile.dev) command not available for Taskfile.yml").bold().red());
        }
    }
    if runners::makefiletoml::is_available() {
        if runners::makefiletoml::is_command_available() {
            tasks.insert("cargo-make".to_string(), runners::makefiletoml::list_tasks().unwrap());
        } else {
            println!("{}", format!("[tk] cargo-make(https://github.com/sagiegurari/cargo-make) command not available for Makefile.toml").bold().red());
        }
    }
    if runners::taskspy::is_available() {
        if runners::taskspy::is_command_available() {
            tasks.insert("invoke".to_string(), runners::taskspy::list_tasks().unwrap());
        } else {
            println!("{}", format!("[tk] invoke(https://www.pyinvoke.org) command not available for tasks.py").bold().red());
        }
    }
    if runners::composer::is_available() {
        if runners::composer::is_command_available() {
            tasks.insert("composer".to_string(), runners::composer::list_tasks().unwrap());
        } else {
            println!("{}", format!("[tk] composer(https://getcomposer.org/) command not available for composer.json").bold().red());
        }
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
