use crate::command_utils::{
    is_command_available, run_command, run_command_line_from_stdin, run_command_with_env_vars, CommandOutput,
};
use crate::errors::KeeperError;
use crate::models::Task;
use crate::task;
use error_stack::{Report, ResultExt};
use logos::Logos;
use std::collections::HashMap;
use std::io::prelude::*;
use std::io::{BufRead, BufReader};
use std::process::ExitStatus;

pub fn is_available() -> bool {
    std::env::current_dir()
        .map(|dir| dir.join("README.md").exists())
        .unwrap_or(false)
}

pub fn list_tasks() -> Result<Vec<Task>, Report<KeeperError>> {
    let readme_md = std::env::current_dir()
        .map(|dir| dir.join("README.md"))
        .and_then(std::fs::read_to_string)
        .change_context(KeeperError::InvalidReadmeMd)?;
    let mut tasks: Vec<Task> = vec![];
    for (info, code) in find_fenced_code_blocks(&readme_md) {
        // format as {#name first=second} {#name}
        let Some(brace) = info.find('{') else { continue };
        let markdown_attributes = info[brace..].trim();
        if !(markdown_attributes.ends_with('}') && markdown_attributes.contains('#')) {
            continue;
        }
        let language = info[..brace].trim();
        let attributes = parse_markdown_attributes(markdown_attributes);
        let Some(name) = attributes.get("id") else { continue };
        let code_runner = attributes.get("class").cloned().unwrap_or("".to_string());
        let description = attributes.get("desc").cloned().unwrap_or("".to_string());
        let code = code.trim();
        if code.is_empty() {
            continue;
        }
        let runner2 = match language {
            "javascript" | "typescript" => {
                if !code_runner.is_empty() {
                    code_runner.split(' ').next().unwrap().to_owned()
                } else if which::which("bun").is_ok() {
                    // make bun as default JS/TS engine
                    "bun".to_owned()
                } else {
                    "node".to_owned()
                }
            }
            "shell" | "sh" => "sh".to_owned(),
            "java" | "jshelllanguage" => "java".to_owned(),
            "kotlin" => "kt".to_owned(),
            "groovy" => "groovy".to_owned(),
            _ => continue,
        };
        tasks.push(parse_task_from_code_block(name, code, &runner2, &description));
    }
    Ok(tasks)
}

/// Fenced code blocks of the markdown text in document order, as (info string, code).
/// Lines are scanned so that a block is matched by its whole language, i.e. `java` never matches `javascript`.
fn find_fenced_code_blocks(text: &str) -> Vec<(&str, String)> {
    let mut blocks = vec![];
    let mut lines = text.lines();
    while let Some(line) = lines.next() {
        let trimmed = line.trim_start();
        let fence_char = match trimmed.chars().next() {
            Some(c @ ('`' | '~')) => c,
            _ => continue,
        };
        let fence_len = trimmed.chars().take_while(|c| *c == fence_char).count();
        if fence_len < 3 {
            continue;
        }
        let info = trimmed[fence_len..].trim();
        // a backtick fence can't have backticks in its info string, so it's inline code
        if fence_char == '`' && info.contains('`') {
            continue;
        }
        let mut code = String::new();
        for line in lines.by_ref() {
            let trimmed = line.trim();
            let closing_len = trimmed.chars().take_while(|c| *c == fence_char).count();
            if closing_len >= fence_len && closing_len == trimmed.len() {
                break;
            }
            code.push_str(line);
            code.push('\n');
        }
        // an unclosed block runs to the end of the document, as in CommonMark
        blocks.push((info, code));
    }
    blocks
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
        .filter(|line| !line.is_empty())
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
    } else if runner2 == "bun" {
        run_command_line_from_stdin("bun run -", &code_block, verbose)
    } else if runner2 == "java" {
        run_command_line_from_stdin("jbang run -", &code_block, verbose)
    } else if runner2 == "groovy" || runner2 == "kt" {
        // jbang picks the language by the file extension; the file is removed when it's dropped
        let mut file = tempfile::Builder::new()
            .prefix("tk_")
            .suffix(&format!(".{}", runner2))
            .tempfile()
            .change_context(KeeperError::FailedToRunTasks(task.name.clone()))?;
        file.write_all(code_block.as_bytes())
            .and_then(|_| file.flush())
            .change_context(KeeperError::FailedToRunTasks(task.name.clone()))?;
        let script = file.path().to_string_lossy().to_string();
        run_command("jbang", &["run", &script], verbose)
    } else if code_block.trim().is_empty() {
        // empty code block, such as only comments: nothing to run
        Ok(CommandOutput {
            status: ExitStatus::default(),
            stdout: None,
            stderr: None,
        })
    } else {
        run_shell_code_block(&code_block, verbose)
    }
}

/// Run the whole block by one shell, so that `cd`, `export` and variables carry over to the next lines,
/// and `-e` stops at the first failed line with its exit code.
/// `-c` rather than the block on stdin, which would leave `read` or `cat` in the block reading the rest of the block.
fn run_shell_code_block(code_block: &str, verbose: bool) -> Result<CommandOutput, Report<KeeperError>> {
    if cfg!(target_os = "windows") && !is_command_available("sh") {
        // no POSIX shell, e.g. without Git for Windows: cmd stops at the first failed line too
        let command_line = code_block.lines().collect::<Vec<&str>>().join(" && ");
        return run_command_with_env_vars("cmd", &["/C", &command_line], &None, &None, verbose);
    }
    run_command_with_env_vars("sh", &["-e", "-c", code_block], &None, &None, verbose)
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
    use crate::command_utils::run_command_line;

    #[test]
    fn test_parse_markdown_attributes() {
        let text = r#"{#hello .node .js defer key1=value1 key2="good morning" x-on:click="count++" @click="open = ! open"  @click.outside="open = false"}"#;
        let attributes = parse_markdown_attributes(text);
        println!("{:?}", attributes);
    }

    #[test]
    fn test_parse_empty_code_block() {
        let task = parse_task_from_code_block("demo", "# only comment\n$\n", "sh", "");
        assert_eq!(task.code_block.as_deref(), Some(""));
        let task = parse_task_from_code_block("demo", "$\n$ echo hi\n", "sh", "");
        assert_eq!(task.code_block.as_deref(), Some("echo hi"));
    }

    #[test]
    fn test_find_fenced_code_blocks_in_order() {
        let text = "```javascript {#js}\nconsole.log(1)\n```\n\n~~~java {#java}\nSystem.out.println(1);\n~~~\n\n````shell {#sh}\necho '```'\n````\n";
        let blocks = find_fenced_code_blocks(text);
        let infos: Vec<&str> = blocks.iter().map(|(info, _)| *info).collect();
        assert_eq!(infos, vec!["javascript {#js}", "java {#java}", "shell {#sh}"]);
        assert_eq!(blocks[2].1, "echo '```'\n");
    }

    #[test]
    #[cfg(unix)]
    fn test_run_shell_code_block() {
        // state carries over between lines
        let output = run_shell_code_block("cd /\nX=1\ntest \"$(pwd)\" = / && test \"$X\" = 1", false).unwrap();
        assert!(output.status.success());
        // stops at the first failed line with its exit code
        let output = run_shell_code_block("false\necho should-not-run", false).unwrap();
        assert!(!output.status.success());
    }

    #[test]
    fn test_run_command_line_invalid() {
        assert!(run_command_line("", false).is_err());
        assert!(run_command_line("echo \"unclosed", false).is_err());
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
