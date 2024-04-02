use std::env;
use std::path::PathBuf;
use colored::Colorize;
use crate::polyglot::PATH_SEPARATOR;

pub fn is_available() -> bool {
    let current_dir = env::current_dir().unwrap();
    return current_dir.join(".java-version").exists()
        || current_dir.join("pom.xml").exists();
}

pub fn get_default_version() -> Option<String> {
    if let Ok(text) = std::fs::read_to_string(".java-version") {
        return Some(text.trim().to_string());
    } else if let Ok(xml) = std::fs::read_to_string("pom.xml") {
        if let Some(offset) = xml.find("<java.version>") {
            let end = xml.find("</java.version").unwrap();
            let java_version = xml[offset + 14..end].trim();
            if str::parse::<u32>(java_version).is_ok() {
                return Some(java_version.to_string());
            }
        }
    }
    None
}

pub fn find_sdk_home() -> Option<PathBuf> {
    if let Some(text) = get_default_version() {
        let java_version = text.trim();
        if let Some(java_home) = dirs::home_dir()
            .map(|dir| {
                dir.join(".jbang").join("cache").join("jdks").join(java_version)
            })
            .filter(|dir| dir.exists()) {
            return Some(java_home);
        }
        if let Some(java_candidates_home) = dirs::home_dir()
            .map(|dir| {
                dir.join(".sdkman").join("candidates").join("java")
            })
            .filter(|dir| dir.exists()) {
            if let Ok(paths) = std::fs::read_dir(java_candidates_home) {
                for path in paths {
                    if let Ok(path) = path {
                        if let Some(name) = path.file_name().to_str() {
                            if name.starts_with(java_version) {
                                return Some(path.path());
                            }
                        }
                    }
                }
            }
        }
    }
    None
}

pub fn init_env() {
    if let Some(java_home) = find_sdk_home() {
        reset_java_home(&java_home);
    }
}

fn reset_java_home(java_home_path: &PathBuf) {
    let java_home = java_home_path.to_string_lossy().to_string();
    env::set_var("JAVA_HOME", &java_home);
    if java_home.contains("-grl") {
        env::set_var("GRAALVM_HOME", &java_home);
    }
    if let Ok(path) = env::var("PATH") {
        let java_bin_path = java_home_path.join("bin").to_string_lossy().to_string();
        env::set_var("PATH", format!("{}{}{}", java_bin_path, PATH_SEPARATOR, path));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_env() {
        init_env();
        println!("JAVA_HOME: {}", env::var("JAVA_HOME").unwrap());
        println!("PATH: {}", env::var("PATH").unwrap());
    }
}
