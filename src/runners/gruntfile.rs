use crate::command_utils::{capture_command_output, run_command, CommandOutput};
use crate::errors::KeeperError;
use crate::models::Task;
use crate::task;
use error_stack::Report;
use which::which;

pub fn is_available() -> bool {
    std::env::current_dir()
        .map(|dir| dir.join("Gruntfile.js").exists() || dir.join("Gruntfile").exists())
        .unwrap_or(false)
}

pub fn is_command_available() -> bool {
    which("grunt").is_ok()
}

pub fn list_tasks() -> Result<Vec<Task>, Report<KeeperError>> {
    let grunt_help_output = capture_command_output("grunt", &["--help"])
        .map(|output| String::from_utf8(output.stdout).unwrap_or("".to_owned()))?;
    let lines = grunt_help_output.lines().collect::<Vec<&str>>();
    // extract lines between "Available tasks" and the next empty line
    let start_index = lines
        .iter()
        .position(|&line| line.trim() == "Available tasks")
        .map(|i| i + 1)
        .unwrap_or(0);
    let end_index = lines[start_index..]
        .iter()
        .position(|&line| line.trim().is_empty())
        .map(|i| start_index + i)
        .unwrap_or(lines.len());
    let tasks: Vec<Task> = lines[start_index..end_index]
        .iter()
        .map(|line| {
            let parts = line.trim().splitn(2, ' ').collect::<Vec<&str>>();
            let description = if parts.len() > 1 { parts[1].trim() } else { "" };
            task!(parts[0], "grunt", description)
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
    run_command("grunt", &args, verbose)
}
