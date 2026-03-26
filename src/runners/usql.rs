use crate::command_utils::{run_command, CommandOutput};
use crate::errors::KeeperError;
use crate::models::Task;
use crate::task;
use error_stack::{Report, ResultExt};
use regex::Regex;
use std::collections::HashSet;
use std::env;
use std::io::BufRead;
use std::ops::Index;
use which::which;

pub fn is_available() -> bool {
    env::current_dir()
        .map(|dir| dir.join("queries.sql").exists())
        .unwrap_or(false)
}

pub fn is_command_available() -> bool {
    which("usql").is_ok()
}

pub fn list_tasks() -> Result<Vec<Task>, Report<KeeperError>> {
    let queries_sql_text = env::current_dir()
        .map(|dir| dir.join("queries.sql"))
        .map(|path| std::fs::read_to_string(path).unwrap())
        .change_context(KeeperError::InvalidQueriesSQL)?;
    let mut tasks: Vec<Task> = vec![];
    let mut sentence_lines: Vec<&str> = vec![];
    queries_sql_text.lines().for_each(|line| {
        if line.starts_with("--") {
            if line.starts_with("-- QUERY") {
                if !sentence_lines.is_empty() {
                    tasks.push(create_task(&sentence_lines));
                }
                sentence_lines.clear();
                sentence_lines.push(line);
            }
        } else {
            if !line.is_empty() {
                sentence_lines.push(line);
            }
        }
    });
    if !sentence_lines.is_empty() {
        tasks.push(create_task(&sentence_lines));
    }
    Ok(tasks)
}

fn get_dsn_url() -> Option<String> {
    if let Ok(url) = env::var("DSN_URL") {
        Some(url)
    } else {
        env::current_dir()
            .map(|dir| dir.join("queries.sql"))
            .map(|path| std::fs::read_to_string(path).unwrap())
            .unwrap()
            .lines()
            .find(|line| line.starts_with("-- DSN_URL="))
            .map(|line| line[line.find('=').unwrap() + 1..].trim().to_string())
    }
}

fn create_task(sentence_lines: &[&str]) -> Task {
    let task_name = sentence_lines[0]
        .split_whitespace()
        .nth(2)
        .unwrap()
        .to_string();
    let task_sql = sentence_lines[1..]
        .join("\n")
        .trim_start_matches("-- ")
        .to_string();
    task!(task_name, "usql", "", "", Some(task_sql))
}

/// 提取 SQL 中的 `:xx` 形式的参数名称
fn extract_sql_parameters(sql: &str) -> Vec<String> {
    let re = Regex::new(r":([a-zA-Z_][a-zA-Z0-9_]*)").unwrap();
    let mut seen = HashSet::new();
    let mut params = Vec::new();
    for cap in re.captures_iter(sql) {
        let param_name = cap.get(1).unwrap().as_str().to_string();
        if seen.insert(param_name.clone()) {
            params.push(param_name);
        }
    }
    params
}

/// 使用给定的参数值替换 SQL 中的占位符
fn replace_sql_with_values(sql: &str, params: &[String], values: &[String]) -> String {
    let mut result = sql.to_string();
    for (param, value) in params.iter().zip(values.iter()) {
        let placeholder = format!(":{}", param);
        let replacement = format!("'{}'", value.replace("'", "''"));
        result = result.replace(&placeholder, &replacement);
    }
    result
}

/// 使用自定义输入提供者替换 SQL 中的占位符
/// `input_provider` 接收参数名，返回对应的值
fn replace_sql_parameters_with_provider<F>(sql: &str, input_provider: F) -> String
where
    F: Fn(&str) -> String,
{
    let params = extract_sql_parameters(sql);
    if params.is_empty() {
        return sql.to_string();
    }
    let values: Vec<String> = params.iter().map(|param| input_provider(param)).collect();
    replace_sql_with_values(sql, &params, &values)
}

/// 提示用户输入参数值并替换 SQL 中的占位符
fn replace_sql_parameters(sql: &str) -> String {
    replace_sql_parameters_with_provider(sql, |param| {
        let prompt = format!("Please input value for '{}': ", param);
        rprompt::prompt_reply(&prompt).unwrap_or_default()
    })
}

pub fn run_task(
    task: &str,
    task_args: &[&str],
    global_args: &[&str],
    verbose: bool,
) -> Result<CommandOutput, Report<KeeperError>> {
    let dsn_url = get_dsn_url();
    if dsn_url.is_none() {
        return Err(Report::new(KeeperError::TaskNotFound(task.to_string())));
    }
    let dns_url = dsn_url.unwrap();
    let tasks = list_tasks()?;
    let task = tasks
        .iter()
        .find(|t| t.name == task)
        .ok_or_else(|| KeeperError::TaskNotFound(task.to_string()))?;
    let sql = task.code_block.clone().unwrap();
    // 提取 SQL 中的 `:xx` 形式的所有参数，提示用户输入，然后进行替换
    let sql = replace_sql_parameters(&sql);
    let mut args = vec![];
    args.extend(global_args);
    args.extend(task_args);
    args.push("-c");
    args.push(&sql);
    args.push(&dns_url);
    run_command("usql", &args, verbose)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command() {
        let args =
            shlex::split("usql -c \"select DATETIME('now')\" sqlite3://demo.sqlite3").unwrap();
        println!("{:?}", args);
    }

    #[test]
    fn test_parse() {
        if let Ok(tasks) = list_tasks() {
            println!("{:?}", tasks);
        }
    }
    #[test]
    fn test_dsn_url() {
        if let Some(dsn_url) = get_dsn_url() {
            println!("DSN_URL: {}", dsn_url);
        }
    }

    #[test]
    fn test_run() {
        if let Ok(output) = run_task("get_all_users", &[], &[], false) {
            let status_code = output.status.code().unwrap_or(0);
            println!("exit code: {}", status_code);
        }
    }

    #[test]
    fn test_extract_sql_parameters() {
        // 测试基本参数提取
        let sql = "SELECT * FROM users WHERE name = :name AND age > :age";
        let params = extract_sql_parameters(sql);
        assert_eq!(params, vec!["name", "age"]);

        // 测试无参数的 SQL
        let sql_no_params = "SELECT * FROM users";
        let params_empty = extract_sql_parameters(sql_no_params);
        assert!(params_empty.is_empty());

        // 测试重复参数（应该去重）
        let sql_dup = "SELECT * FROM users WHERE name = :name OR nickname = :name";
        let params_dup = extract_sql_parameters(sql_dup);
        assert_eq!(params_dup, vec!["name"]);

        // 测试下划线参数名
        let sql_underscore = "SELECT * FROM users WHERE user_id = :user_id";
        let params_underscore = extract_sql_parameters(sql_underscore);
        assert_eq!(params_underscore, vec!["user_id"]);
    }

    #[test]
    fn test_replace_sql_with_values() {
        // 测试基本替换
        let sql = "SELECT * FROM users WHERE name = :name AND age > :age";
        let params = vec!["name".to_string(), "age".to_string()];
        let values = vec!["Alice".to_string(), "25".to_string()];
        let result = replace_sql_with_values(sql, &params, &values);
        assert_eq!(
            result,
            "SELECT * FROM users WHERE name = 'Alice' AND age > '25'"
        );

        // 测试单引号转义
        let sql_quote = "SELECT * FROM users WHERE name = :name";
        let params_quote = vec!["name".to_string()];
        let values_quote = vec!["O'Brien".to_string()];
        let result_quote = replace_sql_with_values(sql_quote, &params_quote, &values_quote);
        assert_eq!(result_quote, "SELECT * FROM users WHERE name = 'O''Brien'");

        // 测试无参数的 SQL
        let sql_no_params = "SELECT * FROM users";
        let result_no_params = replace_sql_with_values(sql_no_params, &[], &[]);
        assert_eq!(result_no_params, "SELECT * FROM users");

        // 测试重复参数的替换
        let sql_dup = "SELECT * FROM users WHERE name = :name OR nickname = :name";
        let params_dup = vec!["name".to_string()];
        let values_dup = vec!["test".to_string()];
        let result_dup = replace_sql_with_values(sql_dup, &params_dup, &values_dup);
        assert_eq!(
            result_dup,
            "SELECT * FROM users WHERE name = 'test' OR nickname = 'test'"
        );
    }

    #[test]
    fn test_replace_sql_parameters_with_provider() {
        // 测试基本替换 - 使用模拟输入
        let sql = "SELECT * FROM users WHERE name = :name AND age > :age";
        let mock_inputs = std::collections::HashMap::from([
            ("name", "Alice"),
            ("age", "25"),
        ]);
        let sql_replaced = replace_sql_parameters_with_provider(sql, |param| {
            mock_inputs.get(param).unwrap_or(&"").to_string()
        });
        assert_eq!(
            sql_replaced,
            "SELECT * FROM users WHERE name = 'Alice' AND age > '25'"
        );
    
        // 测试无参数的 SQL
        let sql_no_params = "SELECT * FROM users";
        let result_no_params = replace_sql_parameters_with_provider(sql_no_params, |_| String::new());
        assert_eq!(result_no_params, "SELECT * FROM users");
    
        // 测试单引号转义
        let sql_quote = "SELECT * FROM users WHERE name = :name";
        let result_quote = replace_sql_parameters_with_provider(sql_quote, |_| "O'Brien".to_string());
        assert_eq!(result_quote, "SELECT * FROM users WHERE name = 'O''Brien'");
    }
}
