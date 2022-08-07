use std::collections::HashMap;

pub fn get_task_command_map() -> HashMap<String, String> {
    let mut task_command_map = HashMap::new();
    task_command_map.insert("init".to_string(), "archetype:generate".to_string());
    task_command_map.insert("compile".to_string(), "mvn compile testCompile".to_string());
    task_command_map.insert("build".to_string(), "mvn -DskipTests package".to_string());
    task_command_map.insert("test".to_string(), "mvn test".to_string());
    task_command_map.insert("deps".to_string(), "mvn dependency:tree".to_string());
    task_command_map.insert("doc".to_string(), "mvn javadoc:javadoc".to_string());
    task_command_map.insert("clean".to_string(), "mvn clean".to_string());
    task_command_map.insert("outdated".to_string(), "mvn versions:display-dependency-updates".to_string());
    task_command_map
}
