use crate::command_utils::{run_command_line, CommandOutput};
use crate::errors::KeeperError;
use error_stack::{IntoReport, Report};
use std::collections::HashMap;
use which::which;

pub fn is_available() -> bool {
    std::env::current_dir()
        .map(|dir| dir.join("conanfile.txt").exists() || dir.join("CMakeLists.txt").exists())
        .unwrap_or(false)
}

pub fn is_command_available() -> bool {
    which("cmake").is_ok()
}

pub fn get_task_command_map() -> HashMap<String, String> {
    let mut task_command_map = HashMap::new();
    let cmake_binary_dir = get_build_dir();
    task_command_map.insert(
        "compile".to_string(),
        format!("cmake -B {} -DCMAKE_BUILD_TYPE=Debug", cmake_binary_dir),
    );
    task_command_map.insert(
        "build".to_string(),
        format!("cmake --build {}", cmake_binary_dir),
    );
    task_command_map.insert(
        "release".to_string(),
        format!(
            "cmake -DCMAKE_BUILD_TYPE=Release --build {}",
            cmake_binary_dir
        ),
    );
    task_command_map.insert(
        "clean".to_string(),
        format!("cmake --build {} --target clean", cmake_binary_dir),
    );
    if std::env::current_dir()
        .map(|dir| dir.join("CMakeLists.txt").exists())
        .unwrap_or(false)
    {
        task_command_map.insert(
            "install".to_string(),
            format!(
                "conan install . -s build_type=Debug --install-folder={}",
                cmake_binary_dir
            ),
        );
        task_command_map.insert("deps".to_string(), "conan info .".to_string());
    }
    task_command_map
}

pub fn run_task(
    task: &str,
    _task_args: &[&str],
    _global_args: &[&str],
    verbose: bool,
) -> Result<CommandOutput, Report<KeeperError>> {
    if task == "build" {
        if !std::env::current_dir()
            .map(|dir| dir.join("cmake-build-debug").exists())
            .unwrap_or(false)
        {
            // run compile first
            run_command_line(get_task_command_map().get("compile").unwrap(), verbose)?;
        }
    }
    if let Some(command_line) = get_task_command_map().get(task) {
        run_command_line(command_line, verbose)
    } else {
        Err(KeeperError::ManagerTaskNotFound(
            task.to_owned(),
            "cmake".to_string()
        ).into_report())
    }
}

fn get_build_dir() -> String {
    std::env::var("CMAKE_BINARY_DIR").unwrap_or("cmake-build-debug".to_string())
}
