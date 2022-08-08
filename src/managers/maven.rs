use std::collections::HashMap;
use std::process::Output;
use error_stack::{IntoReport, report, Result, ResultExt};
use which::which;
use crate::command_utils::{run_command_line};
use crate::errors::KeeperError;

pub fn is_available() -> bool {
    std::env::current_dir()
        .map(|dir| dir.join("pom.xml").exists())
        .unwrap_or(false)
}

pub fn is_command_available() -> bool {
    which("mvn").is_ok() || which("./mvnw").is_ok()
}

pub fn get_task_command_map() -> HashMap<String, String> {
    let mut task_command_map = HashMap::new();
    task_command_map.insert("init".to_string(), "mvn archetype:generate".to_string());
    task_command_map.insert("install".to_string(), "mvn dependency:resolve".to_string());
    task_command_map.insert("compile".to_string(), "mvn compile testCompile".to_string());
    task_command_map.insert("build".to_string(), "mvn -DskipTests package".to_string());
    task_command_map.insert("start".to_string(), "mvn compile exec:java".to_string());
    task_command_map.insert("test".to_string(), "mvn test".to_string());
    task_command_map.insert("deps".to_string(), "mvn dependency:tree".to_string());
    task_command_map.insert("doc".to_string(), "mvn javadoc:javadoc".to_string());
    task_command_map.insert("clean".to_string(), "mvn clean".to_string());
    task_command_map.insert("outdated".to_string(), "mvn versions:display-dependency-updates".to_string());
    task_command_map
}

pub fn run_task(task: &str, extra_args: &[&str], verbose: bool) -> Result<Output, KeeperError> {
    if let Some(command_line) = get_task_command_map().get(task) {
        run_command_line(command_line, verbose)
    } else {
        Err(report!(KeeperError::ManagerTaskNotFound(task.to_owned(), "maven".to_string())))
    }
}

fn get_mvn_command() -> &'static str {
    let wrapper_available = std::env::current_dir()
        .map(|dir| dir.join("mvnw").exists())
        .unwrap_or(false);
    if wrapper_available {
        "./mvnw"
    } else {
        "mvn"
    }
}
