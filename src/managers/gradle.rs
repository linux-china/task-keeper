use std::collections::HashMap;

pub fn is_available() -> bool {
    std::env::current_dir()
        .map(|dir| dir.join("build.gradle").exists() || dir.join("build.gradle.kts").exists())
        .unwrap_or(false)
}

pub fn is_command_available() -> bool {
    which("./gradlew").is_ok() || which("gradle").is_ok()
}

pub fn get_task_command_map() -> HashMap<String, String> {
    let mut task_command_map = HashMap::new();
    task_command_map.insert("init".to_string(), "gradle init".to_string());
    task_command_map.insert("install".to_string(), "./gradlew classes dependencies".to_string());
    task_command_map.insert("compile".to_string(), "./gradlew classes testClasses".to_string());
    task_command_map.insert("build".to_string(), "./gradlew assemble".to_string());
    task_command_map.insert("build".to_string(), "./gradlew run".to_string());
    task_command_map.insert("test".to_string(), "./gradlew test".to_string());
    task_command_map.insert("deps".to_string(), "./gradlew dependencies".to_string());
    task_command_map.insert("doc".to_string(), "./gradlew javadoc".to_string());
    task_command_map.insert("clean".to_string(), "./gradlew clean".to_string());
    task_command_map.insert("outdated".to_string(), "./gradlew dependencyUpdates".to_string());
    task_command_map
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
