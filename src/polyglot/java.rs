use std::env;
use std::path::PathBuf;
use crate::polyglot::PATH_SEPARATOR;

pub fn is_available() -> bool {
    env::current_dir()
        .map(|dir| dir.join(".java-version").exists())
        .unwrap_or(false)
}

pub fn init_env() {
    if let Ok(text) = std::fs::read_to_string(".java-version") {
        let java_version = text.trim();
        let mut java_found = false;
        if let Some(java_home) = dirs::home_dir()
            .map(|dir| {
                dir.join(".jbang").join("cache").join("jdks").join(java_version)
            })
            .filter(|dir| dir.exists()) {
            java_found = true;
            reset_java_home(java_version, &java_home);
        }
        if !java_found {
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
                                    reset_java_home(java_version, &path.path());
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn reset_java_home(java_version: &str, java_home_path: &PathBuf) {
    let java_home = java_home_path.to_string_lossy().to_string();
    env::set_var("JAVA_HOME", &java_home);
    if java_version.ends_with("-grl") {
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
