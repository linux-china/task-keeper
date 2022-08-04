use std::path::Path;
use std::process::{Command, Output, Stdio};

pub fn run_tasks(runner: &str, task_names: &[&str], extra_args: &[&str], verbose: bool) -> anyhow::Result<Output> {
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
        .output()?;
    Ok(output)
}
