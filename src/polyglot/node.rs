use crate::polyglot::PATH_SEPARATOR;
use std::env;
use std::path::PathBuf;

pub fn is_available() -> bool {
    env::current_dir()
        .map(|dir| dir.join(".node-version").exists())
        .unwrap_or(false)
}

pub fn get_default_version() -> std::io::Result<String> {
    std::fs::read_to_string(".node-version").map(|text| text.trim().to_string())
}

pub fn find_sdk_home() -> Option<PathBuf> {
    if let Ok(text) = get_default_version() {
        let node_version = text.trim();
        // find version from nvm
        let node_candidates_path =
            dirs::home_dir().map(|dir| dir.join(".nvm").join("versions").join("node"));
        let mut node_home_path = find_node_home(node_version, &node_candidates_path);
        if node_home_path.is_none() {
            let node_candidates_path = dirs::home_dir()
                .map(|dir| dir.join(".volta").join("tools").join("image").join("node"));
            node_home_path = find_node_home(node_version, &node_candidates_path);
        }
        return node_home_path;
    }
    None
}

pub fn init_env() {
    if let Some(node_home) = find_sdk_home() {
        reset_node_home(&node_home);
    }
}

fn find_node_home(node_version: &str, node_candidates_path: &Option<PathBuf>) -> Option<PathBuf> {
    if let Some(node_candidates_home) = node_candidates_path {
        if let Ok(paths) = std::fs::read_dir(node_candidates_home) {
            for path in paths {
                if let Ok(node_home) = path {
                    if let Some(file_name) = node_home.file_name().to_str() {
                        let mut real_node_version = file_name;
                        if file_name.starts_with('v') {
                            real_node_version = &file_name[1..];
                        }
                        if real_node_version.starts_with(node_version) {
                            return Some(node_home.path());
                        }
                    }
                }
            }
        }
    }
    None
}

fn reset_node_home(node_home_path: &PathBuf) {
    let node_home = node_home_path.to_string_lossy().to_string();
    unsafe {
        env::set_var("NODE_HOME", &node_home);
    }
    if let Ok(path) = env::var("PATH") {
        let node_bin_path = node_home_path.join("bin").to_string_lossy().to_string();
        unsafe {
            env::set_var(
                "PATH",
                format!("{}{}{}", node_bin_path, PATH_SEPARATOR, path),
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_env() {
        init_env();
        println!("NODE_HOME: {}", env::var("NODE_HOME").unwrap());
        println!("PATH: {}", env::var("PATH").unwrap());
    }
}
