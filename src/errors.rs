use thiserror::Error as ThisError;

#[allow(dead_code)]
#[derive(ThisError, Debug)]
pub enum KeeperError {
    #[error("TK-001404: command not found: {0}")]
    CommandNotFound(String),
    #[error("TK-100404: task not found: {0}")]
    TaskNotFound(String),
    #[error("TK-100500: failed to run tasks: {0}")]
    FailedToRunTasks(String),
    #[error("TK-201001: failed to parse Makefile: {0}")]
    InvalidMakefile(String),
    #[error("TK-202001: failed to parse package.json")]
    InvalidPackageJson,
    #[error("TK-203001: failed to parse package.json: {0}")]
    InvalidDenoJson(String),
    #[error("TK-204001: failed to parse package.json: {0}")]
    InvalidFleetRunJson(String),
    #[error("TK-205001: failed to parse Justfile: {0}")]
    InvalidCodeLaunchJson(String),
    #[error("TK-206001: failed to parse Justfile")]
    InvalidMakefileToml,
    #[error("TK-206001: failed to parse Makefile.toml")]
    InvalidJustfile,
    #[error("TK-207001: failed to read Procfile")]
    InvalidComposerJson,
    #[error("TK-208001: failed to read composer.json")]
    InvalidProcfile,
    #[error("TK-300001: failed to read pom.xml")]
    InvalidPomXml,
}
