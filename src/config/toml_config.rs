use std::collections::HashMap;

use serde::de::Error as SerdeError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum DependencyEnum {
    Tag(String),
    Metadata(HashMap<String, String>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PluginManager {
    Lazy,
}

impl<'de> Deserialize<'de> for Dependency {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let dependency = DependencyEnum::deserialize(deserializer)?;
        match dependency {
            DependencyEnum::Tag(tag) => Ok(Dependency {
                tag: Some(tag),
                commit: None,
            }),
            DependencyEnum::Metadata(map) => {
                if map.contains_key("tag") || map.contains_key("commit") {
                    return Ok(Dependency {
                        tag: map.get("tag").cloned(),
                        commit: map.get("commit").cloned(),
                    });
                }
                Err(SerdeError::missing_field("commit"))
            }
        }
    }
}

// toml file structure

#[derive(Serialize, Deserialize, Debug)]
pub struct EnvimConfig {
    pub workspace: Workspace,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Workspace {
    pub dependencies: HashMap<String, Dependency>,
    pub plugin_manager: PluginManager,
}

#[derive(Serialize, Debug)]
pub struct Dependency {
    pub tag: Option<String>,
    pub commit: Option<String>,
}

// EnvimConfig new method

impl EnvimConfig {
    pub fn new() -> Self {
        EnvimConfig {
            workspace: Workspace {
                plugin_manager: PluginManager::Lazy,
                dependencies: HashMap::new(),
            },
        }
    }
}

// Dependency new method
impl Dependency {
    pub fn new() -> Self {
        Dependency {
            tag: None,
            commit: None,
        }
    }
}
