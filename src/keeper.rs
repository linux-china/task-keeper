use crate::common::notification::send_notification;
use crate::errors::KeeperError;
use crate::models::Task;
use crate::runners::RUNNERS;
use crate::{managers, runners};
use colored::Colorize;
use error_stack::Report;
use std::collections::HashMap;

pub fn run_tasks(
    cli_runner: &str,
    target_task_names: &[&str],
    task_args: &[&str],
    global_args: &[&str],
    verbose: bool,
) -> Result<i32, Report<KeeperError>> {
    let mut task_count = 0;
    let all_tasks = list_all_runner_tasks(true);
    if let Ok(tasks_hashmap) = all_tasks {
        if !cli_runner.is_empty() {
            //runner is specified
            if let Some(runner_tasks) = tasks_hashmap.get(cli_runner) {
                for target_task_name in target_task_names {
                    let mut runner_task_found = false;
                    for task in runner_tasks {
                        if task.name.as_str() == *target_task_name {
                            task_count += 1;
                            runner_task_found = true;
                            run_runner_task(
                                cli_runner,
                                target_task_name,
                                task_args,
                                global_args,
                                verbose,
                            )?;
                        }
                    }
                    // execute package manager task
                    if !runner_task_found && managers::COMMANDS.contains(target_task_name) {
                        task_count += run_manager_task(
                            cli_runner,
                            target_task_name,
                            task_args,
                            global_args,
                            verbose,
                        )?;
                    }
                }
            } else if managers::MANAGERS.contains(&cli_runner) {
                // runner is a project/package manager, such as cargo or maven
                for target_task_name in target_task_names {
                    if managers::COMMANDS.contains(target_task_name) {
                        task_count += run_manager_task(
                            cli_runner,
                            target_task_name,
                            task_args,
                            global_args,
                            verbose,
                        )?;
                    }
                }
            }
        } else {
            //unknown runner
            for target_task_name in target_task_names {
                let mut runner_task_found = false;
                for runner in RUNNERS {
                    if let Some(tasks) = tasks_hashmap.get(*runner) {
                        for task in tasks {
                            if task.name.as_str() == *target_task_name {
                                task_count += 1;
                                runner_task_found = true;
                                run_runner_task(
                                    runner,
                                    target_task_name,
                                    task_args,
                                    global_args,
                                    verbose,
                                )?;
                            }
                        }
                    }
                }
                // execute package manager task
                if !runner_task_found && managers::COMMANDS.contains(target_task_name) {
                    task_count += run_manager_task(
                        cli_runner,
                        target_task_name,
                        task_args,
                        global_args,
                        verbose,
                    )?;
                }
            }
        }
    }
    Ok(task_count)
}

pub fn run_runner_task(
    runner: &str,
    task_name: &str,
    task_args: &[&str],
    global_args: &[&str],
    verbose: bool,
) -> Result<(), Report<KeeperError>> {
    let command_output = runners::run_task(runner, task_name, task_args, global_args, verbose)?;
    if std::env::var("TK_TASK_ID").is_ok() {
        send_notification(&command_output, task_name, task_args);
    }
    command_output.ensure_success(task_name)
}

pub fn run_manager_task(
    runner: &str,
    task_name: &str,
    task_args: &[&str],
    global_args: &[&str],
    verbose: bool,
) -> Result<i32, Report<KeeperError>> {
    managers::run_task(runner, task_name, task_args, global_args, verbose)
}

pub fn list_all_runner_tasks(
    error_display: bool,
) -> Result<HashMap<String, Vec<Task>>, KeeperError> {
    let mut all_tasks = HashMap::new();
    if runners::ant::is_available() {
        if runners::ant::is_command_available() {
            insert_runner_tasks(&mut all_tasks, "ant", runners::ant::list_tasks(), error_display);
        } else {
            if error_display {
                println!(
                    "{}",
                    "[tk] ant(https://ant.apache.org/) command not available for build.xml"
                        .bold()
                        .red()
                );
            }
        }
    }
    if runners::fleet::is_available() {
        insert_runner_tasks(&mut all_tasks, "fleet", runners::fleet::list_tasks(), error_display);
    }
    if runners::vstasks::is_available() {
        insert_runner_tasks(&mut all_tasks, "vscode", runners::vstasks::list_tasks(), error_display);
    }
    if runners::zed::is_available() {
        insert_runner_tasks(&mut all_tasks, "zed", runners::zed::list_tasks(), error_display);
    }
    if runners::procfile::is_available() {
        insert_runner_tasks(&mut all_tasks, "procfile", runners::procfile::list_tasks(), error_display);
    }
    if runners::markdown::is_available() {
        insert_runner_tasks(&mut all_tasks, "markdown", runners::markdown::list_tasks(), error_display);
    }
    if runners::taskshell::is_available() {
        insert_runner_tasks(&mut all_tasks, "shell", runners::taskshell::list_tasks(), error_display);
    }
    if runners::justfile::is_available() {
        if runners::justfile::is_command_available() {
            insert_runner_tasks(&mut all_tasks, "just", runners::justfile::list_tasks(), error_display);
        } else {
            if error_display {
                println!(
                    "{}",
                    "[tk] just(https://github.com/casey/just) command not available for justfile"
                        .bold()
                        .red()
                );
            }
        }
    }
    if runners::packagejson::is_available() {
        if runners::packagejson::is_command_available() {
            insert_runner_tasks(&mut all_tasks, "npm", runners::packagejson::list_tasks(), error_display);
        } else {
            if error_display {
                println!(
                    "{}",
                    "[tk] npm(https://nodejs.org) command not available for package.json"
                        .bold()
                        .red()
                );
            }
        }
    }
    if runners::denojson::is_available() {
        if runners::denojson::is_command_available() {
            insert_runner_tasks(&mut all_tasks, "deno", runners::denojson::list_tasks(), error_display);
        } else {
            if error_display {
                println!(
                    "{}",
                    "[tk] deno(https://deno.land) command not available for deno.json"
                        .bold()
                        .red()
                );
            }
        }
    }
    if runners::makefile::is_available() {
        if runners::makefile::is_command_available() {
            insert_runner_tasks(&mut all_tasks, "make", runners::makefile::list_tasks(), error_display);
        } else {
            if error_display {
                println!("{}", "[tk] make(https://www.gnu.org/software/make) command not available for makefile".bold().red());
            }
        }
    }
    if runners::rakefile::is_available() {
        if runners::rakefile::is_command_available() {
            insert_runner_tasks(&mut all_tasks, "rake", runners::rakefile::list_tasks(), error_display);
        } else {
            if error_display {
                println!(
                    "{}",
                    "[tk] rake(https://ruby.github.io/rake/) command not available for rakefile"
                        .bold()
                        .red()
                );
            }
        }
    }
    if runners::jakefile::is_available() {
        if runners::jakefile::is_command_available() {
            insert_runner_tasks(&mut all_tasks, "jake", runners::jakefile::list_tasks(), error_display);
        } else {
            if error_display {
                println!(
                    "{}",
                    "[tk] jake(https://jakejs.com) command not available for jakefile.js"
                        .bold()
                        .red()
                );
            }
        }
    }
    if runners::gulpfile::is_available() {
        if runners::gulpfile::is_command_available() {
            insert_runner_tasks(&mut all_tasks, "gulp", runners::gulpfile::list_tasks(), error_display);
        } else {
            if error_display {
                println!(
                    "{}",
                    "[tk] gulp(https://gulpjs.com/) command not available for gulpfile.js"
                        .bold()
                        .red()
                );
            }
        }
    }
    if runners::gruntfile::is_available() {
        if runners::gruntfile::is_command_available() {
            insert_runner_tasks(&mut all_tasks, "grunt", runners::gruntfile::list_tasks(), error_display);
        } else {
            if error_display {
                println!(
                    "{}",
                    "[tk] grunt(https://gruntjs.com/) command not available for Gruntfile.js"
                        .bold()
                        .red()
                );
            }
        }
    }
    if runners::taskfileyml::is_available() {
        if runners::taskfileyml::is_command_available() {
            insert_runner_tasks(&mut all_tasks, "task", runners::taskfileyml::list_tasks(), error_display);
        } else {
            if error_display {
                println!(
                    "{}",
                    "[tk] task(https://taskfile.dev) command not available for Taskfile.yml"
                        .bold()
                        .red()
                );
            }
        }
    }
    if runners::makefiletoml::is_available() {
        if runners::makefiletoml::is_command_available() {
            insert_runner_tasks(&mut all_tasks, "cargo-make", runners::makefiletoml::list_tasks(), error_display);
        } else {
            if error_display {
                println!("{}", "[tk] cargo-make(https://github.com/sagiegurari/cargo-make) command not available for Makefile.toml".bold().red());
            }
        }
    }
    if runners::bun_shell::is_available() {
        if runners::bun_shell::is_command_available() {
            insert_runner_tasks(&mut all_tasks, "bun-shell", runners::bun_shell::list_tasks(), error_display);
        } else {
            if error_display {
                println!("{}", "[tk] bun(https://bun.sh/docs/runtime/shell) command not available for Taskfile.ts".bold().red());
            }
        }
    }
    if runners::taskspy::is_available() {
        if runners::taskspy::is_command_available() {
            insert_runner_tasks(&mut all_tasks, "invoke", runners::taskspy::list_tasks(), error_display);
        } else {
            if error_display {
                println!(
                    "{}",
                    "[tk] invoke(https://www.pyinvoke.org) command not available for tasks.py"
                        .bold()
                        .red()
                );
            }
        }
    }
    if runners::composer::is_available() {
        if runners::composer::is_command_available() {
            insert_runner_tasks(&mut all_tasks, "composer", runners::composer::list_tasks(), error_display);
        } else {
            if error_display {
                println!("{}", "[tk] composer(https://getcomposer.org/) command not available for composer.json".bold().red());
            }
        }
    }
    if runners::jbang::is_available() {
        if runners::jbang::is_command_available() {
            insert_runner_tasks(&mut all_tasks, "jbang", runners::jbang::list_tasks(), error_display);
        } else {
            if error_display {
                println!("{}", "[tk] jbang(https://www.jbang.dev/) command not available for jbang-catalog.json".bold().red());
            }
        }
    }
    if runners::poetry::is_available() {
        if runners::poetry::is_command_available() {
            insert_runner_tasks(&mut all_tasks, "poetry", runners::poetry::list_tasks(), error_display);
        } else {
            if error_display {
                println!("{}", "[tk] poetry(https://python-poetry.org/) command not available for pyproject.toml".bold().red());
            }
        }
    }
    if runners::poe::is_available() {
        if runners::poe::is_command_available() {
            insert_runner_tasks(&mut all_tasks, "poe", runners::poe::list_tasks(), error_display);
        } else {
            if error_display {
                println!("{}", "[tk] poe(https://github.com/nat-n/poethepoet) command not available for pyproject.toml".bold().red());
            }
        }
    }
    if runners::argcfile::is_available() {
        if runners::argcfile::is_command_available() {
            insert_runner_tasks(&mut all_tasks, "argc", runners::argcfile::list_tasks(), error_display);
        } else {
            if error_display {
                println!("{}", "[tk] argc(https://github.com/sigoden/argc) command not available for Argcfile.sh".bold().red());
            }
        }
    }
    if runners::amberfile::is_available() {
        if runners::amberfile::is_command_available() {
            insert_runner_tasks(&mut all_tasks, "amber", runners::amberfile::list_tasks(), error_display);
        } else {
            if error_display {
                println!("{}", "[tk] Amber(https://amber-lang.com/) command not available for Amberfile".bold().red());
            }
        }
    }
    if runners::uv_scripts::is_available() {
        if runners::uv_scripts::is_command_available() {
            insert_runner_tasks(&mut all_tasks, "uvs", runners::uv_scripts::list_tasks(), error_display);
        } else {
            if error_display {
                println!(
                    "{}",
                    "[tk] uv(https://github.com/astral-sh/uv) command not available for pyproject.toml"
                        .bold()
                        .red()
                );
            }
        }
    }
    if runners::nurfile::is_available() {
        if runners::nurfile::is_command_available() {
            insert_runner_tasks(&mut all_tasks, "nur", runners::nurfile::list_tasks(), error_display);
        } else {
            if error_display {
                println!(
                    "{}",
                    "[tk] nur(https://github.com/ddanier/nur) command not available for nurfile"
                        .bold()
                        .red()
                );
            }
        }
    }
    if runners::usql::is_available() {
        if runners::usql::is_command_available() {
            insert_runner_tasks(&mut all_tasks, "usql", runners::usql::list_tasks(), error_display);
        } else {
            if error_display {
                println!(
                    "{}",
                    "[tk] usql(https://github.com/xo/usql/) command not available for queries.sql"
                        .bold()
                        .red()
                );
            }
        }
    }
    if runners::xtask::is_available() {
        insert_runner_tasks(&mut all_tasks, "xtask", runners::xtask::list_tasks(), error_display);
    }
    if runners::xtask_go::is_available() {
        insert_runner_tasks(&mut all_tasks, "xtask-go", runners::xtask_go::list_tasks(), error_display);
    }
    /*all_tasks.iter().for_each(|(runner, tasks)| {
        println!("{}", format!("[tk] {} tasks:", runner).bold().green());
        tasks.iter().for_each(|task| {
            println!("{}", format!("[tk]   {}", &task.name).bold().yellow());
        });
    });*/
    Ok(all_tasks)
}

/// Add tasks of a runner, or display the error, such as invalid task file, instead of ignoring it
fn insert_runner_tasks<E: std::fmt::Display>(
    all_tasks: &mut HashMap<String, Vec<Task>>,
    runner: &str,
    result: Result<Vec<Task>, E>,
    error_display: bool,
) {
    match result {
        Ok(runner_tasks) => {
            if !runner_tasks.is_empty() {
                all_tasks.insert(runner.to_string(), runner_tasks);
            }
        }
        Err(err) => {
            if error_display {
                println!("{}", format!("[tk] {:#}", err).bold().red());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_run_task() {
        let _ = run_runner_task("npm", "start", &[], &[], true);
    }
}
