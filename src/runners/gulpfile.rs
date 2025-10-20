use crate::command_utils::{capture_command_output, run_command, CommandOutput};
use crate::errors::KeeperError;
use crate::models::Task;
use crate::task;
use error_stack::Report;
use which::which;

pub fn is_available() -> bool {
    std::env::current_dir()
        .map(|dir| dir.join("gulpfile.js").exists() || dir.join("Gulpfile.js").exists())
        .unwrap_or(false)
}

pub fn is_command_available() -> bool {
    which("gulp").is_ok()
}

pub fn list_tasks() -> Result<Vec<Task>, Report<KeeperError>> {
    let jake_tasks_output = capture_command_output("gulp", &["--tasks-simple"])
        .map(|output| String::from_utf8(output.stdout).unwrap_or("".to_owned()))?;
    let tasks: Vec<Task> = jake_tasks_output
        .lines()
        .map(|line| {
            let task_name = line.trim().to_owned();
            task!(task_name, "gulp")
        })
        .collect();
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
    run_command("gulp", &args, verbose)
}
