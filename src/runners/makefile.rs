use crate::command_utils::{capture_command_output, run_command, CommandOutput};
use crate::errors::KeeperError;
use crate::models::Task;
use crate::task;
use regex::Regex;
use std::io::{BufRead, BufReader};
use error_stack::Report;
use which::which;

pub fn is_available() -> bool {
    std::env::current_dir()
        .map(|dir| dir.join("Makefile").exists())
        .unwrap_or(false)
}

pub fn is_command_available() -> bool {
    which("make").is_ok()
}

pub fn list_tasks() -> Result<Vec<Task>, Report<KeeperError>> {
    if which::which("mmake").is_ok() {
        return list_tasks_by_mmake();
    }
    let makefile_meta_text = capture_command_output("make", &["-pRrq"])
        .map(|output| String::from_utf8(output.stdout).unwrap_or("".to_owned()))?;
    let re = Regex::new(r"^[a-zA-Z0-9][a-zA-Z0-9._-]*:.*").unwrap();

    let tasks: Vec<Task> = BufReader::new(makefile_meta_text.as_bytes())
        .lines()
        .filter(|line| line.is_ok() && re.is_match(line.as_ref().unwrap()))
        .map(|line| line.unwrap().split(':').nth(0).unwrap().to_owned())
        .filter(|task_name| task_name != "Makefile")
        .map(|task_name| task!(task_name, "make"))
        .collect();
    Ok(tasks)
}

fn list_tasks_by_mmake() -> Result<Vec<Task>, Report<KeeperError>> {
    let makefile_meta_text = capture_command_output("mmake", &["help"])
        .map(|output| String::from_utf8(output.stdout).unwrap_or("".to_owned()))?;
    let re = Regex::new(r"^[a-zA-Z0-9][a-zA-Z0-9._-]*.*").unwrap();
    let tasks: Vec<Task> = BufReader::new(makefile_meta_text.as_bytes())
        .lines()
        .filter(|line| line.is_ok() && re.is_match(line.as_ref().unwrap().trim()))
        .map(|line| line.unwrap().trim().to_owned())
        .map(|line| {
            let mut parts = line.split_whitespace();
            let task_name = parts.next().unwrap();
            let description = parts.next().unwrap_or("");
            task!(task_name, "make", description)
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
    run_command(get_make_command(), &args, verbose)
}

/// Returns the command to run Makefile, either "mmake" or "make"
fn get_make_command() -> &'static str {
    if which("mmake").is_ok() {
        "mmake"
    } else {
        "make"
    }
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
    fn test_list_tasks_by_mmake() {
        if let Ok(tasks) = list_tasks_by_mmake() {
            println!("{:?}", tasks);
        }
    }

    #[test]
    fn test_run() {
        if let Ok(output) = run_task("hello", &[], &[], true) {
            let status_code = output.status.code().unwrap_or(0);
            println!("exit code: {}", status_code);
        }
    }
}
