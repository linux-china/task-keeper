pub mod fleet;
pub mod justfile;
pub mod packagejson;
pub mod denojson;
pub mod makefile;
pub mod rakefile;
pub mod taskspy;
pub mod taskfileyml;
pub mod makefiletoml;
pub mod procfile;
pub mod markdown;
pub mod taskshell;
pub mod composer;
pub mod jbang;
pub mod vstasks;
pub mod ant;

use std::process::{Output};
use colored::Colorize;
use error_stack::{report, Result};
use crate::errors::KeeperError;

pub const RUNNERS: &'static [&'static str] = &["ant", "rake", "invoke", "task", "cargo-make", "just", "make", "proc", "npm", "deno", "composer", "jbang", "shell", "fleet", "vscode", "markdown"];

pub fn run_task(runner: &str, task_name: &str, task_args: &[&str], global_args: &[&str], verbose: bool) -> Result<Output, KeeperError> {
    println!("{}", format!("[tk] execute {} from {}", task_name, runner).bold().blue());
    match runner {
        "ant" => ant::run_task(task_name, task_args, global_args, verbose),
        "npm" => packagejson::run_task(task_name, task_args, global_args, verbose),
        "just" => justfile::run_task(task_name, task_args, global_args, verbose),
        "fleet" => fleet::run_task(task_name, task_args, global_args, verbose),
        "vscode" => vstasks::run_task(task_name, task_args, global_args, verbose),
        "deno" => denojson::run_task(task_name, task_args, global_args, verbose),
        "make" => makefile::run_task(task_name, task_args, global_args, verbose),
        "rake" => rakefile::run_task(task_name, task_args, global_args, verbose),
        "task" => taskfileyml::run_task(task_name, task_args, global_args, verbose),
        "invoke" => taskspy::run_task(task_name, task_args, global_args, verbose),
        "cargo-make" => makefiletoml::run_task(task_name, task_args, global_args, verbose),
        "procfile" => procfile::run_task(task_name, task_args, global_args, verbose),
        "composer" => composer::run_task(task_name, task_args, global_args, verbose),
        "markdown" => markdown::run_task(task_name, task_args, global_args, verbose),
        "shell" => taskshell::run_task(task_name, task_args, global_args, verbose),
        "jbang" => jbang::run_task(task_name, task_args, global_args, verbose),
        _ => Err(report!(KeeperError::FailedToRunTasks(format!("Unknown runner: {}", runner)))),
    }
}

pub fn get_runner_file_name(runner: &str) -> &'static str {
    match runner {
        "ant" => "build.xml",
        "rake" => "Rakefile",
        "invoke" => "tasks.py",
        "task" => "Taskfile.yml",
        "cargo-make" => "Makefile.toml",
        "just" => "Justfile",
        "make" => "Makefile",
        "proc" => "Procfile",
        "npm" => "package.json",
        "deno" => "deno.json",
        "composer" => "composer.json",
        "fleet" => ".fleet/run.json",
        "vscode" => ".vscode/tasks.json",
        "shell" => "task.sh",
        "markdown" => "README.md",
        "jbang" => "jbang-catalog.json",
        _ => "unknown",
    }
}

pub fn get_runner_web_url(runner: &str) -> &'static str {
    match runner {
        "ant" => "https://ant.apache.org/",
        "rake" => "https://ruby.github.io/rake/",
        "invoke" => "https://www.pyinvoke.org",
        "task" => "https://taskfile.dev",
        "cargo-make" => "https://github.com/sagiegurari/cargo-make",
        "just" => "https://github.com/casey/just",
        "make" => "https://www.gnu.org/software/make",
        "proc" => "https://devcenter.heroku.com/articles/procfile",
        "npm" => "https://nodejs.org",
        "deno" => "https://deno.land",
        "composer" => "https://getcomposer.org",
        "fleet" => "https://www.jetbrains.com/fleet/",
        "vscode" => "https://code.visualstudio.com/docs/editor/tasks",
        "shell" => "https://www.gnu.org/software/bash/",
        "markdown" => "https://github.com/linux-china/task-keeper#tasks-from-readmemd",
        "jbang" => "https://www.jbang.dev/",
        _ => "unknown",
    }
}
