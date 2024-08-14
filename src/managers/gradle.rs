use std::collections::HashMap;
use std::process::Output;
use error_stack::{report, Result};
use which::which;
use crate::command_utils::{run_command_line};
use crate::errors::KeeperError;

pub fn is_available() -> bool {
    std::env::current_dir()
        .map(|dir| dir.join("build.gradle").exists()
            || dir.join("build.gradle.kts").exists()
            || dir.join("settings.gradle").exists()
            || dir.join("settings.gradle.kts").exists()
        )
        .unwrap_or(false)
}

pub fn is_command_available() -> bool {
    which("./gradlew").is_ok() || which("gradle").is_ok()
}

pub fn get_task_command_map() -> HashMap<String, String> {
    let mut task_command_map = HashMap::new();
    let gradle_command = get_gradle_command();
    task_command_map.insert("init".to_string(), "gradle init".to_string());
    task_command_map.insert("install".to_string(), format!("{} --refresh-dependencies classes dependencies", gradle_command));
    task_command_map.insert("compile".to_string(), format!("{} classes testClasses", gradle_command));
    task_command_map.insert("build".to_string(), format!("{} assemble", gradle_command));
    if let Some(start_command) = get_start_command_line() {
        task_command_map.insert("start".to_string(), start_command);
    }
    task_command_map.insert("test".to_string(), format!("{} test", gradle_command));
    task_command_map.insert("deps".to_string(), format!("{} dependencies", gradle_command));
    task_command_map.insert("doc".to_string(), format!("{} javadoc", gradle_command));
    task_command_map.insert("clean".to_string(), format!("{} clean", gradle_command));
    task_command_map.insert("outdated".to_string(), format!("{} dependencyUpdates", gradle_command));
    if let Ok(code) = std::fs::read_to_string("gradle/wrapper/gradle-wrapper.properties") {
        if !code.contains("gradle-8.10") {
            task_command_map.insert("self-update".to_string(), format!("{} wrapper --gradle-version=8.10", gradle_command));
        }
    }
    task_command_map
}

pub fn run_task(task: &str, _task_args: &[&str], _global_args: &[&str], verbose: bool) -> Result<Output, KeeperError> {
    if let Some(command_line) = get_task_command_map().get(task) {
        run_command_line(command_line, verbose)
    } else {
        Err(report!(KeeperError::ManagerTaskNotFound(task.to_owned(), "gradle".to_string())))
    }
}

fn get_gradle_command() -> &'static str {
    let wrapper_available = std::env::current_dir()
        .map(|dir| dir.join("gradlew").exists())
        .unwrap_or(false);
    if wrapper_available {
        "./gradlew"
    } else {
        "gradle"
    }
}

pub fn get_gradle_build_file() -> &'static str {
    let gradle_with_kotlin = std::env::current_dir()
        .map(|dir| dir.join("build.gradle.kts").exists())
        .unwrap_or(false);
    if gradle_with_kotlin {
        "build.gradle.kts"
    } else {
        "build.gradle"
    }
}

fn get_start_command_line() -> Option<String> {
    let build_gradle_file = get_gradle_build_file();
    if std::env::current_dir().map(|dir| dir.join(build_gradle_file).exists()).unwrap_or(false) {
        let gradle_build_code = std::env::current_dir()
            .map(|dir| dir.join(build_gradle_file))
            .map(|path| std::fs::read_to_string(path).unwrap())
            .unwrap_or("".to_owned());
        if (build_gradle_file == "build.gradle.kt" && gradle_build_code.contains(r#"id("org.springframework.boot")"#))
            || (build_gradle_file == "build.gradle" && gradle_build_code.contains(r#"id 'org.springframework.boot'"#)) {
            return Some(format!("{} bootRun", get_gradle_command()));
        } else if (build_gradle_file == "build.gradle.kt" && gradle_build_code.contains(r#"id("io.quarkus")"#))
            || (build_gradle_file == "build.gradle" && gradle_build_code.contains(r#"id 'io.quarkus'"#)) {
            return Some(format!("{} --console=plain quarkusDev", get_gradle_command()));
        }
    }
    None
}
