use crate::command_utils::{run_command_line, CommandOutput};
use crate::errors::KeeperError;
use crate::models::Task;
use crate::task;
use error_stack::{Report, ResultExt};
use std::env;
use std::io::{BufRead, BufReader};

pub fn is_available() -> bool {
    std::env::current_dir()
        .map(|dir| dir.join("Procfile").exists())
        .unwrap_or(false)
}

pub fn list_tasks() -> Result<Vec<Task>, Report<KeeperError>> {
    let procfile_text = env::current_dir()
        .map(|dir| dir.join("Procfile"))
        .map(|path| std::fs::read_to_string(path).unwrap())
        .change_context(KeeperError::InvalidProcfile)?;
    let tasks: Vec<Task> = BufReader::new(procfile_text.as_bytes())
        .lines()
        .filter(|line| line.is_ok() && line.as_ref().unwrap().contains(":"))
        .map(|line| line.unwrap())
        .map(|line| {
            let mut parts = line.splitn(2, ':');
            let name = parts.next().unwrap().trim();
            let mut command = parts.next().unwrap().trim().to_string();
            if command.contains("$PORT") {
                let port_env = env::var("PORT").unwrap_or_else(|_| "8000".to_string());
                command = command.replace("$PORT", &port_env);
            }
            task!(name, "proc", command)
        })
        .collect();
    Ok(tasks)
}

pub fn run_task(
    task: &str,
    _task_args: &[&str],
    _global_args: &[&str],
    verbose: bool,
) -> Result<CommandOutput, Report<KeeperError>> {
    let tasks = list_tasks()?;
    let task = tasks
        .iter()
        .find(|t| t.name == task)
        .ok_or_else(|| KeeperError::TaskNotFound(task.to_string()))?;
    run_command_line(&task.description, verbose)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command() {
        let args = shlex::split("java -jar demo.jar \"demo dh\"").unwrap();
        println!("{:?}", args);
    }

    #[test]
    fn test_parse() {
        if let Ok(tasks) = list_tasks() {
            println!("{:?}", tasks);
        }
    }

    #[test]
    fn test_run() {
        if let Ok(output) = run_task("my-ip", &[], &[], true) {
            let status_code = output.status.code().unwrap_or(0);
            println!("exit code: {}", status_code);
        }
    }
}
