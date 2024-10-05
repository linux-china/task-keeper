use std::env;

pub fn is_available() -> bool {
    env::current_dir()
        .map(|dir| dir.join(".ruby-version").exists())
        .unwrap_or(false)
}

pub fn get_default_version() -> std::io::Result<String> {
    std::fs::read_to_string(".ruby-version").map(|text| text.trim().to_string())
}
