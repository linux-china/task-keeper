use crate::polyglot::PATH_SEPARATOR;
use std::env;
use std::path::Path;
use std::path::PathBuf;

pub fn is_available() -> bool {
    env::current_dir()
        .map(|dir| dir.join(".python-version").exists())
        .unwrap_or(false)
}

pub fn get_default_version() -> std::io::Result<String> {
    std::fs::read_to_string(".python-version").map(|text| text.trim().to_string())
}

pub fn find_sdk_home() -> Option<PathBuf> {
    if let Ok(text) = get_default_version() {
        #[cfg(windows)]
        let home_dir = dirs_sys::known_folder_profile().unwrap();
        #[cfg(not(windows))]
        let home_dir = dirs_sys::home_dir().unwrap();
        let python_version = text.trim();
        // find python from uv
        let python_versions_dir = home_dir
            .join(".local")
            .join("share")
            .join("uv")
            .join("python");
        if python_versions_dir.exists() {
            let prefix = format!("cpython-{}", python_version);
            if let Some(path) = find_sub_directory(&python_versions_dir, &prefix) {
                return Some(path);
            }
        }
        // find python from pyenv
        let python_home = home_dir
            .join(".pyenv")
            .join("versions")
            .join(python_version);
        if python_home.exists() {
            return Some(python_home);
        }
    }
    None
}

pub fn init_env() {
    if !env::current_dir()
        .map(|dir| dir.join("venv").exists() || dir.join(".venv").exists())
        .unwrap_or(false)
    {
        if let Some(python_home) = find_sdk_home() {
            reset_python_home(&python_home);
        }
    }
}

fn reset_python_home(python_home_path: &PathBuf) {
    if let Ok(path) = env::var("PATH") {
        let node_bin_path = python_home_path.join("bin").to_string_lossy().to_string();
        unsafe {
            env::set_var(
                "PATH",
                format!("{}{}{}", node_bin_path, PATH_SEPARATOR, path),
            );
        }
    }
}

pub fn find_sub_directory(dir: &Path, prefix: &str) -> Option<PathBuf> {
    if let Ok(paths) = std::fs::read_dir(dir) {
        for path in paths {
            if let Ok(sub_dir) = path {
                if let Some(file_name) = sub_dir.file_name().to_str() {
                    if file_name.starts_with(prefix) {
                        return Some(sub_dir.path());
                    }
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use dirs::home_dir;

    #[test]
    fn test_init_env() {
        init_env();
        println!("PATH: {}", env::var("PATH").unwrap());
    }

    #[test]
    fn test_find_sub_directory() {
        let home_dir = home_dir().unwrap();
        let python_versions_dir = home_dir
            .join(".local")
            .join("share")
            .join("uv")
            .join("python");
        let prefix = "cpython-3.11.9";
        let path = find_sub_directory(&python_versions_dir, prefix).unwrap();
        println!("path: {:?}", path);
    }
}
