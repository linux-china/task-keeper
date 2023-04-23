use std::io::{BufRead, BufReader};
use std::process::{Output};
use std::env::temp_dir;
use std::fs::File;
use std::io::prelude::*;
use error_stack::{IntoReport, Result, ResultExt};
use uuid::Uuid;
use crate::errors::KeeperError;
use crate::models::Task;
use crate::command_utils::{run_command_line, run_command_line_from_stdin};
use crate::task;
use std::collections::HashMap;
use regex::Regex;

pub fn is_available() -> bool {
    std::env::current_dir()
        .map(|dir| dir.join("README.md").exists())
        .unwrap_or(false)
}

pub fn list_tasks() -> Result<Vec<Task>, KeeperError> {
    let readme_md = std::env::current_dir()
        .map(|dir| dir.join("README.md"))
        .map(|path| std::fs::read_to_string(path).unwrap())
        .into_report()
        .change_context(KeeperError::InvalidProcfile)?;
    let mut tasks: Vec<Task> = vec![];
    let mut offset = find_shell_code_offset(&readme_md);
    while offset.is_some() {
        let mut offset_num = offset.unwrap() + 3;
        let end = readme_md[offset_num..].find("```").map(|x| x + offset_num);
        if end.is_none() {
            break;
        }
        let end_num = end.unwrap();
        let line_break_offset = readme_md[offset_num..].find('\n').map(|x| x + offset_num).unwrap();
        let language_and_attributes: &str = readme_md.get(offset_num..line_break_offset).unwrap().trim();
        if language_and_attributes.contains('{') && language_and_attributes.ends_with('}') && language_and_attributes.contains('#') {
            // format as {#name first=second} {#name}
            let language = language_and_attributes.split('{').next().unwrap().trim();
            let markdown_attributes = language_and_attributes[language_and_attributes.find('{').unwrap()..].trim();
            let attributes = parse_markdown_attributes(markdown_attributes);
            if attributes.contains_key("id") {
                let name = attributes.get("id").unwrap();
                let code_runner = attributes.get("class").cloned().unwrap_or("".to_string());
                let description = attributes.get("desc").cloned().unwrap_or("".to_string());
                let code = readme_md.get((line_break_offset + 1)..end_num).unwrap().trim();
                if !code.is_empty() {
                    if language == "javascript" || language == "typescript" {
                        let runner2 = if code_runner.is_empty() {
                            code_runner.split(' ').next().unwrap().to_owned()
                        } else {
                            "node".to_owned()
                        };
                        tasks.push(parse_task_from_code_block(&name, code, &runner2, &description));
                    } else if language == "shell" {
                        tasks.push(parse_task_from_code_block(&name, code, "sh", &description));
                    } else if language == "java" || language == "jshelllanguage" {
                        tasks.push(parse_task_from_code_block(&name, code, "java", &description));
                    } else if language == "kotlin" {
                        tasks.push(parse_task_from_code_block(&name, code, "kt", &description));
                    } else if language == "groovy" {
                        tasks.push(parse_task_from_code_block(&name, code, "groovy", &description));
                    }
                }
            }
        }
        offset_num = end_num + 3;
        offset = find_shell_code_offset(&readme_md[offset_num..]).map(|x| x + offset_num);
    }
    Ok(tasks)
}

fn find_shell_code_offset(text: &str) -> Option<usize> {
    let mut offset = text.find("```shell");
    if offset.is_none() {
        offset = text.find("```sh");
    }
    if offset.is_none() {
        offset = text.find("```javascript");
    }
    if offset.is_none() {
        offset = text.find("```typescript");
    }
    if offset.is_none() {
        offset = text.find("```java");
    }
    if offset.is_none() {
        offset = text.find("```kotlin");
    }
    if offset.is_none() {
        offset = text.find("```groovy");
    }
    if offset.is_none() {
        offset = text.find("```jshelllanguage");
    }

    offset
}

fn parse_task_from_code_block(task_name: &str, code_block: &str, runner2: &str, description: &str) -> Task {
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
    let task_desc = if description.is_empty() {
        command_lines.join("\n")
    } else {
        description.to_string()
    };
    task!(task_name, "markdown", runner2, task_desc)
}

pub fn run_task(task: &str, _task_args: &[&str], _global_args: &[&str], verbose: bool) -> Result<Output, KeeperError> {
    let tasks = list_tasks()?;
    let task = tasks.iter().find(|t| t.name == task).ok_or_else(|| {
        KeeperError::TaskNotFound(task.to_string())
    })?;
    let runner2 = task.runner2.clone().unwrap_or("sh".to_owned());
    if runner2 == "node" {
        run_command_line_from_stdin("node -", &task.description, verbose)
    } else if runner2 == "deno" {
        run_command_line_from_stdin("deno run -", &task.description, verbose)
    } else if runner2 == "java" {
        run_command_line_from_stdin("jbang run -", &task.description, verbose)
    } else if runner2 == "groovy" || runner2 == "kt" {
        let file_name = format!("{}.{}", Uuid::new_v4(), runner2);
        let file_path = temp_dir().join(&file_name);
        let mut file = File::create(file_path.as_path()).unwrap();
        file.write_all(task.description.as_bytes()).unwrap();
        file.sync_all().unwrap();
        let command_line = format!("jbang run {}", file_path.to_str().unwrap());
        run_command_line(&command_line, verbose)
    } else {
        BufReader::new(task.description.as_bytes())
            .lines()
            .map(|line| {
                run_command_line(&line.unwrap(), verbose)
            })
            .last()
            .unwrap()
    }
}

fn parse_markdown_attributes(markdown_attributes: &str) -> HashMap<String, String> {
    let mut attributes = HashMap::new();
    let mut classes = vec![];
    let pairs_text = if markdown_attributes.starts_with('{') {
        &markdown_attributes[1..markdown_attributes.len() - 1]
    } else {
        markdown_attributes
    };
    let re_id = Regex::new(r"#([a-zA-Z0-9-_]+)\b").unwrap();
    let re_class = Regex::new(r"\.([a-zA-Z0-9-_:.]+)\b").unwrap();
    let re_pair1 = Regex::new(r#"([a-zA-Z0-9-_:@.]+)=([^\s"]+)"#).unwrap();
    let re_pair2 = Regex::new(r#"([a-zA-Z0-9-_:@.]+)="([^"]+)""#).unwrap();
    re_id.find(pairs_text).map(|m| {
        attributes.insert("id".to_string(), m.as_str()[1..].to_string());
    });
    re_class.find_iter(pairs_text).into_iter().for_each(|m| {
        classes.push(m.as_str()[1..].to_string());
    });
    re_pair1.find_iter(pairs_text).into_iter().for_each(|m| {
        let pair = m.as_str();
        let offset = pair.find('=').unwrap();
        attributes.insert(pair[..offset].to_string(), pair[offset + 1..].to_string());
    });
    re_pair2.find_iter(pairs_text).into_iter().for_each(|m| {
        let pair = m.as_str();
        let offset = pair.find('=').unwrap();
        let value = pair[offset + 2..pair.len() - 1].to_string();
        attributes.insert(pair[..offset].to_string(), value);
    });
    if classes.len() > 0 {
        attributes.insert("class".to_string(), classes.join(" "));
    }
    return attributes;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_markdown_attributes() {
        let text = r#"{#hello .node .js key1=value1 key2="good morning" x-on:click="count++" @click="open = ! open"  @click.outside="open = false"}"#;
        let attributes = parse_markdown_attributes(text);
        println!("{:?}", attributes);
    }

    #[test]
    fn test_parse() {
        if let Ok(tasks) = list_tasks() {
            println!("{:?}", tasks);
        }
    }

    #[test]
    fn test_run_js() {
        if let Ok(output) = run_task("js2", &[], &[], true) {
            let status_code = output.status.code().unwrap_or(0);
            println!("exit code: {}", status_code);
        }
    }

    #[test]
    fn test_run() {
        if let Ok(output) = run_task("http-methods", &[], &[], true) {
            let status_code = output.status.code().unwrap_or(0);
            println!("exit code: {}", status_code);
        }
    }
}
