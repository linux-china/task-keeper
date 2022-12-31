use std::collections::HashMap;
use std::process::Output;
use error_stack::{report, Result};
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
    let mvn_command = get_mvn_command();
    task_command_map.insert("init".to_string(), format!("{} archetype:generate", mvn_command));
    task_command_map.insert("install".to_string(), format!("{} -U dependency:resolve", mvn_command));
    task_command_map.insert("compile".to_string(), format!("{} compile test-compile", mvn_command));
    task_command_map.insert("build".to_string(), format!("{} -DskipTests package", mvn_command));
    task_command_map.insert("start".to_string(), get_start_command_line());
    task_command_map.insert("test".to_string(), format!("{} test", mvn_command));
    task_command_map.insert("deps".to_string(), format!("{} dependency:tree", mvn_command));
    task_command_map.insert("doc".to_string(), format!("{} javadoc:javadoc", mvn_command));
    task_command_map.insert("clean".to_string(), format!("{}  clean", mvn_command));
    task_command_map.insert("outdated".to_string(), format!("{} versions:display-dependency-updates", mvn_command));
    task_command_map
}

pub fn run_task(task: &str, _task_args: &[&str], _global_args: &[&str], verbose: bool) -> Result<Output, KeeperError> {
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

fn get_start_command_line() -> String {
    let pom_xml = std::env::current_dir()
        .map(|dir| dir.join("pom.xml"))
        .map(|path| std::fs::read_to_string(path).unwrap())
        .unwrap_or("<project></project>".to_owned());
    return if pom_xml.contains("<artifactId>spring-boot-starter-web</artifactId>")
        || pom_xml.contains("<artifactId>spring-boot-starter-webflux</artifactId>") {
        format!("{} spring-boot:run", get_mvn_command())
    } else if pom_xml.contains("<artifactId>quarkus-maven-plugin</artifactId>") {
        format!("{} quarkus:dev", get_mvn_command())
    } else {
        format!("{} exec:java", get_mvn_command())
    };
}
