use std::any::Any;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PyProjectToml {
    tool: Option<Tool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Tool {
    rye: Option<ToolRye>,
    poetry: Option<ToolPoetry>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct ToolRye {
    scripts: Option<HashMap<String, toml::Value>>,
}

impl ToolRye {
    pub fn get_scripts(&self) -> Option<HashMap<String, String>> {
        self.scripts.as_ref().map(|scripts| {
            scripts.iter().map(|(key, value)| {
                let description: String = match value {
                    toml::Value::String(value) => value.to_string(),
                    toml::Value::Table(table) => table.get("cmd").unwrap_or(&toml::Value::String("".to_owned())).to_string(),
                    _ => "".to_owned()
                };
                return (key.clone(), description);
            }).collect()
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
            scripts.iter().map(|(key, value)| {
                let description: String = match value {
                    toml::Value::String(value) => value.to_string(),
                    toml::Value::Table(table) => table.get("callable").unwrap_or(&toml::Value::String("".to_owned())).to_string(),
                    _ => "".to_owned()
                };
                return (key.clone(), description);
            }).collect()
        })
    }
}

impl PyProjectToml {
    pub fn get_default_project() -> Result<Self, std::io::Error> {
        if let Ok(pyproject_file) = std::env::current_dir()
            .map(|dir| {
                dir.join("pyproject.toml")
            }) {
            let text = std::fs::read_to_string(pyproject_file).unwrap_or("".to_owned());
            toml::from_str(&text).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
        } else {
            Err(std::io::Error::new(std::io::ErrorKind::NotFound, "pyproject.toml not found").into())
        }
    }

    pub fn rye_available(&self) -> bool {
        self.tool.as_ref().map(|tool| tool.rye.is_some()).unwrap_or(false)
    }

    pub fn poetry_available(&self) -> bool {
        self.tool.as_ref().map(|tool| tool.poetry.is_some()).unwrap_or(false)
    }

    pub fn get_rye_scripts(&self) -> Option<HashMap<String, String>> {
        self.tool.as_ref().and_then(|tool| tool.rye.as_ref()).and_then(|rye| rye.get_scripts().clone())
    }

    pub fn get_poetry_scripts(&self) -> Option<HashMap<String, String>> {
        self.tool.as_ref().and_then(|tool| tool.poetry.as_ref()).and_then(|poetry| poetry.get_scripts().clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_default_project() {
        let pyproject = PyProjectToml::get_default_project().unwrap();
        println!("{:#?}", pyproject);
    }
}
