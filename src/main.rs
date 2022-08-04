use crate::app::build_app;
use crate::keeper::{run_tasks, list_tasks, RUNNERS};
use colored::Colorize;

mod app;
mod keeper;

fn main() {
    let app = build_app();
    let matches = app.get_matches();
    let verbose = matches.is_present("verbose");

    // list tasks
    if matches.is_present("list") {
        let all_tasks = list_tasks();
        if let Ok(tasks_hashmap) = all_tasks {
            println!("{}", "Available tasks:".bold().green());
            RUNNERS.iter().for_each(|runner| {
                if let Some(tasks) = tasks_hashmap.get(*runner) {
                    println!("{}", format!("  {}:", runner).bold().blue());
                    tasks.iter().for_each(|task| {
                        println!("    -- {}", task);
                    });
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
    // run tasks
    if matches.is_present("tasks") {
        let mut tasks = matches.values_of("tasks").unwrap().collect::<Vec<&str>>();
        let mut extra_args = vec![];
        let double_dash = tasks.iter().position(|x| *x == "--");
        if let Some(index) = double_dash {
            extra_args = tasks.split_off(index)[1..].to_vec();
        }
        let runner = matches.value_of("runner").unwrap_or("just");
        run_tasks(runner, &tasks, &extra_args, verbose).unwrap();
        return;
    }

    // display help message
    build_app().print_help().unwrap();
}
