use crate::polyglot::PATH_SEPARATOR;
use std::env;
use std::path::{Path, PathBuf};

const NODE_EXECUTABLE: &str = if cfg!(windows) { "node.exe" } else { "node" };

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
        for (candidates_path, sub_dir) in node_candidates_paths() {
            if let Some(node_home) = find_node_home(node_version, &candidates_path, sub_dir) {
                return Some(node_home);
            }
        }
    }
    None
}

/// Directories that contain installed Node.js versions, with the sub directory of node home in each version directory.
fn node_candidates_paths() -> Vec<(PathBuf, Option<&'static str>)> {
    let env_path = |name: &str| {
        env::var_os(name)
            .filter(|v| !v.is_empty())
            .map(PathBuf::from)
    };
    let mut candidates: Vec<(PathBuf, Option<&'static str>)> = vec![];
    // nvm
    if cfg!(windows) {
        // nvm-windows: %NVM_HOME%\v<ver>, default %APPDATA%\nvm or %LOCALAPPDATA%\nvm
        if let Some(nvm_home) = env_path("NVM_HOME") {
            candidates.push((nvm_home, None));
        }
        if let Some(dir) = dirs::data_dir() {
            candidates.push((dir.join("nvm"), None));
        }
        if let Some(dir) = dirs::data_local_dir() {
            candidates.push((dir.join("nvm"), None));
        }
    } else if let Some(nvm_dir) =
        env_path("NVM_DIR").or_else(|| dirs::home_dir().map(|dir| dir.join(".nvm")))
    {
        candidates.push((nvm_dir.join("versions").join("node"), None));
    }
    // volta
    let volta_home = env_path("VOLTA_HOME").or_else(|| {
        if cfg!(windows) {
            dirs::data_local_dir().map(|dir| dir.join("Volta"))
        } else {
            dirs::home_dir().map(|dir| dir.join(".volta"))
        }
    });
    if let Some(volta_home) = volta_home {
        candidates.push((volta_home.join("tools").join("image").join("node"), None));
    }
    // fnm: <FNM_DIR>/node-versions/v<ver>/installation
    let mut fnm_dirs: Vec<PathBuf> = vec![];
    if let Some(fnm_dir) = env_path("FNM_DIR") {
        fnm_dirs.push(fnm_dir);
    }
    if let Some(dir) = dirs::data_dir() {
        fnm_dirs.push(dir.join("fnm"));
    }
    if let Some(dir) = dirs::home_dir() {
        fnm_dirs.push(dir.join(".fnm"));
    }
    for fnm_dir in fnm_dirs {
        candidates.push((fnm_dir.join("node-versions"), Some("installation")));
    }
    candidates
}

pub fn init_env() {
    if let Some(node_home) = find_sdk_home() {
        reset_node_home(&node_home);
    }
}

fn find_node_home(
    node_version: &str,
    node_candidates_home: &Path,
    sub_dir: Option<&str>,
) -> Option<PathBuf> {
    if let Ok(paths) = std::fs::read_dir(node_candidates_home) {
        for path in paths.flatten() {
            if let Some(file_name) = path.file_name().to_str() {
                let real_node_version = file_name.strip_prefix('v').unwrap_or(file_name);
                if real_node_version.starts_with(node_version) {
                    let node_home = match sub_dir {
                        Some(sub_dir) => path.path().join(sub_dir),
                        None => path.path(),
                    };
                    if node_bin_path(&node_home).join(NODE_EXECUTABLE).is_file() {
                        return Some(node_home);
                    }
                }
            }
        }
    }
    None
}

/// Node.js binaries are in `bin/` on Unix, but directly in node home on Windows.
fn node_bin_path(node_home: &Path) -> PathBuf {
    if cfg!(windows) {
        node_home.to_path_buf()
    } else {
        node_home.join("bin")
    }
}

fn reset_node_home(node_home_path: &PathBuf) {
    let node_home = node_home_path.to_string_lossy().to_string();
    unsafe {
        env::set_var("NODE_HOME", &node_home);
    }
    if let Ok(path) = env::var("PATH") {
        let node_bin_path = node_bin_path(node_home_path).to_string_lossy().to_string();
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
