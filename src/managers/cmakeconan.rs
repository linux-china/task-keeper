use std::collections::HashMap;
use std::process::Output;
use error_stack::{report, Result};
use which::which;
use crate::command_utils::{run_command_line};
use crate::errors::KeeperError;

pub fn is_available() -> bool {
    std::env::current_dir()
        .map(|dir| dir.join("conanfile.txt").exists() && dir.join("CMakeLists.txt").exists())
        .unwrap_or(false)
}

pub fn is_command_available() -> bool {
    which("cmake").is_ok() && which("conan").is_ok()
}

pub fn get_task_command_map() -> HashMap<String, String> {
    let mut task_command_map = HashMap::new();
    let cmake_binary_dir = get_build_dir();
    task_command_map.insert("install".to_string(), format!("conan install . -s build_type=Debug --install-folder={}", cmake_binary_dir));
    task_command_map.insert("compile".to_string(), format!("cmake -B {} -DCMAKE_BUILD_TYPE=Debug", cmake_binary_dir));
    task_command_map.insert("build".to_string(), format!("cmake --build {}", cmake_binary_dir));
    task_command_map.insert("deps".to_string(), "conan info .".to_string());
    task_command_map.insert("clean".to_string(), format!("cmake --build {} --target clean", cmake_binary_dir));
    task_command_map
}

pub fn run_task(task: &str, _extra_args: &[&str], verbose: bool) -> Result<Output, KeeperError> {
    if task == "build" {
        if !std::env::current_dir().map(|dir| dir.join("cmake-build-debug").exists()).unwrap_or(false) {
            // run compile first
            run_command_line(get_task_command_map().get("compile").unwrap(), verbose)?;
        }
    }
    if let Some(command_line) = get_task_command_map().get(task) {
        run_command_line(command_line, verbose)
    } else {
        Err(report!(KeeperError::ManagerTaskNotFound(task.to_owned(), "cmake".to_string())))
    }
}

fn get_build_dir() -> String {
    std::env::var("CMAKE_BINARY_DIR").unwrap_or("cmake-build-debug".to_string())
}
