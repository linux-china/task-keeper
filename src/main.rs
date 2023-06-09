use crate::app::build_app;
use crate::keeper::{run_tasks, list_all_runner_tasks};
use colored::Colorize;
use crate::runners::RUNNERS;
use dotenv::dotenv;
use std::collections::HashSet;
use std::io::Write;
use std::path::Path;
use std::env;
use crate::models::TaskContext;
use crate::polyglot::PATH_SEPARATOR;

mod app;
mod keeper;
mod errors;
mod models;
mod runners;
mod managers;
mod command_utils;
mod common;
mod polyglot;

fn main() {
    let app = build_app();
    let matches = app.get_matches();
    let verbose = matches.get_flag("verbose");
    let no_dotenv = matches.get_flag("no-dotenv");

    // summary to list all task names
    if matches.get_flag("summary") {
        let mut task_names: HashSet<String> = HashSet::new();
        let all_tasks = list_all_runner_tasks(false);
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
    if matches.get_flag("doctor") {
        diagnose();
        return;
    }
    // migrate tasks
    if matches.contains_id("from") && matches.contains_id("to") {
        println!("{}", "Task migration has not yet been implemented!".bold().red());
        return;
    }
    // create task file by runner
    if matches.contains_id("init") {
        let runner_name = matches.get_one::<String>("init").unwrap();
        if runner_name == "shell" {
            let exists = Path::new("./task.sh").exists();
            if !exists {
                let mut tasksh_file = std::fs::File::create("task.sh").unwrap();
                tasksh_file.write_all(include_bytes!("./templates/task.sh")).unwrap();
                set_executable("task.sh");
                println!("{}", "task.sh created".bold().green());
            } else {
                println!("{}", "[tk] task.sh already exists".bold().red());
            }
        } else if runner_name == "make" {
            let mut make_file = std::fs::File::create("Makefile").unwrap();
            make_file.write_all(include_bytes!("./templates/Makefile")).unwrap();
            println!("{}", "Makefile created".bold().green());
        } else if runner_name == "just" {
            let mut make_file = std::fs::File::create("justfile").unwrap();
            make_file.write_all(include_bytes!("./templates/justfile")).unwrap();
            println!("{}", "justfile created".bold().green());
        } else if runner_name == "jbang" {
            let mut make_file = std::fs::File::create("jbang-catalog.json").unwrap();
            make_file.write_all(include_bytes!("./templates/jbang-catalog.json")).unwrap();
            println!("{}", "jbang-catalog.json created".bold().green());
        } else if runner_name == "vscode" {
            if !Path::new(".vscode").exists() {
                std::fs::create_dir(".vscode").unwrap();
            }
            let mut tasks_file = std::fs::File::create(".vscode/tasks.json").unwrap();
            tasks_file.write_all(include_bytes!("./templates/tasks.json")).unwrap();
            println!("{}", ".vscode/tasks.json created".bold().green());
        } else if runner_name == "pipenv" {
            let mut make_file = std::fs::File::create("Pipfile").unwrap();
            make_file.write_all(include_bytes!("./templates/Pipfile")).unwrap();
            println!("{}", "Pipfile created".bold().green());
        } else if runner_name == "deno" {
            let mut deno_json_file = std::fs::File::create("deno.json").unwrap();
            deno_json_file.write_all(include_bytes!("./templates/deno.json")).unwrap();
            let mut import_map_file = std::fs::File::create("import_map.json").unwrap();
            import_map_file.write_all(include_bytes!("./templates/import_map.json")).unwrap();
            println!("{}", "deno.json and import_map.json created".bold().green());
        } else {
            println!("[tk] Create task file for {} not support now.", runner_name);
        }
        return;
    }
    // runner
    let task_runner = matches.get_one::<String>("runner");
    // list tasks
    if matches.get_flag("list") {
        let mut task_found = false;
        let all_tasks = list_all_runner_tasks(true);
        if let Ok(tasks_hashmap) = all_tasks {
            if !tasks_hashmap.is_empty() {
                task_found = true;
                println!("{}", "Available task runners:".bold().green());
                RUNNERS.iter().for_each(|runner| {
                    if task_runner.is_none() || task_runner.unwrap() == *runner {
                        if let Some(tasks) = tasks_hashmap.get(*runner) {
                            if !tasks.is_empty() {
                                println!("{}", format!("  {}: {} - {}", runner,
                                                       runners::get_runner_file_name(runner),
                                                       runners::get_runner_web_url(runner)).bold().blue());
                                tasks.iter().for_each(|task| {
                                    if task.description.is_empty() {
                                        println!("    -- {}", task.name.bold());
                                    } else {
                                        println!("    -- {} : {}", task.name.bold(), format_description(&task.description));
                                    }
                                });
                            }
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
                if task_runner.is_none() || task_runner.unwrap() == &manager_name {
                    if manager_name == "npm" {
                        let package_json = common::parse_package_json().unwrap();
                        let package_command = common::get_npm_command(&package_json);
                        println!("{}", format!("  {}: {} - {}", package_command,
                                               managers::get_manager_file_name(&manager_name),
                                               managers::get_manager_web_url(&manager_name)).bold().blue());
                    } else {
                        println!("{}", format!("  {}: {} - {}", manager_name,
                                               managers::get_manager_file_name(&manager_name),
                                               managers::get_manager_web_url(&manager_name)).bold().blue());
                    }
                    let task_command_map = managers::get_manager_command_map(&manager_name);
                    if !task_command_map.is_empty() {
                        task_command_map.into_iter().for_each(|(task_name, command_line)| {
                            if &task_name != "init" {
                                println!("    -- {} : {}", task_name.bold(), command_line);
                            }
                        });
                    }
                }
            });
        }
        if !task_found {
            println!("{}", "No task runner or project management tool found!".bold().red());
        }
        return;
    }
    // run tasks
    if matches.contains_id("tasks") {
        // load .env for tasks
        if !no_dotenv {
            dotenv().ok();
        }
        // inject polyglot for tasks
        polyglot::inject_languages();
        // setup path
        reset_path_env();
        // check to execute command directly
        let tk_args = env::args().skip(1).collect::<Vec<String>>();
        if tk_args[0] == "--" && tk_args.len() > 1 { // execute command line after double dash
            let command = &tk_args[1];
            let args = tk_args.iter().skip(2).map(|arg| arg.as_str()).collect::<Vec<&str>>();
            command_utils::run_command(command, &args, false).unwrap();
            return;
        }
        let tasks_options = matches.get_many::<String>("tasks")
            .into_iter()
            .flatten()
            .map(|s| s as &str)
            .collect::<Vec<_>>();
        let task_context = TaskContext::new(tasks_options);
        let tasks = task_context.names;
        let task_args = &task_context.task_options;
        let global_args = &task_context.global_options;
        let default_runner = "".to_owned();
        let runner = task_runner.unwrap_or(&default_runner);
        let task_count = run_tasks(runner, &tasks, task_args, global_args, verbose).unwrap();
        if task_count == 0 { // no tasks executed
            println!("{}", "[tk] no tasks found".bold().red());
            /*if runners::makefile::is_available() { // try Makefile
                for task in tasks {
                    runners::makefile::run_task(task, task_args, global_args, verbose).unwrap();
                }
            } else {
                println!("{}", "[tk] no tasks found".bold().red());
            }*/
        }
        return;
    }

    // display help message
    build_app().print_help().unwrap();
}

fn reset_path_env() {
    let current_dir = env::current_dir().unwrap();
    let mut new_path = env::var("PATH").unwrap_or_else(|_| "".to_string());
    for dir in ["bin", ".bin", "node_modules/.bin", "venv/bin", ".venv/bin", "vendor/bin"].iter() {
        let bin_path = current_dir.join(dir);
        if bin_path.exists() {
            new_path = format!("{}{}{}", bin_path.to_string_lossy().to_string(), PATH_SEPARATOR, new_path);
        }
    }
    env::set_var("PATH", new_path);
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
    if runners::poetry::is_available() {
        if !runners::poetry::is_command_available() {
            problems_count += 1;
            println!("{} poetry(https://python-poetry.org/) command not available for pyproject.toml", "Warning:".bold().yellow());
        }
    }
    if runners::rye::is_available() {
        if !runners::rye::is_command_available() {
            problems_count += 1;
            println!("{} rye(https://github.com/mitsuhiko/rye) command not available for requirements.lock", "Warning:".bold().yellow());
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
            println!("{} cargo(https://doc.rust-lang.org/cargo/) command not available for Cargo.toml", "Warning:".bold().yellow());
        }
    }
    if managers::composer::is_available() {
        if !managers::composer::is_command_available() {
            problems_count += 1;
            println!("{} composer(https://getcomposer.org/) command not available for composer.json", "Warning:".bold().yellow());
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
    if managers::pipenv::is_available() {
        if !managers::pipenv::is_command_available() {
            problems_count += 1;
            println!("{} pipenv(https://pipenv.pypa.io/en/latest/) command not available for Pipfile", "Warning:".bold().yellow());
        }
    }
    if managers::requirements::is_available() {
        if !managers::requirements::is_command_available() {
            problems_count += 1;
            println!("{} pip(https://pypi.org/project/pip/) command not available for requirements.txt", "Warning:".bold().yellow());
        }
    }
    if managers::rebar3::is_available() {
        if !managers::rebar3::is_command_available() {
            problems_count += 1;
            println!("{} rebar3(https://rebar3.readme.io/) command not available for rebar.config", "Warning:".bold().yellow());
        }
    }
    if managers::mix::is_available() {
        if !managers::mix::is_command_available() {
            problems_count += 1;
            println!("{} mix(https://hexdocs.pm/mix/1.13/Mix.html) command not available for mix.exs", "Warning:".bold().yellow());
        }
    }
    if polyglot::java::is_available() {
        if polyglot::java::find_sdk_home().is_none() {
            problems_count += 1;
            println!("{} .java-version found, but the JDK({}) not installed!",
                     "Warning:".bold().yellow(),
                     polyglot::java::find_sdk_home().unwrap().display());
        }
    }
    if polyglot::node::is_available() {
        if polyglot::node::find_sdk_home().is_none() {
            problems_count += 1;
            println!("{} .node-version found, but the Node.js({}) not installed!",
                     "Warning:".bold().yellow(),
                     polyglot::node::get_default_version().unwrap());
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
