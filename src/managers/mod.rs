pub mod maven;

pub const MANAGERS: &'static [&'static str] = &["maven", "gradle", "sbt", "npm", "cargo", "cmake", "composer", "bundle", "cmake", "go"];

pub fn get_manager_file_name(runner: &str) -> &'static str {
    match runner {
        "maven" => "pom.xml",
        "gradle" => "build.gradle[.kts]",
        "sbt" => "build.sbt",
        "npm" => "package.json",
        "cargo" => "Cargo.toml",
        "cmake" => "CMakeLists.txt",
        "composer" => "composer.json",
        "go" => "go.mod",
        "swift" => "Package.swift",
        _ => "unknown",
    }
}
