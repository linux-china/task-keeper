use crate::command_utils::{run_command_line, CommandOutput};
use crate::errors::KeeperError;
use error_stack::{report, Result};
use serde::Deserialize;
use serde_xml_rs::from_str;
use std::collections::HashMap;
use which::which;

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
    task_command_map.insert(
        "install".to_string(),
        format!("{} -U dependency:resolve", mvn_command),
    );
    task_command_map.insert(
        "compile".to_string(),
        format!("{} compile test-compile", mvn_command),
    );
    task_command_map.insert(
        "build".to_string(),
        format!("{} -DskipTests package", mvn_command),
    );
    task_command_map.insert("start".to_string(), get_start_command_line());
    task_command_map.insert("test".to_string(), format!("{} test", mvn_command));
    task_command_map.insert(
        "deps".to_string(),
        format!("{} dependency:tree", mvn_command),
    );
    task_command_map.insert(
        "doc".to_string(),
        format!("{} javadoc:javadoc", mvn_command),
    );
    task_command_map.insert("clean".to_string(), format!("{}  clean", mvn_command));
    task_command_map.insert(
        "outdated".to_string(),
        format!("{} versions:display-dependency-updates", mvn_command),
    );
    if std::env::current_dir()
        .map(|dir| dir.join(".mvn/wrapper").exists())
        .unwrap_or(false)
    {
        if let Ok(code) = std::fs::read_to_string(".mvn/wrapper/maven-wrapper.properties") {
            if !code.contains("apache-maven-3.9.9") {
                task_command_map.insert("self-update".to_string(), format!("{} org.apache.maven.plugins:maven-wrapper-plugin:3.3.0:wrapper -Dmaven=3.9.9", mvn_command));
            }
        }
    }
    task_command_map
}

pub fn run_task(
    task: &str,
    _task_args: &[&str],
    _global_args: &[&str],
    verbose: bool,
) -> Result<CommandOutput, KeeperError> {
    if let Some(command_line) = get_task_command_map().get(task) {
        run_command_line(command_line, verbose)
    } else {
        Err(report!(KeeperError::ManagerTaskNotFound(
            task.to_owned(),
            "maven".to_string()
        )))
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
        || pom_xml.contains("<artifactId>spring-boot-starter-webflux</artifactId>")
    {
        format!("{} spring-boot:run", get_mvn_command())
    } else if pom_xml.contains("<artifactId>quarkus-maven-plugin</artifactId>") {
        format!("{} quarkus:dev", get_mvn_command())
    } else {
        format!("{} exec:java", get_mvn_command())
    };
}

#[derive(Deserialize, Debug)]
pub struct Metadata {
    #[serde(rename = "groupId")]
    pub group_id: String,
    #[serde(rename = "artifactId")]
    pub artifact_id: String,
    pub versioning: Versioning,
}
#[derive(Deserialize, Debug)]
pub struct Versioning {
    pub latest: String,
    pub release: String,
    #[serde(rename = "lastUpdated")]
    pub last_updated: String,
    pub versions: Versions,
}

#[derive(Debug, Deserialize)]
pub struct Versions {
    #[serde(rename = "version")]
    pub versions: Vec<String>,
}

pub fn parse_maven_metadata(url: &str) -> Result<Metadata, KeeperError> {
    let text = reqwest::blocking::get(url).unwrap().text().unwrap();
    from_str(&text).map_err(|_| report!(KeeperError::InvalidMavenMetadataXml))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_metadata_xml() {
        let url = "https://packages.jetbrains.team/maven/p/amper/amper/org/jetbrains/amper/cli/maven-metadata.xml";
        let metadata: Metadata = parse_maven_metadata(url).unwrap();
        println!("{:?}", metadata);
    }
}
