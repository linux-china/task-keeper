use crate::command_utils::{CommandOutput, run_command};
use crate::errors::KeeperError;
use crate::models::Task;
use crate::task;
use error_stack::Report;
use std::env;
use which::which;

const ARGC_SCRIPT_NAMES: [&str; 2] = ["Amberfile", "amberfile"];

pub fn is_available() -> bool {
    env::current_dir()
        .map(|dir| ARGC_SCRIPT_NAMES.iter().any(|name| dir.join(name).exists()))
        .unwrap_or(false)
}

pub fn is_command_available() -> bool {
    which("amber").is_ok()
}

pub fn find_amber_file_name() -> Option<String> {
    let current_dir = env::current_dir().unwrap();
    ARGC_SCRIPT_NAMES
        .iter()
        .map(|name| current_dir.join(name))
        .find(|path| path.exists())
        .map(|path| path.to_str().unwrap_or("").to_owned())
}

pub fn list_tasks() -> Result<Vec<Task>, Report<KeeperError>> {
    if let Some(amberfile) = find_amber_file_name() {
        let amberfile_text =
            std::fs::read_to_string(env::current_dir().unwrap().join(&amberfile)).unwrap();
        let mut tasks: Vec<Task> = vec![];
        for line in amberfile_text.lines() {
            if line.starts_with("pub fun ") {
                // extract function name from `pub fun xxxx() {`
                let end_offset = line.find("(").unwrap();
                let declaration = &line[0..end_offset].trim();
                let offset = declaration.rfind(' ').unwrap();
                if offset > 0 {
                    let task_name = declaration[offset + 1..].trim().to_string();
                    tasks.push(task!(task_name, "amber"))
                }
            }
        }
        Ok(tasks)
    } else {
        Ok(vec![])
    }
}

pub fn run_task(
    task: &str,
    _task_args: &[&str],
    _global_args: &[&str],
    verbose: bool,
) -> Result<CommandOutput, Report<KeeperError>> {
    let amberfile = find_amber_file_name().unwrap();
    let mut args = vec!["eval"];
    let snippet = format!("import {{{}}} from \"{}\"; {}()", task, amberfile, task);
    args.push(&snippet);
    run_command("amber", &args, verbose)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escape() {
        let snippet = format!(
            "import {{{}}} from \"{}\"; {}()",
            "hello", "Amberfile", "hello"
        );
        println!("{}", snippet);
    }
    #[test]
    fn test_parse() {
        if let Ok(tasks) = list_tasks() {
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
