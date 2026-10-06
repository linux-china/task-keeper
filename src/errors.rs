use thiserror::Error as ThisError;

#[allow(dead_code)]
#[derive(ThisError, Debug)]
pub enum KeeperError {
    #[error("TK-001404: command not found: {0}")]
    CommandNotFound(String),

    #[error("TK-100404: task not found: {0}")]
    TaskNotFound(String),

    #[error("TK-100501: {0} not found for {1}!")]
    ManagerTaskNotFound(String, String),

    #[error("TK-100500: failed to run tasks: {0}")]
    FailedToRunTasks(String),

    #[error("TK-100502: task {0} failed with exit code {1}")]
    TaskFailed(String, i32),

    #[error("TK-201001: failed to parse Makefile: {0}")]
    InvalidMakefile(String),

    #[error("TK-202001: failed to parse package.json")]
    InvalidPackageJson,

    #[error("TK-203001: failed to parse deno.json")]
    InvalidDenoJson,

    #[error("TK-204001: failed to parse fleet.json: {0}")]
    InvalidFleetRunJson(String),

    #[error("TK-206001: failed to parse Makefile.toml")]
    InvalidMakefileToml,

    #[error("TK-206002: failed to parse Justfile")]
    InvalidJustfile,

    #[error("TK-207001: failed to read composer.json")]
    InvalidComposerJson,

    #[error("TK-208001: failed to read Procfile")]
    InvalidProcfile,

    #[error("TK-209001: failed to read jbang-catalog.json")]
    InvalidJBangCatalogJson,

    #[error("TK-210001: failed to read Taskfile.ts")]
    InvalidTaskFileTs,

    #[error("TK-211001: failed to read Argcfile.sh")]
    InvalidArgcFile,

    #[error("TK-212001: failed to parse build.xml")]
    InvalidBuildXml,

    #[error("TK-213001: failed to read README.md")]
    InvalidReadmeMd,

    #[error("TK-300001: failed to read pom.xml")]
    InvalidPomXml,
    
    #[error("TK-300002: failed to read maven-metadata.xml")]
    InvalidMavenMetadataXml,

    #[error("TK-302001: failed to list uv tasks")]
    InvalidUvTasks,

    #[error("TK-303001: failed to read queries.sql")]
    InvalidQueriesSQL,

    #[error("TK-304001: failed to parse tasks.py")]
    InvalidTasksPy,
}
