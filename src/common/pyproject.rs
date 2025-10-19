use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PyProjectToml {
    pub tool: Option<Tool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Tool {
    #[serde(rename = "rye")]
    pub uv: Option<ToolUv>,
    pub poetry: Option<ToolPoetry>,
    pub poe: Option<PeoTasks>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct ToolUv {
    pub scripts: Option<HashMap<String, toml::Value>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct PeoTasks {
    pub tasks: Option<HashMap<String, toml::Value>>,
}

impl ToolUv {
    pub fn get_scripts(&self) -> Option<HashMap<String, String>> {
        self.scripts.as_ref().map(|scripts| {
            scripts
                .iter()
                .map(|(key, value)| {
                    let description: String = match value {
                        toml::Value::String(value) => value.to_string(),
                        toml::Value::Table(table) => table
                            .get("cmd")
                            .unwrap_or(&toml::Value::String("".to_owned()))
                            .to_string(),
                        _ => "".to_owned(),
                    };
                    return (key.clone(), description);
                })
                .collect()
        })
    }
}

impl PeoTasks {
    pub fn get_tasks(&self) -> Option<HashMap<String, String>> {
        self.tasks.as_ref().map(|tasks| {
            tasks
                .iter()
                .map(|(key, value)| {
                    let description: String = match value {
                        toml::Value::String(value) => value.to_string(),
                        toml::Value::Table(table) => table
                            .get("help")
                            .unwrap_or(
                                table.get("cmd").unwrap_or(
                                    table
                                        .get("script")
                                        .unwrap_or(&toml::Value::String("".to_owned())),
                                ),
                            )
                            .to_string(),
                        toml::Value::Array(task_sequences) => task_sequences
                            .iter()
                            .map(|task_name| task_name.to_string())
                            .collect::<Vec<String>>()
                            .join(" && "),
                        _ => "".to_owned(),
                    };
                    return (key.clone(), description);
                })
                .collect()
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct ToolPoetry {
    scripts: Option<HashMap<String, toml::Value>>,
}

impl ToolPoetry {
    pub fn get_scripts(&self) -> Option<HashMap<String, String>> {
        self.scripts.as_ref().map(|scripts| {
            scripts
                .iter()
                .map(|(key, value)| {
                    let description: String = match value {
                        toml::Value::String(value) => value.to_string(),
                        toml::Value::Table(table) => table
                            .get("callable")
                            .unwrap_or(&toml::Value::String("".to_owned()))
                            .to_string(),
                        _ => "".to_owned(),
                    };
                    return (key.clone(), description);
                })
                .collect()
        })
    }
}

impl PyProjectToml {
    pub fn get_default_project() -> Result<Self, std::io::Error> {
        if let Ok(pyproject_file) = std::env::current_dir().map(|dir| dir.join("pyproject.toml")) {
            let text = std::fs::read_to_string(pyproject_file).unwrap_or("".to_owned());
            toml::from_str(&text)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
        } else {
            Err(
                std::io::Error::new(std::io::ErrorKind::NotFound, "pyproject.toml not found")
                    .into(),
            )
        }
    }

    pub fn uv_available(&self) -> bool {
        self.tool
            .as_ref()
            .map(|tool| tool.uv.is_some())
            .unwrap_or(false)
    }

    pub fn poetry_available(&self) -> bool {
        self.tool
            .as_ref()
            .map(|tool| tool.poetry.is_some())
            .unwrap_or(false)
    }

    pub fn poe_available(&self) -> bool {
        self.tool
            .as_ref()
            .map(|tool| tool.poe.is_some())
            .unwrap_or(false)
    }

    pub fn get_uv_scripts(&self) -> Option<HashMap<String, String>> {
        self.tool
            .as_ref()
            .and_then(|tool| tool.uv.as_ref())
            .and_then(|uv| uv.get_scripts().clone())
    }

    pub fn get_uv_script(&self, script_name: &str) -> Option<toml::Value> {
        if let Some(tool) = self.tool.as_ref()
            && let Some(uv) = tool.uv.as_ref()
            && let Some(scripts) = uv.scripts.as_ref()
            && let Some(script) = scripts.get(script_name) {
            Some(script.clone())
        } else {
            None
        }
    }

    pub fn get_poetry_scripts(&self) -> Option<HashMap<String, String>> {
        self.tool
            .as_ref()
            .and_then(|tool| tool.poetry.as_ref())
            .and_then(|poetry| poetry.get_scripts().clone())
    }

    pub fn get_poe_tasks(&self) -> Option<HashMap<String, String>> {
        self.tool
            .as_ref()
            .and_then(|tool| tool.poe.as_ref())
            .and_then(|peo_tasks| peo_tasks.get_tasks().clone())
    }

    pub fn venv_bin_path(&self) -> PathBuf {
        std::env::current_dir().unwrap().join(".venv").join("bin")
    }

    pub fn venv_path(&self) -> PathBuf {
        std::env::current_dir().unwrap().join(".venv")
    }
}

pub fn get_uv_tool_path(tool_name: &str) -> Option<String> {
    let user_home = dirs::home_dir();
    if let Some(user_home) = user_home {
        let tool_bin = user_home.join(".local").join("bin").join(tool_name);
        if tool_bin.exists() {
            return Some(tool_bin.to_string_lossy().to_string());
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_default_project() {
        let pyproject = PyProjectToml::get_default_project().unwrap();
        println!("{:#?}", pyproject);
        // println!("{:#?}", pyproject.get_poe_tasks());
    }
}
