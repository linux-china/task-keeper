use std::collections::HashMap;
use std::process::{Command, Output, Stdio};
use error_stack::{IntoReport, Result, ResultExt};
use crate::errors::KeeperError;

pub const RUNNERS: &'static [&'static str] = &["make", "npm", "deno", "just"];

pub fn run_tasks(runner: &str, task_names: &[&str], extra_args: &[&str], verbose: bool) -> Result<Output, KeeperError> {
    let mut command = Command::new(runner);
    if extra_args.len() > 0 {
        command.args(extra_args);
    }
    command.args(task_names);
    if verbose {
        println!("[tk] command line:  {:?}", command);
    }
    let output = command
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .report()
        .change_context(KeeperError::FailedToRunTasks(format!("{:?}", command)))?;
    Ok(output)
}

pub fn list_tasks() -> Result<HashMap<String, Vec<String>>, KeeperError> {
    let mut tasks = HashMap::new();
    tasks.insert("just".to_string(), vec!["hello".to_string(), "hello2".to_string()]);
    tasks.insert("make".to_string(), vec!["clean".to_string(), "compile".to_string()]);
    Ok(tasks)
}
