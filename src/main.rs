use crate::app::build_app;
use crate::keeper::{run_tasks, list_all_runner_tasks};
use colored::Colorize;
use crate::runners::RUNNERS;
use dotenv::dotenv;
use std::collections::HashSet;
use std::io::Write;
use std::path::Path;
use crate::models::TaskContext;

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
    // check your system for potential problems to run tasks
    if matches.is_present("doctor") {
        diagnose();
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
        println!("{}", "Task migration has not yet been implemented!".bold().red());
        return;
    }
    // create task file by runner
    if matches.is_present("init") {
        let runner_name = matches.value_of("init").unwrap();
        if runner_name == "shell" {
            let exists = Path::new("./task.sh").exists();
            if !exists {
                let mut tasksh_file = std::fs::File::create("task.sh").unwrap();
                let bytes = include_bytes!("./templates/task.sh");
                tasksh_file.write_all(bytes).unwrap();
                set_executable("task.sh");
                println!("{}", "task.sh created".bold().green());
            } else {
                println!("{}", "[tk] task.sh already exists".bold().red());
            }
        } else if runner_name == "make" {
            let mut make_file = std::fs::File::create("Makefile").unwrap();
            let bytes = include_bytes!("./templates/Makefile");
            make_file.write_all(bytes).unwrap();
            println!("{}", "Makefile created".bold().green());
        } else if runner_name == "just" {
            let mut make_file = std::fs::File::create("justfile").unwrap();
            let bytes = include_bytes!("./templates/justfile");
            make_file.write_all(bytes).unwrap();
            println!("{}", "justfile created".bold().green());
        } else if runner_name == "jbang" {
            let mut make_file = std::fs::File::create("jbang-catalog.json").unwrap();
            let bytes = include_bytes!("./templates/jbang-catalog.json");
            make_file.write_all(bytes).unwrap();
            println!("{}", "jbang-catalog.json created".bold().green());
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
        let tasks_options = matches.values_of("tasks").unwrap().collect::<Vec<&str>>();
        let task_context = TaskContext::new(tasks_options);
        let tasks = task_context.names;
        let task_args = &task_context.task_options;
        let global_args = &task_context.global_options;
        let runner = matches.value_of("runner").unwrap_or("");
        let task_count = run_tasks(runner, &tasks, task_args, global_args, verbose).unwrap();
        if task_count == 0 { // no tasks executed
            if runners::makefile::is_available() { // try Makefile
                for task in tasks {
                    runners::makefile::run_task(task, task_args, global_args, verbose).unwrap();
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

fn diagnose() {
    let mut problems_count = 0;
    if runners::justfile::is_available() {
        if !runners::justfile::is_command_available() {
            problems_count += 1;
            println!("{} just(https://github.com/casey/just) command not available for justfile", "Warning:".bold().yellow());
        }
    }
    if runners::packagejson::is_available() {
        if !runners::packagejson::is_command_available() {
            problems_count += 1;
            println!("{} npm(https://nodejs.org) command not available for package.json", "Warning:".bold().yellow());
        }
    }
    if runners::denojson::is_available() {
        if !runners::denojson::is_command_available() {
            problems_count += 1;
            println!("{} deno(https://deno.land) command not available for deno.json", "Warning:".bold().yellow());
        }
    }
    if runners::makefile::is_available() {
        if !runners::makefile::is_command_available() {
            problems_count += 1;
            println!("{} make(https://www.gnu.org/software/make) command not available for makefile", "Warning:".bold().yellow());
        }
        if which::which("mmake").is_err() {
            println!("{} mmake(https://github.com/tj/mmake) is more powerful to run Makefile", "Suggestion:".bold().yellow());
        }
    }
    if runners::rakefile::is_available() {
        if !runners::rakefile::is_command_available() {
            problems_count += 1;
            println!("{} rake(https://ruby.github.io/rake/) command not available for rakefile", "Warning:".bold().yellow());
        }
    }
    if runners::taskfileyml::is_available() {
        if !runners::taskfileyml::is_command_available() {
            problems_count += 1;
            println!("{} task(https://taskfile.dev) command not available for Taskfile.yml", "Warning:".bold().yellow());
        }
    }
    if runners::makefiletoml::is_available() {
        if !runners::makefiletoml::is_command_available() {
            problems_count += 1;
            println!("{} cargo-make(https://github.com/sagiegurari/cargo-make) command not available for Makefile.toml", "Warning:".bold().yellow());
        }
    }
    if runners::taskspy::is_available() {
        if !runners::taskspy::is_command_available() {
            problems_count += 1;
            println!("{} invoke(https://www.pyinvoke.org) command not available for tasks.py", "Warning:".bold().yellow());
        }
    }
    if runners::composer::is_available() {
        if !runners::composer::is_command_available() {
            problems_count += 1;
            println!("{} composer(https://getcomposer.org/) command not available for composer.json", "Warning:".bold().yellow());
        }
    }
    if runners::jbang::is_available() {
        if !runners::jbang::is_command_available() {
            problems_count += 1;
            println!("{} jbang(https://www.jbang.dev/) command not available for jbang-catalog.json", "Warning:".bold().yellow());
        }
    }
    // ==========package managers============
    if managers::maven::is_available() {
        if !managers::maven::is_command_available() {
            problems_count += 1;
            println!("{} maven(https://maven.apache.org/) command not available for pom.xml", "Warning:".bold().yellow());
        }
    }
    if managers::gradle::is_available() {
        if !managers::gradle::is_command_available() {
            problems_count += 1;
            println!("{} gradle(https://gradle.org/) command not available for {}", "Warning:".bold().yellow(), managers::gradle::get_gradle_build_file());
        } else {
            //global plugins for gradle $HOME/.gradle/init.d/plugins.gradle
            if !dirs::home_dir().unwrap().join(".gradle").join("init.d").join("plugins.gradle").exists() {
                println!("{} global {} not available for {} task, please check https://github.com/linux-china/task-keeper#gradle",
                         "Suggestion:".bold().yellow(), "plugins.gradle".bold().blue(), "dependencyUpdates".bold().blue());
            }
        }
    }
    if managers::sbt::is_available() {
        if !managers::sbt::is_command_available() {
            problems_count += 1;
            println!("{} sbt(https://www.scala-sbt.org/) command not available for build.sbt", "Warning:".bold().yellow());
        } else {
            //global plugins for sbt $HOME/.sbt/1.0/plugins/plugins.sbt
            if !dirs::home_dir().unwrap().join(".sbt").join("1.0").join("plugins").join("plugins.sbt").exists() {
                println!("{} global {} not available for {} task, please check https://github.com/linux-china/task-keeper#sbt",
                         "Suggestion:".bold().yellow(), "plugins.sbt".bold().blue(), "dependencyUpdates".bold().blue());
            }
        }
    }
    if managers::lein::is_available() {
        if !managers::lein::is_command_available() {
            problems_count += 1;
            println!("{} lein(https://leiningen.org/) command not available for project.clj", "Warning:".bold().yellow());
        } else {
            //global plugins for lein $HOME/.lein/profiles.clj
            if !dirs::home_dir().unwrap().join(".lein").join("profiles.clj").exists() {
                println!("{} global {} not available for {} task, please check https://github.com/linux-china/task-keeper#lein",
                         "Suggestion:".bold().yellow(), "profiles.clj".bold().blue(), "outdated".bold().blue());
            }
        }
    }
    if managers::npm::is_available() {
        if !managers::npm::is_command_available() {
            problems_count += 1;
            println!("{} npm(https://nodejs.org/) command not available for package.json", "Warning:".bold().yellow());
        }
    }
    if managers::cargo::is_available() {
        if !managers::cargo::is_command_available() {
            problems_count += 1;
            println!("{} cargo(https://gradle.org/) command not available for Cargo.toml", "Warning:".bold().yellow());
        }
    }
    if managers::composer::is_available() {
        if !managers::composer::is_command_available() {
            problems_count += 1;
            println!("{} gradle(https://gradle.org/) command not available for composer.json", "Warning:".bold().yellow());
        }
    }
    if managers::bundler::is_available() {
        if !managers::bundler::is_command_available() {
            problems_count += 1;
            println!("{} bundle(https://bundler.io/) command not available for Gemfile", "Warning:".bold().yellow());
        }
    }
    if managers::golang::is_available() {
        if !managers::golang::is_command_available() {
            problems_count += 1;
            println!("{} go(https://go.dev/) command not available for go.mod", "Warning:".bold().yellow());
        }
    }
    if managers::cmakeconan::is_available() {
        if !managers::cmakeconan::is_command_available() {
            problems_count += 1;
            println!("{} cmake and conan(https://github.com/conan-io/cmake-conan/) command not available for CMakeLists.txt and conanfile.txt", "Warning:".bold().yellow());
        }
    }
    if managers::swift::is_available() {
        if !managers::swift::is_command_available() {
            problems_count += 1;
            println!("{} swift(https://www.swift.org/) command not available for Package.swift", "Warning:".bold().yellow());
        }
    }
    if managers::bazel::is_available() {
        if !managers::bazel::is_command_available() {
            problems_count += 1;
            println!("{} bazel(https://bazel.build/) command not available for WORKSPACE", "Warning:".bold().yellow());
        }
    }
    if managers::poetry::is_available() {
        if !managers::poetry::is_command_available() {
            problems_count += 1;
            println!("{} poetry(https://python-poetry.org/) command not available for pyproject.toml", "Warning:".bold().yellow());
        }
    }
    if managers::rebar3::is_available() {
        if !managers::rebar3::is_command_available() {
            problems_count += 1;
            println!("{} rebar3(https://rebar3.readme.io/) command not available for rebar.config", "Warning:".bold().yellow());
        }
    }
    if problems_count > 0 {
        println!("{} {} problems found!", "Warning:".bold().yellow(), problems_count);
    } else {
        println!("{} no problems found, and you are a nice developer :)", "Success:".bold().green());
    }
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
