use std::collections::HashMap;
use std::process::Output;
use colored::Colorize;
use crate::errors::KeeperError;
use error_stack::{Result};

pub mod maven;
pub mod gradle;
pub mod npm;
pub mod cargo;
pub mod sbt;
pub mod composer;
pub mod bundler;
pub mod golang;
pub mod cmakeconan;
pub mod swift;
pub mod bazel;
pub mod poetry;
pub mod lein;
pub mod rebar3;
pub mod mix;
pub mod requirements;
pub mod pipenv;
pub mod rye;
pub mod dart;

pub const COMMANDS: &'static [&'static str] = &["init", "install", "compile", "build", "start", "test", "deps", "doc", "clean", "outdated", "update"];
pub const MANAGERS: &'static [&'static str] = &["maven", "gradle", "sbt", "npm", "cargo", "cmake", "composer", "bundle", "cmake", "go", "swift", "bazel", "poetry", "pip", "pipenv", "rye", "lein", "rebar3", "mix","dart"];

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
    if golang::is_available() {
        managers.push("go".to_string());
    }
    if cmakeconan::is_available() {
        managers.push("cmake".to_string());
    }
    if swift::is_available() {
        managers.push("swift".to_string());
    }
    if bazel::is_available() {
        managers.push("bazel".to_string());
    }
    if poetry::is_available() {
        managers.push("poetry".to_string());
    }
    if pipenv::is_available() {
        managers.push("pipenv".to_string());
    }
    if requirements::is_available() {
        managers.push("pip".to_string());
    }
    if rye::is_available() {
        managers.push("rye".to_string());
    }
    if lein::is_available() {
        managers.push("lein".to_string());
    }
    if rebar3::is_available() {
        managers.push("rebar3".to_string());
    }
    if mix::is_available() {
        managers.push("mix".to_string());
    }
    if dart::is_available() {
        managers.push("dart".to_string());
    }
    managers
}

pub fn get_manager_file_name(runner: &str) -> &'static str {
    match runner {
        "maven" => "pom.xml",
        "gradle" => gradle::get_gradle_build_file(),
        "sbt" => "build.sbt",
        "npm" => "package.json",
        "cargo" => "Cargo.toml",
        "cmake" => "CMakeLists.txt, conanfile.txt",
        "composer" => "composer.json",
        "go" => "go.mod",
        "swift" => "Package.swift",
        "bundle" => "Gemfile",
        "bazel" => "WORKSPACE",
        "poetry" => "pyproject.toml",
        "lein" => "project.clj",
        "rebar3" => "rebar.config",
        "mix" => "mix.exs",
        "pip" => "requirements.txt",
        "rye" => "requirements.lock",
        "pipenv" => "Pipfile",
        "dart" => "pubspec.yaml",
        _ => "unknown",
    }
}

pub fn get_manager_web_url(runner: &str) -> &'static str {
    match runner {
        "maven" => "https://maven.apache.org",
        "gradle" => "https://gradle.org",
        "sbt" => "https://www.scala-sbt.org",
        "npm" => "https://nodejs.org",
        "cargo" => "https://doc.rust-lang.org/cargo/",
        "cmake" => "https://cmake.org/",
        "composer" => "https://getcomposer.org",
        "go" => "https://go.dev/ref/mod",
        "swift" => "https://www.swift.org/package-manager/",
        "bundle" => "https://bundler.io/",
        "bazel" => "https://bazel.build/",
        "poetry" => "https://python-poetry.org",
        "lein" => "https://leiningen.org",
        "rebar3" => "https://rebar3.org",
        "mix" => "https://hexdocs.pm/mix/Mix.html",
        "pip" => "https://pip.pypa.io/en/stable/reference/requirements-file-format/",
        "pipenv" => "https://pipenv.pypa.io",
        "rye" => "https://github.com/mitsuhiko/rye",
        "dart" => "https://dart.dev/guides/packages",
        _ => "unknown",
    }
}

pub fn get_manager_command_map(runner: &str) -> HashMap<String, String> {
    match runner {
        "maven" => maven::get_task_command_map(),
        "gradle" => gradle::get_task_command_map(),
        "sbt" => sbt::get_task_command_map(),
        "npm" => npm::get_task_command_map(),
        "cargo" => cargo::get_task_command_map(),
        "composer" => composer::get_task_command_map(),
        "go" => golang::get_task_command_map(),
        "cmake" => cmakeconan::get_task_command_map(),
        "bundle" => bundler::get_task_command_map(),
        "swift" => swift::get_task_command_map(),
        "bazel" => bazel::get_task_command_map(),
        "poetry" => poetry::get_task_command_map(),
        "lein" => lein::get_task_command_map(),
        "rebar3" => rebar3::get_task_command_map(),
        "mix" => mix::get_task_command_map(),
        "pip" => requirements::get_task_command_map(),
        "pipenv" => pipenv::get_task_command_map(),
        "rye" => rye::get_task_command_map(),
        "dart" => dart::get_task_command_map(),
        _ => HashMap::new(),
    }
}

pub fn run_task(runner: &str, task_name: &str, task_args: &[&str], global_args: &[&str], verbose: bool) -> Result<(), KeeperError> {
    let mut queue: HashMap<&str, fn(&str, &[&str], &[&str], bool) -> Result<Output, KeeperError>> = HashMap::new();
    if maven::is_available() {
        if maven::is_command_available() {
            queue.insert("maven", maven::run_task);
        } else {
            println!("{}", format!("[tk] maven(https://maven.apache.org/) command not available for pom.xml").bold().red());
        }
    }
    if gradle::is_available() {
        if gradle::is_command_available() {
            queue.insert("gradle", gradle::run_task);
        } else {
            println!("{}", format!("[tk] gradle(https://gradle.org/) command not available").bold().red());
        }
    }
    if sbt::is_available() {
        if sbt::is_command_available() {
            queue.insert("sbt", sbt::run_task);
        } else {
            println!("{}", format!("[tk] sbt(https://www.scala-sbt.org/) command not available for build.sbt").bold().red());
        }
    }
    if npm::is_available() {
        if npm::is_command_available() {
            if npm::get_task_command_map().contains_key(task_name) {
                queue.insert("npm", npm::run_task);
            }
        } else {
            println!("{}", format!("[tk] npm(https://nodejs.org/) command not available for package.json").bold().red());
        }
    }
    if cargo::is_available() {
        if cargo::is_command_available() {
            queue.insert("cargo", cargo::run_task);
        } else {
            println!("{}", format!("[tk] cargo(https://gradle.org/) command not available for Cargo.toml").bold().red());
        }
    }
    if composer::is_available() {
        if composer::is_command_available() {
            if composer::get_task_command_map().contains_key(task_name) {
                queue.insert("composer", composer::run_task);
            }
        } else {
            println!("{}", format!("[tk] gradle(https://gradle.org/) command not available for composer.json").bold().red());
        }
    }
    if bundler::is_available() {
        if bundler::is_command_available() {
            queue.insert("bundle", bundler::run_task);
        } else {
            println!("{}", format!("[tk] bundle(https://bundler.io/) command not available for Gemfile").bold().red());
        }
    }
    if golang::is_available() {
        if golang::is_command_available() {
            queue.insert("go", golang::run_task);
        } else {
            println!("{}", format!("[tk] go(https://go.dev/) command not available for go.mod").bold().red());
        }
    }
    if cmakeconan::is_available() {
        if cmakeconan::is_command_available() {
            queue.insert("cmake", cmakeconan::run_task);
        } else {
            println!("{}", format!("[tk] cmake and conan(https://github.com/conan-io/cmake-conan/) command not available for CMakeLists.txt and conanfile.txt").bold().red());
        }
    }
    if swift::is_available() {
        if swift::is_command_available() {
            queue.insert("swift", swift::run_task);
        } else {
            println!("{}", format!("[tk] swift(https://www.swift.org/) command not available for Package.swift").bold().red());
        }
    }
    if bazel::is_available() {
        if bazel::is_command_available() {
            queue.insert("bazel", bazel::run_task);
        } else {
            println!("{}", format!("[tk] bazel(https://bazel.build/) command not available for WORKSPACE").bold().red());
        }
    }
    if poetry::is_available() {
        if poetry::is_command_available() {
            queue.insert("poetry", poetry::run_task);
        } else {
            println!("{}", format!("[tk] poetry(https://python-poetry.org/) command not available for pyproject.toml").bold().red());
        }
    }
    if pipenv::is_available() {
        if pipenv::is_command_available() {
            queue.insert("pipenv", pipenv::run_task);
        } else {
            println!("{}", format!("[tk] pipenv(https://pipenv.pypa.io/en/latest/) command not available for Pipfile").bold().red());
        }
    }
    if rye::is_available() {
        if rye::is_command_available() {
            queue.insert("rye", rye::run_task);
        } else {
            println!("{}", format!("[tk] rye(https://github.com/mitsuhiko/rye) command not available for requirements.lock").bold().red());
        }
    }
    if requirements::is_available() {
        if requirements::is_command_available() {
            queue.insert("requirements", requirements::run_task);
        } else {
            println!("{}", format!("[tk] pip(https://pypi.org/project/pip/) command not available for requirements.txt").bold().red());
        }
    }
    if lein::is_available() {
        if lein::is_command_available() {
            queue.insert("lein", lein::run_task);
        } else {
            println!("{}", format!("[tk] lein(https://leiningen.org/) command not available for project.clj").bold().red());
        }
    }
    if rebar3::is_available() {
        if rebar3::is_command_available() {
            queue.insert("rebar3", rebar3::run_task);
        } else {
            println!("{}", format!("[tk] rebar3(https://rebar3.readme.io/) command not available for rebar.config").bold().red());
        }
    }
    if mix::is_available() {
        if mix::is_command_available() {
            queue.insert("mix", mix::run_task);
        } else {
            println!("{}", format!("[tk] mix(https://hexdocs.pm/mix/1.13/Mix.html) command not available for mix.exs").bold().red());
        }
    }
    if dart::is_available() {
        if dart::is_command_available() {
            queue.insert("dart", dart::run_task);
        } else {
            println!("{}", format!("[tk] dart(https://dart.dev/guides/packages) command not available for pubspec.yaml").bold().red());
        }
    }
    if queue.is_empty() { // no manager found
        println!("{}", format!("[tk] no available manager detected").bold().red());
    } else if !runner.is_empty() { // run task by runner name
        if let Some(task) = queue.get(runner) {
            println!("{}", format!("[tk] execute {} from {}", task_name, runner).bold().blue());
            task(task_name, task_args, global_args, verbose).unwrap();
        } else {
            println!("{}", format!("[tk] {} manager not available", runner).bold().red());
        }
    } else { // run task by all available managers
        match task_name {
            "init" => {}
            /*"start" => { // only execute start task once
                if queue.len() == 1 {
                    queue.iter().for_each(|(runner_name, task)| {
                        println!("{}", format!("[tk] execute {} from {}", task_name, runner_name).bold().blue());
                        task(task_name, task_args, global_args, verbose).unwrap();
                    });
                } else {
                    let runner_names = queue.iter().map(|(runner_name, _task)| runner_name.to_owned()).collect::<Vec<_>>().join(",");
                    println!("{}", format!("[tk] Failed to run start because of multi start tasks from {}", runner_names).bold().red());
                }
            }*/
            _ => {
                queue.iter().for_each(|(runner_name, task)| {
                    println!("{}", format!("[tk] execute {} from {}", task_name, runner_name).bold().blue());
                    task(task_name, task_args, global_args, verbose).unwrap();
                });
            }
        }
    }
    Ok(())
}
