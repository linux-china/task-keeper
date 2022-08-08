use std::collections::HashMap;

pub fn is_available() -> bool {
    std::env::current_dir()
        .map(|dir| dir.join("Cargo.toml").exists())
        .unwrap_or(false)
}

pub fn is_command_available() -> bool {
    which("cargo").is_ok()
}

pub fn get_task_command_map() -> HashMap<String, String> {
    let mut task_command_map = HashMap::new();
    task_command_map.insert("init".to_string(), "cargo new {name}".to_string());
    task_command_map.insert("install".to_string(), "cargo build".to_string());
    task_command_map.insert("compile".to_string(), "cargo build".to_string());
    task_command_map.insert("build".to_string(), "cargo build".to_string());
    task_command_map.insert("start".to_string(), "cargo run".to_string());
    task_command_map.insert("test".to_string(), "cargo test".to_string());
    task_command_map.insert("deps".to_string(), "cargo tree".to_string());
    task_command_map.insert("doc".to_string(), "cargo doc".to_string());
    task_command_map.insert("clean".to_string(), "cargo clean".to_string());
    task_command_map.insert("outdated".to_string(), "cargo outdated".to_string());
    task_command_map.insert("update".to_string(), "cargo update".to_string());
    task_command_map
}

