use crate::common::notification::send_notification;
use crate::errors::KeeperError;
use crate::models::Task;
use crate::runners::RUNNERS;
use crate::{managers, runners};
use colored::Colorize;
use error_stack::{Report};
use std::collections::HashMap;

pub fn run_tasks(
    cli_runner: &str,
    target_task_names: &[&str],
    task_args: &[&str],
    global_args: &[&str],
    verbose: bool,
) -> core::result::Result<i32, Report<KeeperError>> {
    let mut task_count = 0;
    let all_tasks = list_all_runner_tasks(true);
    if let Ok(tasks_hashmap) = all_tasks {
        if !cli_runner.is_empty() {
            //runner is specified
            if let Some(runner_tasks) = tasks_hashmap.get(cli_runner) {
                let mut runner_task_found = false;
                for target_task_name in target_task_names {
                    runner_tasks.iter().for_each(|task| {
                        if task.name.as_str() == *target_task_name {
                            task_count += 1;
                            runner_task_found = true;
                            run_runner_task(
                                cli_runner,
                                target_task_name,
                                task_args,
                                global_args,
                                verbose,
                            );
                        }
                    });
                    // execute package manager task
                    if !runner_task_found && managers::COMMANDS.contains(target_task_name) {
                        task_count += 1;
                        run_manager_task(
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
                RUNNERS.iter().for_each(|runner| {
                    if let Some(tasks) = tasks_hashmap.get(*runner) {
                        tasks.iter().for_each(|task| {
                            if task.name.as_str() == *target_task_name {
                                task_count += 1;
                                runner_task_found = true;
                                run_runner_task(
                                    runner,
                                    target_task_name,
                                    task_args,
                                    global_args,
                                    verbose,
                                );
                            }
                        });
                    }
                });
                // execute package manager task
                if !runner_task_found && managers::COMMANDS.contains(target_task_name) {
                    task_count += 1;
                    run_manager_task(
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
) {
    let command_output =
        runners::run_task(runner, task_name, task_args, global_args, verbose).unwrap();
    if std::env::var("TK_TASK_ID").is_ok() {
        send_notification(&command_output, task_name, task_args);
    }
}

pub fn run_manager_task(
    runner: &str,
    task_name: &str,
    task_args: &[&str],
    global_args: &[&str],
    verbose: bool,
) -> Result<(), Report<KeeperError>> {
    managers::run_task(runner, task_name, task_args, global_args, verbose)
}

pub fn list_all_runner_tasks(
    error_display: bool,
) -> Result<HashMap<String, Vec<Task>>, KeeperError> {
    let mut all_tasks = HashMap::new();
    if runners::ant::is_available() {
        if runners::ant::is_command_available() {
            if let Ok(runner_tasks) = runners::ant::list_tasks() {
                if !runner_tasks.is_empty() {
                    all_tasks.insert("ant".to_string(), runner_tasks);
                }
            }
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
        if let Ok(runner_tasks) = runners::fleet::list_tasks() {
            if !runner_tasks.is_empty() {
                all_tasks.insert("fleet".to_string(), runner_tasks);
            }
        }
    }
    if runners::vstasks::is_available() {
        if let Ok(runner_tasks) = runners::vstasks::list_tasks() {
            if !runner_tasks.is_empty() {
                all_tasks.insert("vscode".to_string(), runner_tasks);
            }
        }
    }
    if runners::zed::is_available() {
        if let Ok(runner_tasks) = runners::zed::list_tasks() {
            if !runner_tasks.is_empty() {
                all_tasks.insert("zed".to_string(), runner_tasks);
            }
        }
    }
    if runners::procfile::is_available() {
        if let Ok(runner_tasks) = runners::procfile::list_tasks() {
            if !runner_tasks.is_empty() {
                all_tasks.insert("procfile".to_string(), runner_tasks);
            }
        }
    }
    if runners::markdown::is_available() {
        if let Ok(runner_tasks) = runners::markdown::list_tasks() {
            if !runner_tasks.is_empty() {
                all_tasks.insert("markdown".to_string(), runner_tasks);
            }
        }
    }
    if runners::taskshell::is_available() {
        if let Ok(runner_tasks) = runners::taskshell::list_tasks() {
            if !runner_tasks.is_empty() {
                all_tasks.insert("shell".to_string(), runner_tasks);
            }
        }
    }
    if runners::justfile::is_available() {
        if runners::justfile::is_command_available() {
            if let Ok(runner_tasks) = runners::justfile::list_tasks() {
                if !runner_tasks.is_empty() {
                    all_tasks.insert("just".to_string(), runner_tasks);
                }
            }
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
            if let Ok(runner_tasks) = runners::packagejson::list_tasks() {
                if !runner_tasks.is_empty() {
                    all_tasks.insert("npm".to_string(), runner_tasks);
                }
            }
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
            if let Ok(runner_tasks) = runners::denojson::list_tasks() {
                if !runner_tasks.is_empty() {
                    all_tasks.insert("deno".to_string(), runner_tasks);
                }
            }
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
            if let Ok(runner_tasks) = runners::makefile::list_tasks() {
                if !runner_tasks.is_empty() {
                    all_tasks.insert("make".to_string(), runner_tasks);
                }
            }
        } else {
            if error_display {
                println!("{}", "[tk] make(https://www.gnu.org/software/make) command not available for makefile".bold().red());
            }
        }
    }
    if runners::rakefile::is_available() {
        if runners::rakefile::is_command_available() {
            if let Ok(runner_tasks) = runners::rakefile::list_tasks() {
                if !runner_tasks.is_empty() {
                    all_tasks.insert("rake".to_string(), runner_tasks);
                }
            }
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
            if let Ok(runner_tasks) = runners::jakefile::list_tasks() {
                if !runner_tasks.is_empty() {
                    all_tasks.insert("jake".to_string(), runner_tasks);
                }
            }
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
            if let Ok(runner_tasks) = runners::gulpfile::list_tasks() {
                if !runner_tasks.is_empty() {
                    all_tasks.insert("gulp".to_string(), runner_tasks);
                }
            }
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
            if let Ok(runner_tasks) = runners::gruntfile::list_tasks() {
                if !runner_tasks.is_empty() {
                    all_tasks.insert("grunt".to_string(), runner_tasks);
                }
            }
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
            if let Ok(runner_tasks) = runners::taskfileyml::list_tasks() {
                if !runner_tasks.is_empty() {
                    all_tasks.insert("task".to_string(), runner_tasks);
                }
            }
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
            if let Ok(runner_tasks) = runners::makefiletoml::list_tasks() {
                if !runner_tasks.is_empty() {
                    all_tasks.insert("cargo-make".to_string(), runner_tasks);
                }
            }
        } else {
            if error_display {
                println!("{}", "[tk] cargo-make(https://github.com/sagiegurari/cargo-make) command not available for Makefile.toml".bold().red());
            }
        }
    }
    if runners::bun_shell::is_available() {
        if runners::bun_shell::is_command_available() {
            if let Ok(runner_tasks) = runners::bun_shell::list_tasks() {
                if !runner_tasks.is_empty() {
                    all_tasks.insert("bun-shell".to_string(), runner_tasks);
                }
            }
        } else {
            if error_display {
                println!("{}", "[tk] bun(https://bun.sh/docs/runtime/shell) command not available for Taskfile.ts".bold().red());
            }
        }
    }
    if runners::taskspy::is_available() {
        if runners::taskspy::is_command_available() {
            if let Ok(runner_tasks) = runners::taskspy::list_tasks() {
                if !runner_tasks.is_empty() {
                    all_tasks.insert("invoke".to_string(), runner_tasks);
                }
            }
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
            if let Ok(runner_tasks) = runners::composer::list_tasks() {
                if !runner_tasks.is_empty() {
                    all_tasks.insert("composer".to_string(), runner_tasks);
                }
            }
        } else {
            if error_display {
                println!("{}", "[tk] composer(https://getcomposer.org/) command not available for composer.json".bold().red());
            }
        }
    }
    if runners::jbang::is_available() {
        if runners::jbang::is_command_available() {
            if let Ok(runner_tasks) = runners::jbang::list_tasks() {
                if !runner_tasks.is_empty() {
                    all_tasks.insert("jbang".to_string(), runner_tasks);
                }
            }
        } else {
            if error_display {
                println!("{}", "[tk] jbang(https://www.jbang.dev/) command not available for jbang-catalog.json".bold().red());
            }
        }
    }
    if runners::poetry::is_available() {
        if runners::poetry::is_command_available() {
            if let Ok(runner_tasks) = runners::poetry::list_tasks() {
                if !runner_tasks.is_empty() {
                    all_tasks.insert("poetry".to_string(), runner_tasks);
                }
            }
        } else {
            if error_display {
                println!("{}", "[tk] poetry(https://python-poetry.org/) command not available for pyproject.toml".bold().red());
            }
        }
    }
    if runners::poe::is_available() {
        if runners::poe::is_command_available() {
            if let Ok(runner_tasks) = runners::poe::list_tasks() {
                if !runner_tasks.is_empty() {
                    all_tasks.insert("poe".to_string(), runner_tasks);
                }
            }
        } else {
            if error_display {
                println!("{}", "[tk] poe(https://github.com/nat-n/poethepoet) command not available for pyproject.toml".bold().red());
            }
        }
    }
    if runners::argcfile::is_available() {
        if runners::argcfile::is_command_available() {
            if let Ok(runner_tasks) = runners::argcfile::list_tasks() {
                if !runner_tasks.is_empty() {
                    all_tasks.insert("argc".to_string(), runner_tasks);
                }
            }
        } else {
            if error_display {
                println!("{}", "[tk] argc(https://github.com/sigoden/argc) command not available for Argcfile.sh".bold().red());
            }
        }
    }
    if runners::uv_scripts::is_available() {
        if runners::uv_scripts::is_command_available() {
            if let Ok(runner_tasks) = runners::uv_scripts::list_tasks() {
                if !runner_tasks.is_empty() {
                    all_tasks.insert("uvs".to_string(), runner_tasks);
                }
            }
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
            if let Ok(runner_tasks) = runners::nurfile::list_tasks() {
                if !runner_tasks.is_empty() {
                    all_tasks.insert("nur".to_string(), runner_tasks);
                }
            }
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
    if runners::xtask::is_available() {
        if let Ok(runner_tasks) = runners::xtask::list_tasks() {
            if !runner_tasks.is_empty() {
                all_tasks.insert("xtask".to_string(), runner_tasks);
            }
        }
    }
    if runners::xtask_go::is_available() {
        if let Ok(runner_tasks) = runners::xtask_go::list_tasks() {
            if !runner_tasks.is_empty() {
                all_tasks.insert("xtask-go".to_string(), runner_tasks);
            }
        }
    }
    /*all_tasks.iter().for_each(|(runner, tasks)| {
        println!("{}", format!("[tk] {} tasks:", runner).bold().green());
        tasks.iter().for_each(|task| {
            println!("{}", format!("[tk]   {}", &task.name).bold().yellow());
        });
    });*/
    Ok(all_tasks)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_task() {
        run_runner_task("npm", "start", &[], &[], true);
    }
}
