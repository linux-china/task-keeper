use crate::errors::KeeperError;
use colored::Colorize;
use error_stack::{IntoReport, Report, ResultExt};
use std::collections::HashMap;
use std::ffi::OsString;
use std::io;
use std::io::{Read, Write};
use std::process::{Command, ExitStatus, Output, Stdio};
use which::which;

pub struct CommandOutput {
    pub status: ExitStatus,
    pub stdout: Option<String>,
    pub stderr: Option<String>,
}

impl CommandOutput {
    pub fn from(output: Output) -> Self {
        CommandOutput {
            status: output.status,
            stdout: if output.stdout.len() == 0 {
                None
            } else {
                String::from_utf8(output.stdout).ok()
            },
            stderr: if output.stderr.len() == 0 {
                None
            } else {
                String::from_utf8(output.stderr).ok()
            },
        }
    }

    /// Turn a non-zero exit status into `KeeperError::TaskFailed` carrying the exit code.
    pub fn ensure_success(&self, task_name: &str) -> Result<(), Report<KeeperError>> {
        if self.status.success() {
            Ok(())
        } else {
            Err(KeeperError::TaskFailed(task_name.to_string(), exit_code(&self.status)).into_report())
        }
    }
}

/// Exit code of a finished process; on Unix, a process killed by a signal maps to 128 + signal.
pub fn exit_code(status: &ExitStatus) -> i32 {
    if let Some(code) = status.code() {
        return code;
    }
    #[cfg(unix)]
    {
        use std::os::unix::process::ExitStatusExt;
        if let Some(signal) = status.signal() {
            return 128 + signal;
        }
    }
    1
}

pub fn is_command_available(command_name: &str) -> bool {
    which(command_name).is_ok()
}

/// Program to spawn for `command_name`.
/// On Windows, `Command::new` only appends `.exe` when searching PATH, so `.cmd`/`.bat` wrappers,
/// such as npm, pnpm, yarn and composer, are not found; `which` honors `PATHEXT` and gives the full path.
/// Since Rust 1.77, arguments for `.cmd`/`.bat` are escaped safely by std (CVE-2024-24576).
pub fn resolve_program(command_name: &str) -> OsString {
    if cfg!(target_os = "windows") {
        if let Ok(path) = which(command_name) {
            return path.into_os_string();
        }
    }
    OsString::from(command_name)
}

/// Split a command line into command name and arguments.
/// On Windows, cmd.exe/PowerShell command lines follow different quoting rules
/// than POSIX shells (`shlex` targets), so a dedicated Windows-style splitter
/// is used there; on other platforms, `shlex::split` is used as before.
pub fn split_command_line(command_line: &str) -> Option<Vec<String>> {
    if cfg!(target_os = "windows") {
        Some(windows_args::Args::parse_cmd(command_line).collect::<Vec<String>>())
    } else {
        shlex::split(command_line)
    }
}

pub fn run_command(
    command_name: &str,
    args: &[&str],
    verbose: bool,
) -> Result<CommandOutput, Report<KeeperError>> {
    run_command_with_env_vars(command_name, args, &None, &None, verbose)
}

pub fn run_command_line(command_line: &str, verbose: bool) -> Result<CommandOutput, Report<KeeperError>> {
    let command_and_args = split_command_line(command_line)
        .filter(|parts| !parts.is_empty())
        .ok_or_else(|| {
            KeeperError::FailedToRunTasks(format!("invalid command line: '{}'", command_line))
                .into_report()
        })?;
    // command line contains shell syntax, such as pipe, redirection, `&&` or `$VAR`
    if needs_shell(command_line) || starts_with_env_assignment(&command_and_args[0]) {
        return run_command_by_shell(command_line, verbose);
    }
    let command_name = &command_and_args[0];
    let args: Vec<&str> = command_and_args[1..].iter().map(AsRef::as_ref).collect();
    if is_command_available(&command_name) {
        run_command(&command_name, &args, verbose)
    } else {
        println!(
            "{}",
            format!(
                "{} is not available to run '{}'",
                command_name, command_line
            )
            .bold()
            .red()
        );
        Err(KeeperError::CommandNotFound(command_name.to_string()).into_report())
    }
}

pub fn run_command_line_from_stdin(
    command_line: &str,
    input: &str,
    verbose: bool,
) -> Result<CommandOutput, Report<KeeperError>> {
    let command_and_args = split_command_line(command_line).unwrap();
    let command_name = &command_and_args[0];
    let args: Vec<&str> = if command_and_args.len() > 1 {
        command_and_args[1..].iter().map(AsRef::as_ref).collect()
    } else {
        vec![]
    };
    if verbose {
        println!("[tk] command line:  {:?}", command_line);
    }
    if is_command_available(&command_name) {
        let mut child = Command::new(resolve_program(command_name))
            .args(&args)
            .envs(std::env::vars())
            .stdin(Stdio::piped())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .change_context(KeeperError::FailedToRunTasks(format!("{:?}", command_name)))?;
        child
            .stdin
            .as_mut()
            .ok_or("Child process stdin has not been captured!")
            .unwrap()
            .write_all(input.as_bytes())
            .unwrap();
        child
            .wait_with_output()
            .map(CommandOutput::from)
            .change_context(KeeperError::FailedToRunTasks(format!("{:?}", command_name)))
    } else {
        println!(
            "{}",
            format!(
                "{} is not available to run '{}'",
                command_name, command_line
            )
            .bold()
            .red()
        );
        Err(KeeperError::CommandNotFound(command_name.to_string()).into_report())
    }
}

pub fn run_command_with_env_vars(
    command_name: &str,
    args: &[&str],
    working_dir: &Option<String>,
    env_vars: &Option<HashMap<String, String>>,
    verbose: bool,
) -> Result<CommandOutput, Report<KeeperError>> {
    // child process inherits environment variables from tk, and no `envs(std::env::vars())`
    // to avoid printing all environment variables(including secrets) in verbose mode
    let mut command = Command::new(resolve_program(command_name));
    if args.len() > 0 {
        command.args(args);
    }
    if let Some(current_dir) = working_dir {
        command.current_dir(current_dir);
    }
    if let Some(vars) = env_vars {
        for (key, value) in vars {
            command.env(key, value);
        }
    }
    if verbose {
        println!("[tk] command line:  {:?}", command);
    }
    if std::env::var("TK_TASK_ID").is_ok() {
        return intercept_output(&mut command);
    }
    command
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .map(CommandOutput::from)
        .change_context(KeeperError::FailedToRunTasks(format!("{:?}", command)))
}

/// Run command line by POSIX `sh -c` (`cmd /C` on Windows), never by `$SHELL`,
/// because fish/nushell/xonsh are not compatible with POSIX syntax.
pub fn run_command_by_shell(
    command_line: &str,
    verbose: bool,
) -> Result<CommandOutput, Report<KeeperError>> {
    if cfg!(target_os = "windows") {
        run_command_with_env_vars("cmd", &["/C", command_line], &None, &None, verbose)
    } else {
        run_command_with_env_vars("sh", &["-c", command_line], &None, &None, verbose)
    }
}

/// Check whether command line contains shell syntax outside of quotes:
/// pipe, redirection, `&&`, `||`, `;`, `&`, subshell, `$VAR`, `$(...)`, backtick, glob or `~`.
fn needs_shell(command_line: &str) -> bool {
    let windows = cfg!(target_os = "windows");
    let mut in_single_quote = false;
    let mut in_double_quote = false;
    let mut chars = command_line.chars();
    while let Some(c) = chars.next() {
        if in_single_quote {
            if c == '\'' {
                in_single_quote = false;
            }
            continue;
        }
        match c {
            // escaped character is literal, except on Windows where `\` is a path separator
            '\\' if !windows => {
                chars.next();
            }
            '\'' if !in_double_quote => in_single_quote = true,
            '"' => in_double_quote = !in_double_quote,
            // expansion still works inside double quotes
            '$' | '`' => return true,
            '%' if windows => return true,
            '|' | '&' | ';' | '<' | '>' | '(' | ')' | '*' | '?' | '[' | '~' | '\n'
                if !in_double_quote =>
            {
                return true
            }
            '^' if windows && !in_double_quote => return true,
            _ => {}
        }
    }
    false
}

/// Check `FOO=bar` style environment variable assignment before the command name
fn starts_with_env_assignment(first_arg: &str) -> bool {
    match first_arg.split_once('=') {
        Some((name, _)) => {
            !name.is_empty()
                && !name.starts_with(|c: char| c.is_ascii_digit())
                && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
        }
        None => false,
    }
}

pub fn intercept_output(command: &mut Command) -> Result<CommandOutput, Report<KeeperError>> {
    let mut child = command
        .stdin(Stdio::inherit())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .change_context(KeeperError::FailedToRunTasks(format!("{:?}", command)))?;
    // Create threads to handle both streams
    let mut stdout = child.stdout.take().unwrap();
    let mut stderr = child.stderr.take().unwrap();

    let stdout_thread = std::thread::spawn(move || {
        let mut stdout_bytes = Vec::new();
        let mut buffer = [0; 8192];
        while let Ok(n) = stdout.read(&mut buffer) {
            if n == 0 {
                break;
            }
            // Print to console
            let content = &buffer[..n];
            io::stdout().write_all(content).unwrap();
            // Collect output
            stdout_bytes.extend_from_slice(content);
        }
        String::from_utf8_lossy(&stdout_bytes).to_string()
    });

    let stderr_thread = std::thread::spawn(move || {
        let mut stderr_bytes = Vec::new();
        let mut buffer = [0; 32];
        while let Ok(n) = stderr.read(&mut buffer) {
            if n == 0 {
                break;
            }
            // Print to console
            let content = &buffer[..n];
            io::stderr().write_all(content).unwrap();
            // You can also process the error output here
            stderr_bytes.extend_from_slice(content);
        }
        String::from_utf8_lossy(&stderr_bytes).to_string()
    });

    let output = stdout_thread.join().unwrap();
    let error = stderr_thread.join().unwrap();

    let status = child.wait().unwrap();
    Ok(CommandOutput {
        status,
        stdout: if output.is_empty() {
            None
        } else {
            Some(output)
        },
        stderr: if error.is_empty() { None } else { Some(error) },
    })
}

pub fn capture_command_output(command_name: &str, args: &[&str]) -> Result<Output, Report<KeeperError>> {
    let mut command = Command::new(resolve_program(command_name));
    if args.len() > 0 {
        command.args(args);
    }
    let output = command
        .envs(std::env::vars())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .change_context(KeeperError::FailedToRunTasks(format!("{:?}", command)))?;
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_needs_shell() {
        for line in [
            "ls -al | wc -l",
            "ls -al|wc -l",
            "cargo build && cargo test",
            "make clean; make",
            "echo hi > out.txt",
            "cmd 2>&1",
            "echo $HOME",
            "echo \"$HOME\"",
            "echo $(date)",
            "echo `date`",
            "rm -rf build/*",
            "ls ~/bin",
            "(cd src && ls)",
            "sleep 1 &",
        ] {
            assert!(needs_shell(line), "{} should need shell", line);
        }
        for line in [
            "cargo build --release",
            "npm run build",
            "echo 'a | b && c'",
            "echo \"a | b; c > d\"",
            "echo 'single $HOME'",
            "echo a\\|b",
            "mvn -DskipTests package",
        ] {
            assert!(!needs_shell(line), "{} should not need shell", line);
        }
    }

    #[test]
    fn test_resolve_program() {
        if cfg!(target_os = "windows") {
            // `cmd` is resolved by PATHEXT to the full path of cmd.exe
            let program = resolve_program("cmd").to_string_lossy().to_lowercase();
            assert!(program.ends_with("cmd.exe"), "{}", program);
        } else {
            assert_eq!(resolve_program("sh"), OsString::from("sh"));
        }
        assert_eq!(resolve_program("tk-no-such-command"), OsString::from("tk-no-such-command"));
    }

    #[test]
    fn test_starts_with_env_assignment() {
        assert!(starts_with_env_assignment("RUST_LOG=debug"));
        assert!(starts_with_env_assignment("FOO="));
        assert!(!starts_with_env_assignment("cargo"));
        assert!(!starts_with_env_assignment("--name=value"));
        assert!(!starts_with_env_assignment("1FOO=bar"));
        assert!(!starts_with_env_assignment("=bar"));
    }

    #[test]
    fn test_run_shell_syntax() {
        let output = run_command_line("true && false", false).unwrap();
        assert!(!output.status.success());
        let output = run_command_line("FOO=bar sh -c 'test \"$FOO\" = bar'", false).unwrap();
        assert!(output.status.success());
        let output = run_command_line("test \"$(echo ok)\" = ok", false).unwrap();
        assert!(output.status.success());
    }

    #[test]
    fn test_run_pipe_line() {
        run_command_line("ls -al | wc -l", true).unwrap();
    }

    #[test]
    fn test_run_command_line_from_stdin() {
        run_command_line_from_stdin("deno run -", "console.log('hello world')", true).unwrap();
    }

    #[test]
    fn test_function_alias() {
        let command_name = "lss";
        if let Ok(path) = which(command_name) {
            println!("{:?}", path);
        } else {
            println!("{} is a function", command_name);
        }
    }

    #[test]
    fn test_intercept_output() {
        let mut command = Command::new("java");
        command.args(["-version"]).envs(std::env::vars());
        let output = intercept_output(&mut command).unwrap();
        println!("{}", output.stderr.unwrap())
    }
}
