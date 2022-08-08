use std::process::Output;
use colored::Colorize;
use crate::errors::KeeperError;
use error_stack::{IntoReport, report, Result, ResultExt};

pub mod maven;
pub mod gradle;
pub mod npm;
pub mod cargo;
pub mod sbt;
pub mod composer;
pub mod bundler;

pub const COMMANDS: &'static [&'static str] = &["init", "install", "compile", "build", "start", "test", "deps", "doc", "clean", "outdated", "update"];
pub const MANAGERS: &'static [&'static str] = &["maven", "gradle", "sbt", "npm", "cargo", "cmake", "composer", "bundle", "cmake", "go"];

pub fn get_available_managers() -> Vec<String> {
    let mut managers = Vec::new();
    if maven::is_available() {
        managers.push("maven".to_string());
    }
    if gradle::is_available() {
        managers.push("gradle".to_string());
    }
    if sbt::is_available() {
        managers.push("sbt".to_string());
    }
    if npm::is_available() {
        managers.push("npm".to_string());
    }
    if cargo::is_available() {
        managers.push("cargo".to_string());
    }
    if composer::is_available() {
        managers.push("composer".to_string());
    }
    if bundler::is_available() {
        managers.push("bundle".to_string());
    }
    managers
}

pub fn get_manager_file_name(runner: &str) -> &'static str {
    match runner {
        "maven" => "pom.xml",
        "gradle" => "build.gradle[.kts]",
        "sbt" => "build.sbt",
        "npm" => "package.json",
        "cargo" => "Cargo.toml",
        "cmake" => "CMakeLists.txt",
        "composer" => "composer.json",
        "go" => "go.mod",
        "swift" => "Package.swift",
        "bundle" => "Gemfile",
        _ => "unknown",
    }
}

pub fn run_task(runner: &str, task_name: &str, extra_args: &[&str], verbose: bool) -> Result<(), KeeperError> {
    println!("{}", format!("[tk] execute {} from {}", task_name, runner).bold().blue());
    let mut queue: Vec<fn(&str, &[&str], bool) -> Result<Output, KeeperError>> = vec![];
    if maven::is_available() {
        if maven::is_command_available() {
            queue.push(maven::run_task);
        } else {
            println!("{}", format!("[tk] maven(https://maven.apache.org/) command not available for pom.xml").bold().red());
        }
    }
    if gradle::is_available() {
        if gradle::is_command_available() {
            queue.push(gradle::run_task);
        } else {
            println!("{}", format!("[tk] gradle(https://gradle.org/) command not available").bold().red());
        }
    }
    if sbt::is_available() {
        if sbt::is_command_available() {
            queue.push(sbt::run_task);
        } else {
            println!("{}", format!("[tk] sbt(https://www.scala-sbt.org/) command not available for build.sbt").bold().red());
        }
    }
    if npm::is_available() {
        if npm::is_command_available() {
            queue.push(npm::run_task);
        } else {
            println!("{}", format!("[tk] npm(https://nodejs.org/) command not available for package.json").bold().red());
        }
    }
    if cargo::is_available() {
        if cargo::is_command_available() {
            queue.push(cargo::run_task);
        } else {
            println!("{}", format!("[tk] cargo(https://gradle.org/) command not available for Cargo.toml").bold().red());
        }
    }
    if composer::is_available() {
        if composer::is_command_available() {
            queue.push(composer::run_task);
        } else {
            println!("{}", format!("[tk] gradle(https://gradle.org/) command not available for composer.json").bold().red());
        }
    }
    if bundler::is_available() {
        if bundler::is_command_available() {
            queue.push(bundler::run_task);
        } else {
            println!("{}", format!("[tk] bundle(https://bundler.io/) command not available for Gemfile").bold().red());
        }
    }
    match task_name {
        "init" => {}
        "start" => { // only execute start task once
            if let Some(task) = queue.first() {
                task(task_name, extra_args, verbose).unwrap();
            }
        }
        _ => {
            queue.iter().for_each(|task| {
                task(task_name, extra_args, verbose).unwrap();
            });
        }
    }
    Ok(())
}
