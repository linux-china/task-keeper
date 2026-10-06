use crate::command_utils::{CommandOutput, run_command, run_command_line, split_command_line};
use crate::errors::KeeperError;
use error_stack::{IntoReport, Report, ResultExt};
use std::collections::HashMap;
use which::which;

pub fn is_available() -> bool {
    std::env::current_dir()
        .map(|dir| {
            dir.join("build.gradle").exists()
                || dir.join("build.gradle.kts").exists()
                || dir.join("settings.gradle").exists()
                || dir.join("settings.gradle.kts").exists()
        })
        .unwrap_or(false)
}

pub fn is_command_available() -> bool {
    which("./gradlew").is_ok() || which("gradle").is_ok()
}

pub fn get_task_command_map() -> HashMap<String, String> {
    let mut task_command_map = HashMap::new();
    let gradle_command = get_gradle_command();
    task_command_map.insert(
        "install".to_string(),
        format!(
            "{} --refresh-dependencies classes dependencies",
            gradle_command
        ),
    );
    task_command_map.insert(
        "compile".to_string(),
        format!("{} classes testClasses", gradle_command),
    );
    task_command_map.insert("build".to_string(), format!("{} assemble", gradle_command));
    if let Some(start_command) = get_start_command_line() {
        task_command_map.insert("start".to_string(), start_command);
    }
    task_command_map.insert("test".to_string(), format!("{} test", gradle_command));
    task_command_map.insert(
        "deps".to_string(),
        format!("{} dependencies", gradle_command),
    );
    task_command_map.insert("doc".to_string(), format!("{} javadoc", gradle_command));
    task_command_map.insert("clean".to_string(), format!("{} clean", gradle_command));
    task_command_map.insert(
        "update".to_string(),
        format!("{} dependencyUpdates", gradle_command),
    );
    task_command_map.insert(
        "outdated".to_string(),
        format!("{} dependencyUpdates", gradle_command),
    );
    task_command_map.insert(
        "sbom".to_string(),
        format!("{} cyclonedxDirectBom", gradle_command),
    );
    task_command_map.insert(
        "skills".to_string(),
        format!("{} extractSkillsJars", gradle_command),
    );
    if let Ok(code) = std::fs::read_to_string("gradle/wrapper/gradle-wrapper.properties") {
        if !code.contains("gradle-9.8.0") {
            task_command_map.insert(
                "self-update".to_string(),
                format!("{} wrapper --gradle-version=9.8.0", gradle_command),
            );
        }
    }
    task_command_map
}

pub fn run_task(
    task: &str,
    task_args: &[&str],
    _global_args: &[&str],
    verbose: bool,
) -> Result<CommandOutput, Report<KeeperError>> {
    if let Some(command_line) = get_task_command_map().get(task) {
        if task == "update" || task == "outdated" {
            let script = include_bytes!("./gradle_scripts/init-versions.gradle");
            let extra_args: Vec<String> = task_args.iter().map(|arg| arg.to_string()).collect();
            run_with_init_script(command_line, "init-versions.gradle", script, &extra_args, verbose)
        } else if task == "sbom" {
            let script = include_bytes!("./gradle_scripts/init-cyclonedx.gradle");
            let extra_args: Vec<String> = task_args.iter().map(|arg| arg.to_string()).collect();
            run_with_init_script(command_line, "init-cyclonedx.gradle", script, &extra_args, verbose)
        } else if task == "skills" {
            let script = include_bytes!("./gradle_scripts/init-skillsjars.gradle");
            run_with_init_script(command_line, "init-skillsjars.gradle", script, &skills_args(task_args), verbose)
        } else {
            run_command_line(command_line, verbose)
        }
    } else {
        Err(KeeperError::ManagerTaskNotFound(task.to_owned(), "gradle".to_string()).into_report())
    }
}

/// Write the init script to temp dir, and run gradle with `--init-script <path>` as separate arguments,
/// so that paths with spaces and quoted task arguments are passed to gradle unchanged.
fn run_with_init_script(
    command_line: &str,
    script_name: &str,
    script: &[u8],
    extra_args: &[String],
    verbose: bool,
) -> Result<CommandOutput, Report<KeeperError>> {
    let temp_file = std::env::temp_dir().join(script_name);
    std::fs::write(&temp_file, script).change_context(KeeperError::FailedToRunTasks(format!(
        "failed to write {}",
        temp_file.display()
    )))?;
    let mut args = split_command_line(command_line)
        .filter(|parts| !parts.is_empty())
        .ok_or_else(|| {
            KeeperError::FailedToRunTasks(format!("invalid command line: '{}'", command_line))
                .into_report()
        })?;
    let program = args.remove(0);
    args.push("--init-script".to_owned());
    args.push(temp_file.to_string_lossy().to_string());
    args.extend(extra_args.iter().cloned());
    let args: Vec<&str> = args.iter().map(String::as_str).collect();
    run_command(&program, &args, verbose)
}

/// Arguments for `extractSkillsJars`: `-Ddir=`, `--dir=` and `-dir=` are converted to the project property `-Pdir=`,
/// and the default dir is `.agents/skills`.
fn skills_args(task_args: &[&str]) -> Vec<String> {
    if task_args.is_empty() {
        return vec!["-Pdir=.agents/skills".to_owned()];
    }
    task_args
        .iter()
        .map(|arg| {
            ["-Ddir=", "--dir=", "-dir="]
                .iter()
                .find_map(|prefix| arg.strip_prefix(prefix))
                .map(|dir| format!("-Pdir={}", dir))
                .unwrap_or_else(|| arg.to_string())
        })
        .collect()
}

pub fn get_gradle_command() -> &'static str {
    if cfg!(windows) {
        let wrapper_available = std::env::current_dir()
            .map(|dir| dir.join("gradlew.bat").exists())
            .unwrap_or(false);
        if wrapper_available {
            ".\\gradlew.bat"
        } else {
            "gradle.bat"
        }
    } else {
        let wrapper_available = std::env::current_dir()
            .map(|dir| dir.join("gradlew").exists())
            .unwrap_or(false);
        if wrapper_available {
            "./gradlew"
        } else {
            "gradle"
        }
    }
}

pub fn get_gradle_build_file() -> &'static str {
    let gradle_with_kotlin = std::env::current_dir()
        .map(|dir| dir.join("build.gradle.kts").exists())
        .unwrap_or(false);
    if gradle_with_kotlin {
        "build.gradle.kts"
    } else {
        "build.gradle"
    }
}

fn get_start_command_line() -> Option<String> {
    let build_gradle_file = get_gradle_build_file();
    if std::env::current_dir()
        .map(|dir| dir.join(build_gradle_file).exists())
        .unwrap_or(false)
    {
        let gradle_build_code = std::env::current_dir()
            .map(|dir| dir.join(build_gradle_file))
            .and_then(std::fs::read_to_string)
            .unwrap_or("".to_owned());
        if (build_gradle_file == "build.gradle.kts"
            && gradle_build_code.contains(r#"id("org.springframework.boot")"#))
            || (build_gradle_file == "build.gradle"
                && gradle_build_code.contains(r#"id 'org.springframework.boot'"#))
        {
            return Some(format!("{} bootRun", get_gradle_command()));
        } else if (build_gradle_file == "build.gradle.kts"
            && gradle_build_code.contains(r#"id("io.quarkus")"#))
            || (build_gradle_file == "build.gradle"
                && gradle_build_code.contains(r#"id 'io.quarkus'"#))
        {
            return Some(format!(
                "{} --console=plain quarkusDev",
                get_gradle_command()
            ));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skills_args() {
        assert_eq!(skills_args(&[]), vec!["-Pdir=.agents/skills"]);
        assert_eq!(skills_args(&["-Ddir=a b"]), vec!["-Pdir=a b"]);
        assert_eq!(skills_args(&["--dir=x"]), vec!["-Pdir=x"]);
        assert_eq!(skills_args(&["-dir=x", "--info"]), vec!["-Pdir=x", "--info"]);
        assert_eq!(skills_args(&["-Pdir=x"]), vec!["-Pdir=x"]);
    }

    #[cfg(unix)]
    #[test]
    fn test_run_with_init_script_keeps_arguments() {
        // `sh -c 'script' tk <args>` checks that each argument arrives unchanged
        let check = r#"test "$1" = --init-script && test -f "$2" && test "$3" = "a b""#;
        let command_line = format!("sh -c '{}' tk", check);
        let output = run_with_init_script(
            &command_line,
            "tk test init.gradle",
            b"// test",
            &["a b".to_owned()],
            false,
        )
        .unwrap();
        assert!(output.status.success());
    }
}
