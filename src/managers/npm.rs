use std::collections::HashMap;

pub fn get_task_command_map() -> HashMap<String, String> {
    let mut task_command_map = HashMap::new();
    task_command_map.insert("init".to_string(), "npm init".to_string());
    task_command_map.insert("compile".to_string(), "npm run compile".to_string());
    task_command_map.insert("build".to_string(), "npm run build".to_string());
    task_command_map.insert("test".to_string(), "npm run test".to_string());
    task_command_map.insert("deps".to_string(), "npm list".to_string());
    task_command_map.insert("doc".to_string(), "npm run doc".to_string());
    task_command_map.insert("clean".to_string(), "npm run clean".to_string());
    task_command_map.insert("outdated".to_string(), "npm outdated".to_string());
    task_command_map.insert("update".to_string(), "npm update".to_string());
    task_command_map
}
