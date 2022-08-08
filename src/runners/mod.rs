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

use std::process::{Output};
use colored::Colorize;
use error_stack::{report, Result};
use crate::errors::KeeperError;

pub const RUNNERS: &'static [&'static str] = &["rake", "invoke", "task", "cargo-make", "just", "make", "proc", "npm", "deno", "composer", "shell", "fleet", "markdown"];

pub fn run_task(runner: &str, task_name: &str, extra_args: &[&str], verbose: bool) -> Result<Output, KeeperError> {
    println!("{}", format!("[tk] execute {} from {}", task_name, runner).bold().blue());
    match runner {
        "npm" => packagejson::run_task(task_name, extra_args, verbose),
        "just" => justfile::run_task(task_name, extra_args, verbose),
        "fleet" => fleet::run_task(task_name, extra_args, verbose),
        "deno" => denojson::run_task(task_name, extra_args, verbose),
        "make" => makefile::run_task(task_name, extra_args, verbose),
        "rake" => rakefile::run_task(task_name, extra_args, verbose),
        "task" => taskfileyml::run_task(task_name, extra_args, verbose),
        "invoke" => taskspy::run_task(task_name, extra_args, verbose),
        "cargo-make" => makefiletoml::run_task(task_name, extra_args, verbose),
        "procfile" => procfile::run_task(task_name, extra_args, verbose),
        "composer" => composer::run_task(task_name, extra_args, verbose),
        "markdown" => markdown::run_task(task_name, extra_args, verbose),
        "shell" => taskshell::run_task(task_name, extra_args, verbose),
        _ => Err(report!(KeeperError::FailedToRunTasks(format!("Unknown runner: {}", runner)))),
    }
}

pub fn get_runner_file_name(runner: &str) -> &'static str {
    match runner {
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
        "shell" => "task.sh",
        "markdown" => "README.md",
        _ => "unknown",
    }
}
