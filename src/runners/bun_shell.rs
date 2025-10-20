use crate::command_utils::{run_command_line, CommandOutput};
use crate::errors::KeeperError;
use crate::models::Task;
use crate::task;
use error_stack::{Report, ResultExt};
use std::io::{BufRead, BufReader};
use which::which;

pub fn is_available() -> bool {
    std::env::current_dir()
        .map(|dir| dir.join("Taskfile.ts").exists())
        .unwrap_or(false)
}

pub fn is_command_available() -> bool {
    which("bun").is_ok()
}

pub fn list_tasks() -> Result<Vec<Task>, Report<KeeperError>> {
    let taskfile_text = std::env::current_dir()
        .map(|dir| dir.join("Taskfile.ts"))
        .map(|path| std::fs::read_to_string(path).unwrap())
        .change_context(KeeperError::InvalidTaskFileTs)?;
    let tasks: Vec<Task> = BufReader::new(taskfile_text.as_bytes())
        .lines()
        .filter(|line| {
            if line.is_ok() {
                let trim_line = line.as_ref().unwrap();
                trim_line.starts_with("export async function ")
                    || trim_line.starts_with("export function ")
            } else {
                false
            }
        })
        .map(|line| line.unwrap())
        .map(|line| {
            let offset = line.find(" function ");
            let offset2 = line.find("(");
            let name = line[offset.unwrap() + 10..offset2.unwrap()].trim();
            task!(name, "bun-shell", "")
        })
        .collect();
    Ok(tasks)
}

pub fn run_task(
    task: &str,
    _task_args: &[&str],
    _global_args: &[&str],
    verbose: bool,
) -> Result<CommandOutput, Report<KeeperError>> {
    let tasks = list_tasks()?;
    let task = tasks
        .iter()
        .find(|t| t.name == task)
        .ok_or_else(|| KeeperError::TaskNotFound(task.to_string()))?;
    let mut full_command = r#"bun -e 'if (await Bun.file("Taskfile.ts").exists()) {let module = await import("./Taskfile.ts");if (Bun.argv.length >= 3) {let taskName = Bun.argv[2];taskName in module ? (await module[taskName]()) : console.error(`Task not found: ${taskName}`);} else {console.log("Available tasks:");Object.keys(module).filter(k => typeof module[k] === "function").forEach(k => console.log(" " + k));}} else {console.error("Taskfile.ts not found");}' 0 "#.to_string();
    full_command.push_str(task.name.as_str());
    run_command_line(&full_command, verbose)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_command() {
        let mut full_command = r#"bun -e 'if (await Bun.file("Taskfile.ts").exists()) {let module = await import("./Taskfile.ts");if (Bun.argv.length >= 3) {let taskName = Bun.argv[2];taskName in module ? (await module[taskName]()) : console.error(`Task not found: ${taskName}`);} else {console.log("Available tasks:");Object.keys(module).filter(k => typeof module[k] === "function").forEach(k => console.log(" " + k));}} else {console.error("Taskfile.ts not found");}' 0 "#.to_string();
        full_command.push_str(" hello");
        let args = shlex::split(&full_command).unwrap();
        println!("{:?}", args);
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
