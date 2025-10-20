use crate::command_utils::{capture_command_output, run_command, CommandOutput};
use crate::errors::KeeperError;
use crate::models::Task;
use crate::task;
use std::io::{BufRead, BufReader};
use error_stack::Report;

pub fn is_available() -> bool {
    std::env::current_dir()
        .map(|dir| dir.join("task.sh").exists())
        .unwrap_or(false)
}

pub fn list_tasks() -> Result<Vec<Task>, Report<KeeperError>> {
    let makefile_meta_text = capture_command_output("./task.sh", &[])
        .map(|output| String::from_utf8(output.stdout).unwrap_or("{}".to_owned()))?;
    let tasks: Vec<Task> = BufReader::new(makefile_meta_text.as_bytes())
        .lines()
        .filter(|line| line.is_ok() && line.as_ref().unwrap().starts_with("commands:"))
        .map(|line| line.unwrap()[9..].to_owned())
        .flat_map(|line| {
            line.split_whitespace()
                .into_iter()
                .map(|task_name| task!(task_name, "shell"))
                .collect::<Vec<Task>>()
        })
        .collect::<Vec<Task>>();
    Ok(tasks)
}

pub fn run_task(
    task: &str,
    task_args: &[&str],
    global_args: &[&str],
    verbose: bool,
) -> Result<CommandOutput, Report<KeeperError>> {
    let mut args = vec![];
    args.extend(global_args);
    args.push(task);
    args.extend(task_args);
    run_command("./task.sh", &args, verbose)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        if let Ok(tasks) = list_tasks() {
            println!("{:?}", tasks);
        }
    }

    #[test]
    fn test_run() {
        if let Ok(output) = run_task("start", &[], &[], true) {
            let status_code = output.status.code().unwrap_or(0);
            println!("exit code: {}", status_code);
        }
    }
}
