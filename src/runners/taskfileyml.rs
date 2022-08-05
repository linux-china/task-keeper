use std::io::{BufRead, BufReader};
use std::process::Output;
use crate::errors::KeeperError;
use crate::models::Task;
use crate::runners::{run_command, capture_command_output};
use crate::task;
use error_stack::{Result};
use regex::Regex;


pub fn is_available() -> bool {
    std::env::current_dir()
        .map(|dir| dir.join("Taskfile.yml").exists() || dir.join("Taskfile.yaml").exists())
        .unwrap_or(false)
}


pub fn list_tasks() -> Result<Vec<Task>, KeeperError> {
    let makefile_meta_text = capture_command_output("go-task", &["--list-all"])
        .map(|output| {
            String::from_utf8(output.stdout).unwrap_or("{}".to_owned())
        })?;
    let re = Regex::new(r"^\* [a-zA-Z0-9._-]+:.*").unwrap();
    let tasks: Vec<Task> = BufReader::new(makefile_meta_text.as_bytes())
        .lines()
        .filter(|line| {
            line.is_ok() && re.is_match(line.as_ref().unwrap())
        })
        .map(|line| line.unwrap())
        .map(|line| {
            let mut parts = line.split(':');
            let mut task_name = parts.next().unwrap().trim().to_owned();
            if task_name.starts_with("* ") {
                task_name = task_name.split_off(2);
            }
            let description = parts.next().unwrap_or("").trim().to_owned();
            task!(task_name, "task", description)
        }).collect();
    Ok(tasks)
}

pub fn run_task(task: &str, extra_args: &[&str], verbose: bool) -> Result<Output, KeeperError> {
    let mut args = vec![];
    args.extend(extra_args);
    args.push(task);
    run_command("go-task", &args, verbose)
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
        if let Ok(output) = run_task("hello", &[], true) {
            let status_code = output.status.code().unwrap_or(0);
            println!("exit code: {}", status_code);
        }
    }
}