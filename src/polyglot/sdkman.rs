use std::env;
use std::fs::File;
use std::io::BufReader;
use crate::polyglot::PATH_SEPARATOR;
use colored::Colorize;


pub fn is_available() -> bool {
    env::current_dir()
        .map(|dir| dir.join(".sdkmanrc").exists())
        .unwrap_or(false)
}

pub fn init_env() {
    let mut sdkmanrc_file = File::open(".sdkmanrc").unwrap();
    let sdkman_map = java_properties::read(BufReader::new(sdkmanrc_file)).unwrap();
    let candidates_home = dirs::home_dir().unwrap().join(".sdkman").join("candidates");
    //iterate over the sdkman_map
    for (key, value) in sdkman_map.iter() {
        let candidate_home_name = format!("{}_HOME", key.to_uppercase());
        let candidate_home_path = candidates_home.join(key).join(value);
        if candidate_home_path.exists() {
            // set candidate home env variable
            env::set_var(&candidate_home_name, &candidate_home_path.to_string_lossy().to_string());
            // Add candidate bin path to PATH env variable on first position
            let candidate_bin_path = candidates_home.join(key).join(value).join("bin");
            let bin_path = if candidate_bin_path.exists() {
                candidate_bin_path.to_string_lossy().to_string()
            } else {
                candidate_home_path.to_string_lossy().to_string()
            };
            if let Ok(path) = env::var("PATH") {
                env::set_var("PATH", format!("{}{}{}", bin_path, PATH_SEPARATOR, path));
            }
        }
    }
}

pub fn diagnose() -> i32 {
    let mut sdkmanrc_file = File::open(".sdkmanrc").unwrap();
    let sdkman_map = java_properties::read(BufReader::new(sdkmanrc_file)).unwrap();
    let candidates_home = dirs::home_dir().unwrap().join(".sdkman").join("candidates");
    let mut problems_count = 0;
    for (key, value) in sdkman_map.iter() {
        let candidate_home_path = candidates_home.join(key).join(value);
        if !candidate_home_path.exists() {
            problems_count += 1;
            println!("{} {} found in .sdkmanrc, but not installed, please use `sdk install {} {}` to install it.",
                     "Warning:".bold().yellow(),
                     key, key, value);
        }
    }
    return problems_count;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_env() {
        init_env();
        println!("JAVA_HOME: {}", env::var("JAVA_HOME").expect("env: JAVA_HOME"));
        println!("JBANG_HOME: {}", env::var("JBANG_HOME").expect("env: JBANG_HOME"));
        println!("PATH: {}", env::var("PATH").expect("env: PATH"));
    }
}
