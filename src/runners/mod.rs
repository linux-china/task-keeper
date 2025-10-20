pub mod fleet;

pub mod ant;
pub mod argcfile;
pub mod bun_shell;
pub mod composer;
pub mod denojson;
pub mod jbang;
pub mod justfile;
pub mod makefile;
pub mod makefiletoml;
pub mod markdown;
pub mod nurfile;
pub mod packagejson;
pub mod poe;
pub mod poetry;
pub mod procfile;
pub mod rakefile;
pub mod taskfileyml;
pub mod taskshell;
pub mod taskspy;
pub mod vstasks;
pub mod xtask;
pub mod xtask_go;
pub mod zed;
pub mod jakefile;
pub mod gulpfile;
pub mod gruntfile;
pub mod uv_scripts;

use crate::command_utils::CommandOutput;
use crate::errors::KeeperError;
use colored::Colorize;
use error_stack::{IntoReport, Report};

pub const RUNNERS: &'static [&'static str] = &[
    "ant",
    "rake",
    "jake",
    "gulp",
    "grunt",
    "invoke",
    "task",
    "cargo-make",
    "just",
    "make",
    "proc",
    "npm",
    "deno",
    "composer",
    "jbang",
    "shell",
    "fleet",
    "vscode",
    "zed",
    "markdown",
    "poe",
    "poetry",
    "bun-shell",
    "argc",
    "xtask",
    "xtask-go",
    "nur",
    "uvs"
];

pub fn run_task(
    runner: &str,
    task_name: &str,
    task_args: &[&str],
    global_args: &[&str],
    verbose: bool,
) -> Result<CommandOutput, Report<KeeperError>> {
    println!(
        "{}",
        format!("[tk] execute {} from {}", task_name, runner)
            .bold()
            .blue()
    );
    match runner {
        "ant" => ant::run_task(task_name, task_args, global_args, verbose),
        "npm" => packagejson::run_task(task_name, task_args, global_args, verbose),
        "just" => justfile::run_task(task_name, task_args, global_args, verbose),
        "fleet" => fleet::run_task(task_name, task_args, global_args, verbose),
        "vscode" => vstasks::run_task(task_name, task_args, global_args, verbose),
        "zed" => zed::run_task(task_name, task_args, global_args, verbose),
        "deno" => denojson::run_task(task_name, task_args, global_args, verbose),
        "make" => makefile::run_task(task_name, task_args, global_args, verbose),
        "rake" => rakefile::run_task(task_name, task_args, global_args, verbose),
        "jake" => jakefile::run_task(task_name, task_args, global_args, verbose),
        "gulp" => gulpfile::run_task(task_name, task_args, global_args, verbose),
        "grunt" => gruntfile::run_task(task_name, task_args, global_args, verbose),
        "task" => taskfileyml::run_task(task_name, task_args, global_args, verbose),
        "invoke" => taskspy::run_task(task_name, task_args, global_args, verbose),
        "cargo-make" => makefiletoml::run_task(task_name, task_args, global_args, verbose),
        "procfile" => procfile::run_task(task_name, task_args, global_args, verbose),
        "composer" => composer::run_task(task_name, task_args, global_args, verbose),
        "markdown" => markdown::run_task(task_name, task_args, global_args, verbose),
        "shell" => taskshell::run_task(task_name, task_args, global_args, verbose),
        "jbang" => jbang::run_task(task_name, task_args, global_args, verbose),
        "poe" => poe::run_task(task_name, task_args, global_args, verbose),
        "poetry" => poetry::run_task(task_name, task_args, global_args, verbose),
        "argc" => argcfile::run_task(task_name, task_args, global_args, verbose),
        "nur" => nurfile::run_task(task_name, task_args, global_args, verbose),
        "uvs" => uv_scripts::run_task(task_name, task_args, global_args, verbose),
        "bun-shell" => bun_shell::run_task(task_name, task_args, global_args, verbose),
        "xtask" => xtask::run_task(task_name, task_args, global_args, verbose),
        "xtask-go" => xtask_go::run_task(task_name, task_args, global_args, verbose),
        _ => Err(KeeperError::FailedToRunTasks(format!(
            "Unknown runner: {}",
            runner
        )).into_report()),
    }
}

pub fn get_runner_file_name(runner: &str) -> &'static str {
    match runner {
        "ant" => "build.xml",
        "rake" => "Rakefile",
        "jake" => "jakefile.js",
        "gulp" => "gulpfile.js",
        "grunt" => "Gruntfile.js",
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
        "zed" => ".zed/tasks.json",
        "shell" => "task.sh",
        "markdown" => "README.md",
        "jbang" => "jbang-catalog.json",
        "poe" => "pyproject.toml",
        "poetry" => "pyproject.toml",
        "bun-shell" => "Taskfile.ts",
        "argc" => "Argcfile.sh",
        "nur" => "nurfile",
        "uvs" => "pyproject.toml",
        "xtask" => "xtask/",
        "xtask-go" => "xtask/main.go",
        _ => "unknown",
    }
}

pub fn get_runner_web_url(runner: &str) -> &'static str {
    match runner {
        "ant" => "https://ant.apache.org/",
        "rake" => "https://ruby.github.io/rake/",
        "jake" => "https://jakejs.com/",
        "gulp" => "https://gulpjs.com/",
        "grunt" => "https://gruntjs.com/",
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
        "zed" => "https://zed.dev/docs/tasks",
        "shell" => "https://www.gnu.org/software/bash/",
        "markdown" => "https://github.com/linux-china/task-keeper#tasks-from-readmemd",
        "jbang" => "https://www.jbang.dev/",
        "poe" => "https://github.com/nat-n/poethepoet",
        "poetry" => "https://python-poetry.org",
        "bun-shell" => "https://bun.sh/docs/runtime/shell",
        "argc" => "https://github.com/sigoden/argc",
        "nur" => "https://github.com/ddanier/nur",
        "uvs" => "https://rye.astral.sh/guide/pyproject/#toolryescripts",
        "xtask" => "https://github.com/matklad/cargo-xtask",
        "xtask-go" => "https://github.com/linux-china/xtask-go-demo",
        _ => "unknown",
    }
}
