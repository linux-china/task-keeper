use crate::command_utils::{
    is_command_available, run_command, run_command_by_cmd, run_command_line_from_stdin, run_command_with_env_vars,
    CommandOutput,
};
use crate::errors::KeeperError;
use crate::models::Task;
use crate::task;
use error_stack::{Report, ResultExt};
use logos::Logos;
use std::collections::HashMap;
use std::io::prelude::*;
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

/// Remove the shell prompt `$ ` copied from a terminal session, but keep `$VAR` and `${VAR}`,
/// and keep the indentation of other lines, such as the content of a heredoc.
fn strip_prompt(line: &str) -> &str {
    match line.trim_start().strip_prefix('$') {
        Some(rest) if rest.is_empty() || rest.starts_with(char::is_whitespace) => rest.trim(),
        _ => line.trim_end(),
    }
}

/// Remove the indentation shared by all non-blank lines, e.g. a code block nested in a list item,
/// while the relative indentation inside the block is kept.
fn dedent(lines: Vec<&str>) -> Vec<&str> {
    let indent = lines
        .iter()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.len() - line.trim_start().len())
        .min()
        .unwrap_or(0);
    lines
        .into_iter()
        .map(|line| if line.trim().is_empty() { "" } else { &line[indent..] })
        .collect()
}

/// Commands of a shell script without blank lines and comments, and with `\` continuation lines joined,
/// for the task description and the cmd fallback on Windows, which has no heredoc or multi-line syntax.
fn shell_command_lines(script: &str) -> Vec<String> {
    let mut command_lines: Vec<String> = vec![];
    let mut line_escape = false;
    script
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with("#"))
        .for_each(|line| {
            let temp_line = line.strip_suffix('\\').unwrap_or(line);
            if line_escape {
                command_lines.last_mut().unwrap().push_str(temp_line);
            } else {
                command_lines.push(temp_line.to_string());
            }
            line_escape = line.ends_with('\\');
        });
    command_lines
}

fn parse_task_from_code_block(
    task_name: &str,
    code_block: &str,
    runner2: &str,
    description: &str,
) -> Task {
    // the script runs as it's written: blank lines, comments and indentation matter in a heredoc
    let lines = dedent(code_block.lines().map(strip_prompt).collect());
    let script = lines.join("\n").trim_matches('\n').to_string();
    let command_lines = shell_command_lines(&script);
    let task_desc = if description.is_empty() {
        command_lines.join("\n")
    } else {
        description.to_string()
    };
    // only comments or prompts: nothing to run
    let code_block = if command_lines.is_empty() { String::new() } else { script };
    task!(task_name, "markdown", runner2, task_desc, Some(code_block))
}

pub fn run_task(
    task: &str,
    task_args: &[&str],
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
        run_shell_code_block(&code_block, task_args, verbose)
    }
}

/// Run the whole block by one shell, so that `cd`, `export` and variables carry over to the next lines,
/// and `-e` stops at the first failed line with its exit code.
/// `-c` rather than the block on stdin, which would leave `read` or `cat` in the block reading the rest of the block.
/// Task arguments are the positional parameters `$1`, `$2`, `$@` of the block, and `$0` is `tk`.
fn run_shell_code_block(
    code_block: &str,
    task_args: &[&str],
    verbose: bool,
) -> Result<CommandOutput, Report<KeeperError>> {
    if cfg!(target_os = "windows") && !is_command_available("sh") {
        // no POSIX shell, e.g. without Git for Windows: cmd stops at the first failed line too
        let command_line = shell_command_lines(code_block).join(" && ");
        return run_command_by_cmd(&command_line, verbose);
    }
    let mut args = vec!["-e", "-c", code_block, "tk"];
    args.extend(task_args);
    run_command_with_env_vars("sh", &args, &None, &None, verbose)
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
    fn test_strip_prompt() {
        assert_eq!(strip_prompt("$ echo hi"), "echo hi");
        assert_eq!(strip_prompt("$\techo hi"), "echo hi");
        assert_eq!(strip_prompt("  $ echo hi"), "echo hi");
        assert_eq!(strip_prompt("$"), "");
        assert_eq!(strip_prompt("$EDITOR README.md"), "$EDITOR README.md");
        assert_eq!(strip_prompt("${CC:-cc} -o app main.c"), "${CC:-cc} -o app main.c");
        assert_eq!(strip_prompt("$(pwd)/run.sh"), "$(pwd)/run.sh");
        assert_eq!(strip_prompt("echo $HOME"), "echo $HOME");
        assert_eq!(strip_prompt("  indented line  "), "  indented line");
    }

    #[test]
    fn test_parse_heredoc_keeps_indentation() {
        let code = "$ cat <<EOF\n  indented\n\n# not a comment\nEOF\n";
        let task = parse_task_from_code_block("demo", code, "sh", "");
        assert_eq!(
            task.code_block.as_deref(),
            Some("cat <<EOF\n  indented\n\n# not a comment\nEOF")
        );
        // a block nested in a list item is dedented, the relative indentation is kept
        let code = "   if true; then\n     echo hi\n   fi\n";
        let task = parse_task_from_code_block("demo", code, "sh", "");
        assert_eq!(task.code_block.as_deref(), Some("if true; then\n  echo hi\nfi"));
        // continuation lines are joined for the description only
        let task = parse_task_from_code_block("demo", "cargo build \\\n  --release\n", "sh", "");
        assert_eq!(task.description, "cargo build --release");
        assert_eq!(task.code_block.as_deref(), Some("cargo build \\\n  --release"));
    }

    #[test]
    fn test_parse_dollar_variable_lines() {
        let code = "$ export NAME=tk\n$EDITOR README.md\n${CC:-cc} -o app main.c";
        let task = parse_task_from_code_block("demo", code, "sh", "");
        assert_eq!(
            task.code_block.as_deref(),
            Some("export NAME=tk\n$EDITOR README.md\n${CC:-cc} -o app main.c")
        );
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
        let output = run_shell_code_block("cd /\nX=1\ntest \"$(pwd)\" = / && test \"$X\" = 1", &[], false).unwrap();
        assert!(output.status.success());
        // stops at the first failed line with its exit code
        let output = run_shell_code_block("false\necho should-not-run", &[], false).unwrap();
        assert!(!output.status.success());
        // task arguments are the positional parameters
        let output = run_shell_code_block("test \"$1\" = x1 && test \"$#\" = 2", &["x1", "a b"], false).unwrap();
        assert!(output.status.success());
        // heredoc keeps the indentation of its content
        let output = run_shell_code_block("test \"$(cat <<EOF\n  indented\nEOF\n)\" = \"  indented\"", &[], false).unwrap();
        assert!(output.status.success());
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
