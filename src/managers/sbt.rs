use std::collections::HashMap;

pub fn get_task_command_map() -> HashMap<String, String> {
    let mut task_command_map = HashMap::new();
    task_command_map.insert("init".to_string(), "sbt new scala/scala-seed.g8".to_string());
    task_command_map.insert("install".to_string(), "sbt update".to_string());
    task_command_map.insert("compile".to_string(), "sbt compile".to_string());
    task_command_map.insert("build".to_string(), "sbt package".to_string());
    task_command_map.insert("start".to_string(), "sbt run".to_string());
    task_command_map.insert("test".to_string(), "sbt test".to_string());
    task_command_map.insert("deps".to_string(), "sbt dependencyTree".to_string());
    task_command_map.insert("doc".to_string(), "sbt doc".to_string());
    task_command_map.insert("clean".to_string(), "sbt clean".to_string());
    task_command_map.insert("outdated".to_string(), "sbt dependencyUpdates".to_string());
    task_command_map.insert("update".to_string(), "sbt update".to_string());
    task_command_map
}

