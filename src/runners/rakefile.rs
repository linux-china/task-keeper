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
        .map(|dir| dir.join("Rakefile").exists() || dir.join("rakefile").exists())
        .unwrap_or(false)
}

pub fn is_command_available() -> bool {
    which("rake").is_ok()
}

pub fn list_tasks() -> Result<Vec<Task>, Report<KeeperError>> {
    let rake_output = capture_command_output("rake", &["-AT"])
        .map(|output| String::from_utf8(output.stdout).unwrap_or("{}".to_owned()))?;
    let re = Regex::new(r"^[a-zA-Z0-9][a-zA-Z0-9._-].*").unwrap();
    let tasks: Vec<Task> = BufReader::new(rake_output.as_bytes())
        .lines()
        .filter(|line| line.is_ok() && re.is_match(line.as_ref().unwrap()))
        .map(|line| line.unwrap())
        .map(|line| {
            let mut parts = line.split('#');
            let mut task_name = parts.next().unwrap().trim().to_owned();
            if task_name.starts_with("rake ") {
                task_name = task_name.split_off(5);
            }
            let description = parts.next().unwrap_or("").trim().to_owned();
            task!(task_name, "rake", description)
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
    run_command("rake", &args, verbose)
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
        if let Ok(output) = run_task("doit", &[], &[], true) {
            let status_code = output.status.code().unwrap_or(0);
            println!("exit code: {}", status_code);
        }
    }
}
