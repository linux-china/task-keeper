use std::collections::HashMap;

pub fn get_task_command_map() -> HashMap<String, String> {
    let mut task_command_map = HashMap::new();
    task_command_map.insert("init".to_string(), "cargo new {name}".to_string());
    task_command_map.insert("compile".to_string(), "cargo build".to_string());
    task_command_map.insert("build".to_string(), "cargo build".to_string());
    task_command_map.insert("test".to_string(), "cargo test".to_string());
    task_command_map.insert("deps".to_string(), "cargo tree".to_string());
    task_command_map.insert("doc".to_string(), "cargo doc".to_string());
    task_command_map.insert("clean".to_string(), "cargo clean".to_string());
    task_command_map.insert("outdated".to_string(), "cargo outdated".to_string());
    task_command_map.insert("update".to_string(), "cargo update".to_string());
    task_command_map
}

