use crate::app::build_app;
use crate::keeper::{run_tasks, list_all_runner_tasks};
use colored::Colorize;
use crate::runners::RUNNERS;
use dotenv::dotenv;
use std::collections::HashSet;
use std::io::Write;
use std::path::Path;

mod app;
mod keeper;
mod errors;
mod models;
mod runners;
mod managers;
mod command_utils;

fn main() {
    let app = build_app();
    let matches = app.get_matches();
    let verbose = matches.is_present("verbose");
    let no_dotenv = matches.is_present("no-dotenv");

    // summary to list all task names
    if matches.is_present("summary") {
        let mut task_names: HashSet<String> = HashSet::new();
        let all_tasks = list_all_runner_tasks();
        if let Ok(tasks_hashmap) = all_tasks {
            RUNNERS.iter().for_each(|runner| {
                if let Some(tasks) = tasks_hashmap.get(*runner) {
                    tasks.iter().for_each(|task| {
                        task_names.insert(task.name.clone());
                    });
                }
            });
        }
        println!("{}", task_names.into_iter().collect::<Vec<String>>().join(" "));
        return;
    }
    // list tasks
    if matches.is_present("list") {
        let mut task_found = false;
        let all_tasks = list_all_runner_tasks();
        if let Ok(tasks_hashmap) = all_tasks {
            if !tasks_hashmap.is_empty() {
                task_found = true;
                println!("{}", "Available task runners:".bold().green());
                RUNNERS.iter().for_each(|runner| {
                    if let Some(tasks) = tasks_hashmap.get(*runner) {
                        if !tasks.is_empty() {
                            println!("{}", format!("  {}: {}", runner, runners::get_runner_file_name(runner)).bold().blue());
                            tasks.iter().for_each(|task| {
                                if task.description.is_empty() {
                                    println!("    -- {}", task.name.bold());
                                } else {
                                    println!("    -- {} : {}", task.name.bold(), format_description(&task.description));
                                }
                            });
                        }
                    }
                });
            }
        }
        let managers = managers::get_available_managers();
        if !managers.is_empty() {
            task_found = true;
            println!("{}", "Available project/package management tools:".bold().green());
            managers.into_iter().for_each(|manager_name| {
                println!("{}", format!("  {}: {}", manager_name, managers::get_manager_file_name(&manager_name)).bold().blue());
                let task_command_map = managers::get_manager_command_map(&manager_name);
                if !task_command_map.is_empty() {
                    task_command_map.into_iter().for_each(|(task_name, command_line)| {
                        if &task_name != "init" {
                            println!("    -- {} : {}", task_name.bold(), command_line);
                        }
                    });
                }
            });
        }
        if !task_found {
            println!("{}", "No task runner or project management tool found!".bold().red());
        }
        return;
    }
    /*// display runner help and tasks
    if matches.is_present("runner") && !matches.is_present("tasks") {
        let runner = matches.value_of("runner").unwrap();
        let mut intro = "";
        let mut usage = "";
        println!("{}", format!("{}: ", runner).bold().blue());
        println!("{}", format!("{}: ", runner).bold().blue());
        // match item in runners
        let tasks = match runner {
            "npm" => {
                runners::packagejson::list_tasks()
            }
            "just" => { runners::justfile::list_tasks() }
            "fleet" => { runners::fleet::list_tasks() }
            "deno" => { runners::denojson::list_tasks() }
            "make" => { runners::makefile::list_tasks() }
            "rake" => { runners::rakefile::list_tasks() }
            "task" => { runners::taskfileyml::list_tasks() }
            "invoke" => { runners::taskspy::list_tasks() }
            "cargo-make" => { runners::makefiletoml::list_tasks() }
            "procfile" => { runners::procfile::list_tasks() }
            "composer" => { runners::composer::list_tasks() }
            "markdown" => { runners::markdown::list_tasks() }
            "shell" => { runners::taskshell::list_tasks() }
            "mave" => { runners::taskshell::list_tasks() }
            "gradle" => { runners::taskshell::list_tasks() }
            "sbt" => { runners::taskshell::list_tasks() }
            "cargo" => {
                intro = "Rust's package manager";
                usage = "cargo [+toolchain] [OPTIONS] [SUBCOMMAND]";
                runners::taskshell::list_tasks()
            }
            "bundle" => { runners::taskshell::list_tasks() }
            _ => vec![]
        };
    }*/

    // migrate tasks
    if matches.is_present("from") && matches.is_present("to") {
        println!("{}", "migrate tasks");
        return;
    }
    // create task file by runner
    if matches.is_present("init") {
        let runner_name = matches.value_of("init").unwrap();
        if runner_name == "shell" {
            let exists = Path::new("./task.sh").exists();
            if !exists {
                let mut tasksh_file = std::fs::File::create("./task.sh").unwrap();
                let bytes = include_bytes!("./templates/task.sh");
                tasksh_file.write_all(bytes).unwrap();
                set_executable("./task.sh");
                println!("{}", "task.sh created".bold().green());
            } else {
                println!("{}", "[tk] task.sh already exists".bold().red());
            }
        } else {
            println!("[tk] Create task file for {} not support now.", runner_name);
        }
        return;
    }
    // run tasks
    if matches.is_present("tasks") {
        // load .env for tasks
        if !no_dotenv {
            dotenv().ok();
        }
        let mut tasks = matches.values_of("tasks").unwrap().collect::<Vec<&str>>();
        let mut extra_args = vec![];
        let double_dash = tasks.iter().position(|x| *x == "--");
        if let Some(index) = double_dash {
            extra_args = tasks.split_off(index)[1..].to_vec();
        }
        let runner = matches.value_of("runner").unwrap_or("");
        let task_count = run_tasks(runner, &tasks, &extra_args, verbose).unwrap();
        if task_count == 0 { // no tasks executed
            if runners::makefile::is_available() { // try Makefile
                for task in tasks {
                    runners::makefile::run_task(task, &extra_args, verbose).unwrap();
                }
            } else {
                println!("{}", "[tk] no tasks found".bold().red());
            }
        }
        return;
    }

    // display help message
    build_app().print_help().unwrap();
}

fn format_description(description: &str) -> String {
    let mut short_desc = description.to_string();
    if description.contains("\n") {
        short_desc = description.split("\n").next().unwrap().to_string();
        short_desc = format!("{} ...", short_desc);
    }
    if short_desc.len() > 60 {
        short_desc = format!("{} ...", &short_desc[0..60]);
    }
    return short_desc;
}

#[cfg(target_family = "unix")]
fn set_executable(path: &str) {
    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o755)).unwrap();
}

#[cfg(not(target_family = "unix"))]
fn set_executable(path: &str) {}
