use std::env;
use std::path::PathBuf;
use crate::polyglot::PATH_SEPARATOR;

pub fn is_available() -> bool {
    let current_dir = env::current_dir().unwrap();
    current_dir.join(".java-version").exists()
        || current_dir.join(".sdkmanrc").exists()
        || current_dir.join("pom.xml").exists()
        || current_dir.join("build.gradle.kts").exists()
        || current_dir.join("build.gradle").exists()
}

pub fn get_default_version() -> Option<String> {
    if let Ok(text) = std::fs::read_to_string(".java-version") {
        return Some(text.trim().to_string());
    } else if let Ok(text) = std::fs::read_to_string(".sdkmanrc") {
        let map = java_properties::read(text.as_bytes()).unwrap();
        return map.get("java").map(|version| version.to_string());
    } else if let Ok(xml) = std::fs::read_to_string("pom.xml") {
        return extract_java_version_from_pom(&xml);
    } else if let Ok(code) = std::fs::read_to_string("build.gradle.kts") {
        return extract_java_version_from_gradle(&code);
    } else if let Ok(code) = std::fs::read_to_string("build.gradle") {
        return extract_java_version_from_gradle(&code);
    }
    None
}

fn extract_java_version_from_pom(xml: &str) -> Option<String> {
    if let Some(offset) = xml.find("<java.version>") {
        let end = xml.find("</java.version").unwrap();
        let java_version = xml[offset + 14..end].trim();
        if str::parse::<u32>(java_version).is_ok() {
            return Some(java_version.to_string());
        }
    }
    None
}


fn extract_java_version_from_gradle(code: &str) -> Option<String> {
    if let Some(offset) = code.find("JavaLanguageVersion.of(") {
        let start = offset + 23;
        let end = code[start..].find(")").unwrap();
        let java_version = code[start..start + end].trim();
        if str::parse::<u32>(java_version).is_ok() {
            return Some(java_version.to_string());
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
    unsafe {
        env::set_var("JAVA_HOME", &java_home);
    }
    if java_home.contains("-grl") {
        unsafe {
            env::set_var("GRAALVM_HOME", &java_home);
        }
    }
    if let Ok(path) = env::var("PATH") {
        let java_bin_path = java_home_path.join("bin").to_string_lossy().to_string();
        unsafe {
            env::set_var("PATH", format!("{}{}{}", java_bin_path, PATH_SEPARATOR, path));
        }
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

    #[test]
    fn test_extract_java_version_from_pom() {
        let xml = r#"
            <project>
                <properties>
                    <java.version>11</java.version>
                </properties>
            </project>
            "#;
        assert_eq!(Some("11".to_string()), extract_java_version_from_pom(xml));
    }

    #[test]
    fn test_extract_java_version_from_gradle() {
        let code = r#"
            plugins {
                id("java")
            }
            java {
                toolchain {
                    languageVersion.set(JavaLanguageVersion.of(11))
                }
            }
        "#;
        assert_eq!(Some("11".to_string()), extract_java_version_from_gradle(code));
    }
}
