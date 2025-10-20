use error_stack::Report;
use crate::command_utils::{run_command, CommandOutput};
use crate::errors::KeeperError;
use crate::models::Task;
use crate::task;
use serde::Deserialize;
use which::which;

#[derive(Deserialize, Debug, Default)]
struct Project {
    #[serde(rename = "@name")]
    pub name: Option<String>,
    #[serde(rename = "target")]
    pub targets: Option<Vec<Target>>,
}

#[derive(Deserialize, Debug, Default)]
struct Target {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@description")]
    pub description: Option<String>,
}

pub fn is_available() -> bool {
    std::env::current_dir()
        .map(|dir| dir.join("build.xml").exists())
        .unwrap_or(false)
}

pub fn is_command_available() -> bool {
    which("ant").is_ok()
}

pub fn list_tasks() -> Result<Vec<Task>, Report<KeeperError>> {
    Ok(parse_build_xml()
        .targets
        .map(|targets| {
            targets
                .into_iter()
                .map(|target| {
                    task!(
                        &target.name.clone(),
                        "ant",
                        &target.description.clone().unwrap_or("".to_owned())
                    )
                })
                .collect()
        })
        .unwrap_or_else(|| vec![]))
}

fn parse_build_xml() -> Project {
    std::env::current_dir()
        .map(|dir| dir.join("build.xml"))
        .map(|path| std::fs::read_to_string(path).unwrap())
        .map(|data| serde_xml_rs::from_str(&data).unwrap())
        .unwrap()
}

pub fn run_task(
    task: &str,
    task_args: &[&str],
    _global_args: &[&str],
    verbose: bool,
) -> Result<CommandOutput, Report<KeeperError>> {
    let mut args = vec![];
    //args.extend(global_args);
    args.push(task);
    args.extend(task_args);
    run_command("ant", &args, verbose)
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
        if let Ok(output) = run_task("compile", &[], &[], true) {
            let status_code = output.status.code().unwrap_or(0);
            println!("exit code: {}", status_code);
        }
    }
}
