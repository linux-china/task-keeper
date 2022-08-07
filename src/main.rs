use crate::app::build_app;
use crate::keeper::{run_tasks, list_tasks};
use colored::Colorize;
use crate::runners::RUNNERS;
use dotenv::dotenv;
use std::collections::HashSet;

mod app;
mod keeper;
mod errors;
mod models;
mod runners;

fn main() {
    let app = build_app();
    let matches = app.get_matches();
    let verbose = matches.is_present("verbose");
    let no_dotenv = matches.is_present("no-dotenv");

    // summary to list all task names
    if matches.is_present("summary") {
        let mut task_names: HashSet<String> = HashSet::new();
        let all_tasks = list_tasks();
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
        let all_tasks = list_tasks();
        if let Ok(tasks_hashmap) = all_tasks {
            println!("{}", "Available tasks:".bold().green());
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
        } else {
            println!("[tk] no tasks found");
        }
        return;
    }
    // migrate tasks
    if matches.is_present("from") && matches.is_present("to") {
        println!("{}", "migrate tasks");
        return;
    }
    // load .env for tasks
    if !no_dotenv {
        dotenv().ok();
    }
    // run tasks
    if matches.is_present("tasks") {
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
