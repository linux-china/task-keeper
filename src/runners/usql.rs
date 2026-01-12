use crate::command_utils::{run_command, CommandOutput};
use crate::errors::KeeperError;
use crate::models::Task;
use crate::task;
use error_stack::{Report, ResultExt};
use std::env;
use std::io::BufRead;
use std::ops::Index;
use which::which;

pub fn is_available() -> bool {
    env::current_dir()
        .map(|dir| dir.join("queries.sql").exists())
        .unwrap_or(false)
}

pub fn is_command_available() -> bool {
    which("usql").is_ok()
}

pub fn list_tasks() -> Result<Vec<Task>, Report<KeeperError>> {
    let queries_sql_text = env::current_dir()
        .map(|dir| dir.join("queries.sql"))
        .map(|path| std::fs::read_to_string(path).unwrap())
        .change_context(KeeperError::InvalidQueriesSQL)?;
    let mut tasks: Vec<Task> = vec![];
    let mut sentence_lines: Vec<&str> = vec![];
    queries_sql_text.lines().for_each(|line| {
        if line.starts_with("--") {
            if line.starts_with("-- QUERY") {
                if !sentence_lines.is_empty() {
                    tasks.push(create_task(&sentence_lines));
                }
                sentence_lines.clear();
                sentence_lines.push(line);
            }
        } else {
            if !line.is_empty() {
                sentence_lines.push(line);
            }
        }
    });
    if !sentence_lines.is_empty() {
        tasks.push(create_task(&sentence_lines));
    }
    Ok(tasks)
}

fn get_dsn_url() -> Option<String> {
    if let Ok(url) = env::var("DSN_URL") {
        Some(url)
    } else {
        env::current_dir()
            .map(|dir| dir.join("queries.sql"))
            .map(|path| std::fs::read_to_string(path).unwrap())
            .unwrap()
            .lines()
            .find(|line| line.starts_with("-- DSN_URL="))
            .map(|line| line[line.find('=').unwrap()+1..].trim().to_string())
    }
}

fn create_task(sentence_lines: &[&str]) -> Task {
    let task_name = sentence_lines[0]
        .split_whitespace()
        .nth(2)
        .unwrap()
        .to_string();
    let task_sql = sentence_lines[1..]
        .join("\n")
        .trim_start_matches("-- ")
        .to_string();
    task!(task_name, "usql", "", "", Some(task_sql))
}

pub fn run_task(
    task: &str,
    task_args: &[&str],
    global_args: &[&str],
    verbose: bool,
) -> Result<CommandOutput, Report<KeeperError>> {
    let dsn_url = get_dsn_url();
    if dsn_url.is_none() {
        return Err(Report::new(KeeperError::TaskNotFound(task.to_string())));
    }
    let dns_url = dsn_url.unwrap();
    let tasks = list_tasks()?;
    let task = tasks
        .iter()
        .find(|t| t.name == task)
        .ok_or_else(|| KeeperError::TaskNotFound(task.to_string()))?;
    let sql = &task.code_block.clone().unwrap();
    let mut args = vec![];
    args.extend(global_args);
    args.extend(task_args);
    args.push("-c");
    args.push(sql);
    args.push(&dns_url);
    run_command("usql", &args, verbose)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command() {
        let args =
            shlex::split("usql -c \"select DATETIME('now')\" sqlite3://demo.sqlite3").unwrap();
        println!("{:?}", args);
    }

    #[test]
    fn test_parse() {
        if let Ok(tasks) = list_tasks() {
            println!("{:?}", tasks);
        }
    }
    #[test]
    fn test_dsn_url() {
        if let Some(dsn_url) = get_dsn_url() {
            println!("DSN_URL: {}", dsn_url);
        }
    }

    #[test]
    fn test_run() {
        if let Ok(output) = run_task("get_all_users", &[], &[], false) {
            let status_code = output.status.code().unwrap_or(0);
            println!("exit code: {}", status_code);
        }
    }
}
