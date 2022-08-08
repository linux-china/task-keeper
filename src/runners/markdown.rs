use std::io::{BufRead, BufReader};
use std::process::{Output};
use error_stack::{IntoReport, Result, ResultExt};
use crate::errors::KeeperError;
use crate::models::Task;
use crate::command_utils::{run_command_line};
use crate::task;

pub fn is_available() -> bool {
    std::env::current_dir()
        .map(|dir| dir.join("README.md").exists())
        .unwrap_or(false)
}

pub fn list_tasks() -> Result<Vec<Task>, KeeperError> {
    let readme_md = std::env::current_dir()
        .map(|dir| dir.join("README.md"))
        .map(|path| std::fs::read_to_string(path).unwrap())
        .report()
        .change_context(KeeperError::InvalidProcfile)?;
    let mut tasks: Vec<Task> = vec![];
    let mut offset = readme_md.find("```shell");
    let mut shell_counter = 0;
    while offset.is_some() {
        shell_counter += 1;
        let mut offset_num = offset.unwrap() + 8;
        let end = readme_md[offset_num..].find("```").map(|x| x + offset_num);
        if end.is_none() {
            break;
        }
        let end_num = end.unwrap();
        let code = readme_md.get(offset_num..end_num).unwrap().trim();
        if !code.is_empty() {
            tasks.push(parse_task_from_code_block(shell_counter, code));
        }
        offset_num = end_num + 3;
        offset = readme_md[offset_num..].find("```shell").map(|x| x + offset_num);
    }
    Ok(tasks)
}

fn parse_task_from_code_block(counter: i32, code_block: &str) -> Task {
    let lines = BufReader::new(code_block.as_bytes())
        .lines()
        .filter(|line| line.is_ok() && !line.as_ref().unwrap().is_empty())
        .map(|line| line.unwrap())
        .map(|line| {
            if line.starts_with("$") {
                line[1..].trim().to_string()
            } else {
                line.trim().to_string()
            }
        })
        .collect::<Vec<String>>();
    let first_line = lines.get(0).unwrap();
    let mut command_lines: Vec<String> = vec![];
    let mut line_escape = false;
    lines.iter()
        .filter(|line| !line.starts_with("#"))
        .for_each(|line| {
            let mut temp_line = line.as_str();
            if line.ends_with("\\") {
                temp_line = line[..line.len() - 1].as_ref();
            }
            if line_escape {
                command_lines.last_mut().unwrap().push_str(temp_line);
            } else {
                command_lines.push(temp_line.to_string());
            }
            line_escape = line.ends_with("\\");
        });
    let mut name = format!("md-{}", counter);
    let description = command_lines.join("\n");
    if first_line.starts_with("##") {
        name = str::replace(first_line[2..].trim(), " ", "-");
    }
    task!(name, "markdown", description)
}

pub fn run_task(task: &str, _extra_args: &[&str], verbose: bool) -> Result<Output, KeeperError> {
    let tasks = list_tasks()?;
    let task = tasks.iter().find(|t| t.name == task).ok_or_else(|| {
        KeeperError::TaskNotFound(task.to_string())
    })?;
    BufReader::new(task.description.as_bytes())
        .lines()
        .map(|line| {
            run_command_line(&line.unwrap(), verbose)
        })
        .last()
        .unwrap()
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
        if let Ok(output) = run_task("http-methods", &[], true) {
            let status_code = output.status.code().unwrap_or(0);
            println!("exit code: {}", status_code);
        }
    }
}