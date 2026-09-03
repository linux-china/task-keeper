use crate::command_utils::{run_command_line, run_command_line_from_stdin, CommandOutput};
use crate::errors::KeeperError;
use crate::models::Task;
use crate::task;
use error_stack::{Report, ResultExt};
use logos::Logos;
use std::collections::HashMap;
use std::env::temp_dir;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufRead, BufReader};
use uuid::Uuid;

pub fn is_available() -> bool {
    std::env::current_dir()
        .map(|dir| dir.join("README.md").exists())
        .unwrap_or(false)
}

pub fn list_tasks() -> Result<Vec<Task>, Report<KeeperError>> {
    let readme_md = std::env::current_dir()
        .map(|dir| dir.join("README.md"))
        .map(|path| std::fs::read_to_string(path).unwrap())
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
        let line_break_offset = readme_md[offset_num..]
            .find('\n')
            .map(|x| x + offset_num)
            .unwrap();
        let language_and_attributes: &str =
            readme_md.get(offset_num..line_break_offset).unwrap().trim();
        if language_and_attributes.contains('{')
            && language_and_attributes.ends_with('}')
            && language_and_attributes.contains('#')
        {
            // format as {#name first=second} {#name}
            let language = language_and_attributes.split('{').next().unwrap().trim();
            let markdown_attributes =
                language_and_attributes[language_and_attributes.find('{').unwrap()..].trim();
            let attributes = parse_markdown_attributes(markdown_attributes);
            if attributes.contains_key("id") {
                let name = attributes.get("id").unwrap();
                let code_runner = attributes.get("class").cloned().unwrap_or("".to_string());
                let description = attributes.get("desc").cloned().unwrap_or("".to_string());
                let code = readme_md
                    .get((line_break_offset + 1)..end_num)
                    .unwrap()
                    .trim();
                if !code.is_empty() {
                    if language == "javascript" || language == "typescript" {
                        let runner2 = if !code_runner.is_empty() {
                            code_runner.split(' ').next().unwrap().to_owned()
                        } else {
                            // make bun as default JS/TS engine
                            if which::which("bun").is_ok() {
                                "bun".to_owned()
                            } else {
                                "node".to_owned()
                            }
                        };
                        tasks.push(parse_task_from_code_block(
                            &name,
                            code,
                            &runner2,
                            &description,
                        ));
                    } else if language == "shell" {
                        tasks.push(parse_task_from_code_block(&name, code, "sh", &description));
                    } else if language == "java" || language == "jshelllanguage" {
                        tasks.push(parse_task_from_code_block(
                            &name,
                            code,
                            "java",
                            &description,
                        ));
                    } else if language == "kotlin" {
                        tasks.push(parse_task_from_code_block(&name, code, "kt", &description));
                    } else if language == "groovy" {
                        tasks.push(parse_task_from_code_block(
                            &name,
                            code,
                            "groovy",
                            &description,
                        ));
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

fn parse_task_from_code_block(
    task_name: &str,
    code_block: &str,
    runner2: &str,
    description: &str,
) -> Task {
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
    lines
        .iter()
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
    let code_block = command_lines.join("\n");
    task!(task_name, "markdown", runner2, task_desc, Some(code_block))
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
    let runner2 = task.runner2.clone().unwrap_or("sh".to_owned());
    let code_block = task.code_block.clone().unwrap_or("".to_string());
    if runner2 == "node" {
        run_command_line_from_stdin("node -", &code_block, verbose)
    } else if runner2 == "deno" {
        run_command_line_from_stdin("deno run -", &code_block, verbose)
    } else if runner2 == "java" {
        run_command_line_from_stdin("jbang run -", &code_block, verbose)
    } else if runner2 == "groovy" || runner2 == "kt" {
        let file_name = format!("{}.{}", Uuid::new_v4(), runner2);
        let file_path = temp_dir().join(&file_name);
        let mut file = File::create(file_path.as_path()).unwrap();
        file.write_all(code_block.as_bytes()).unwrap();
        file.sync_all().unwrap();
        let command_line = format!("jbang run {}", file_path.to_str().unwrap());
        run_command_line(&command_line, verbose)
    } else {
        BufReader::new(code_block.as_bytes())
            .lines()
            .map(|line| run_command_line(&line.unwrap(), verbose))
            .last()
            .unwrap()
    }
}

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")] // Ignore this regex pattern between tokens
enum MarkdownAttribute<'a> {
    // Tokens can be literal strings, of any length.
    #[token("{")]
    LBRACE,
    // Tokens can be literal strings, of any length.
    #[token("}")]
    RBRACE,
    #[regex(r"#([a-zA-Z0-9_-]+)")]
    ID(&'a str),
    #[regex(r"\.([a-zA-Z0-9]+)")]
    CLASS(&'a str),
    #[regex(r"([a-zA-Z0-9]+)")]
    BooleanKey(&'a str),
    #[regex(r#"([a-zA-Z0-9-_:@.]+)=([^\s"]+)"#)]
    KV(&'a str),
    #[regex(r#"([a-zA-Z0-9-_:@.]+)="([^"]+)""#)]
    KV2(&'a str),
}

fn parse_markdown_attributes(markdown_attributes: &str) -> HashMap<String, String> {
    let mut attributes = HashMap::new();
    let mut classes = vec![];
    let lex = MarkdownAttribute::lexer(markdown_attributes);
    for token in lex.into_iter() {
        if let Ok(attribute) = token {
            // match for Attribute
            match attribute {
                MarkdownAttribute::ID(id) => {
                    attributes.insert("id".to_string(), id[1..].to_string());
                }
                MarkdownAttribute::CLASS(class) => {
                    classes.push(class[1..].to_string());
                }
                MarkdownAttribute::BooleanKey(key) => {
                    attributes.insert(key.to_string(), "true".to_string());
                }
                MarkdownAttribute::KV(kv) => {
                    let offset = kv.find('=').unwrap();
                    attributes.insert(kv[..offset].to_string(), kv[offset + 1..].to_string());
                }
                MarkdownAttribute::KV2(kv2) => {
                    let offset = kv2.find('=').unwrap();
                    let value = kv2[offset + 2..kv2.len() - 1].to_string();
                    attributes.insert(kv2[..offset].to_string(), value);
                }
                _ => {}
            }
        }
    }
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
        let text = r#"{#hello .node .js defer key1=value1 key2="good morning" x-on:click="count++" @click="open = ! open"  @click.outside="open = false"}"#;
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
        if let Ok(output) = run_task("myip", &[], &[], true) {
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
