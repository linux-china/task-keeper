use std::env;
use std::path::PathBuf;
use std::path::Path;
use crate::polyglot::PATH_SEPARATOR;

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
        let python_version = text.trim();
        // find version from rye
        let mut python_home = dirs::home_dir()
            .map(|dir| {
                dir.join(".rye").join("py").join(format!("cpython@{}", python_version)).join("install")
            });
        if Path::new(python_home.as_ref().unwrap()).exists() {
            python_home = dirs::home_dir()
                .map(|dir| {
                    dir.join(".pyenv").join("versions").join(python_version)
                });
        }
        if Path::new(python_home.as_ref().unwrap()).exists() {
            return python_home;
        }
    }
    None
}

pub fn init_env() {
    if !env::current_dir()
        .map(|dir| dir.join("venv").exists() || dir.join(".venv").exists())
        .unwrap_or(false) {
        if let Some(python_home) = find_sdk_home() {
            reset_python_home(&python_home);
        }
    }
}

fn reset_python_home(python_home_path: &PathBuf) {
    if let Ok(path) = env::var("PATH") {
        let node_bin_path = python_home_path.join("bin").to_string_lossy().to_string();
        env::set_var("PATH", format!("{}{}{}", node_bin_path, PATH_SEPARATOR, path));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_env() {
        init_env();
        println!("PATH: {}", env::var("PATH").unwrap());
    }
}
