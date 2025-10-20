use crate::command_utils::{capture_command_output, run_command, CommandOutput};
use crate::errors::KeeperError;
use crate::models::Task;
use crate::task;
use error_stack::{Report, ResultExt};
use serde::{Deserialize, Serialize};
use which::which;

const ARGC_SCRIPT_NAMES: [&str; 6] = [
    "Argcfile.sh",
    "Argcfile",
    "argcfile.sh",
    "argcfile",
    "ARGCFILE.sh",
    "ARGCFILE",
];

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArgcfileJson {
    pub subcommands: Vec<ArgcSubCommand>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArgcSubCommand {
    pub name: String,
    pub describe: Option<String>,
}

pub fn is_available() -> bool {
    std::env::current_dir()
        .map(|dir| ARGC_SCRIPT_NAMES.iter().any(|name| dir.join(name).exists()))
        .unwrap_or(false)
}

pub fn is_command_available() -> bool {
    which("argc").is_ok()
}

pub fn list_tasks() -> Result<Vec<Task>, Report<KeeperError>> {
    let current_dir = std::env::current_dir().unwrap();
    let argc_file = ARGC_SCRIPT_NAMES
        .iter()
        .map(|name| current_dir.join(name))
        .find(|path| path.exists())
        .map(|path| path.to_str().unwrap_or("").to_owned())
        .unwrap_or("".to_owned());
    let json_text = capture_command_output("argc", &["--argc-export", &argc_file])
        .map(|output| String::from_utf8(output.stdout).unwrap_or("{}".to_owned()))?;
    serde_json::from_str::<ArgcfileJson>(&json_text)
        .map(|argc_file_json| {
            let mut tasks: Vec<Task> = vec![];
            // sub tasks
            let sub_tasks: Vec<Task> = argc_file_json
                .subcommands
                .iter()
                .map(|sub_command| {
                    task!(
                        sub_command.name.clone(),
                        "argc",
                        &sub_command.describe.clone().unwrap_or("".to_owned())
                    )
                })
                .collect();
            tasks.extend(sub_tasks);
            tasks
        })
        .change_context(KeeperError::InvalidArgcFile)
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
    run_command("argc", &args, verbose)
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
        if let Ok(output) = run_task("build1", &[], &[], true) {
            let status_code = output.status.code().unwrap_or(0);
            println!("exit code: {}", status_code);
        }
    }
}
