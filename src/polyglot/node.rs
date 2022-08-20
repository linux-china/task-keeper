use std::env;
use std::path::PathBuf;
use crate::polyglot::PATH_SEPARATOR;

pub fn is_available() -> bool {
    env::current_dir()
        .map(|dir| dir.join(".node-version").exists())
        .unwrap_or(false)
}

pub fn init_env() {
    if let Ok(text) = std::fs::read_to_string(".node-version") {
        let node_version = text.trim();
        let node_found = false;
        // find version from nvm
        let node_candidates_path = dirs::home_dir()
            .map(|dir| {
                dir.join(".nvm").join("versions").join("node")
            });
        let mut node_home_path = find_node_home(node_version, &node_candidates_path);
        if node_home_path.is_none() {
            let node_candidates_path = dirs::home_dir()
                .map(|dir| {
                    dir.join(".volta").join("tools").join("image").join("node")
                });
            node_home_path = find_node_home(node_version, &node_candidates_path);
        }
        if let Some(node_home) = node_home_path {
            reset_node_home(&node_home);
        }
    }
}

fn find_node_home(node_version: &str, node_candidates_path: &Option<PathBuf>) -> Option<PathBuf> {
    if let Some(node_candidates_home) = node_candidates_path {
        if let Ok(paths) = std::fs::read_dir(node_candidates_home) {
            for path in paths {
                if let Ok(node_home) = path {
                    if let Some(name) = node_home.file_name().to_str() {
                        if name.contains(node_version) {
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
    env::set_var("NODE_HOME", &node_home);
    if let Ok(path) = env::var("PATH") {
        let node_bin_path = node_home_path.join("bin").to_string_lossy().to_string();
        env::set_var("PATH", format!("{}{}{}", node_bin_path, PATH_SEPARATOR, path));
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
