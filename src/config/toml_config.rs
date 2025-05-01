use std::collections::HashMap;

use serde::de::Error as SerdeError;
use serde::{Deserialize, Serialize};
use toml_edit::{DocumentMut, InlineTable, Item, Value};

pub trait TomlEditor {
    fn edit_toml_table(&self, table: &mut Item);
}

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

type DependencyMap = HashMap<String, Dependency>;

#[derive(Serialize, Deserialize, Debug)]
pub struct EnvimConfig {
    #[serde(skip_serializing, skip_deserializing)]
    toml_doc: DocumentMut,
    pub workspace: Workspace,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Workspace {
    pub dependencies: DependencyMap,
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
            toml_doc: DocumentMut::new(),
            workspace: Workspace {
                plugin_manager: PluginManager::Lazy,
                dependencies: HashMap::new(),
            },
        }
    }

    pub fn edit_toml(&mut self) {
        if !self
            .toml_doc
            .get_key_value("workspace")
            .is_some_and(|(_, y)| y.is_table())
        {
            self.toml_doc["workspace"] = toml_edit::table();
            self.toml_doc["workspace"]
                .as_table_mut()
                .map(|t| t.set_implicit(true));
        }

        self.workspace
            .edit_toml_table(&mut self.toml_doc["workspace"]);
    }

    pub fn toml_to_string(&self) -> String {
        self.toml_doc.to_string()
    }

    pub fn set_doc(&mut self, doc: DocumentMut) {
        self.toml_doc = doc;
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

// TomlEditor trait for PluginManager

impl TomlEditor for PluginManager {
    fn edit_toml_table(&self, table: &mut Item) {
        table["plugin_manager"] = toml_edit::value(match self {
            PluginManager::Lazy => "lazy",
        });
    }
}

// TomlEditor for DependencyMap

impl TomlEditor for DependencyMap {
    fn edit_toml_table(&self, table: &mut Item) {
        for (key, val) in self {
            table[key] = Item::Value(val.into());
        }
    }
}

// TomlEditor for Workspace

impl TomlEditor for Workspace {
    fn edit_toml_table(&self, table: &mut Item) {
        if !table
            .get("dependencies")
            .is_some_and(|x| x.is_table())
        {
            table["dependencies"] = toml_edit::table();
        }

        self.plugin_manager.edit_toml_table(table);
        self.dependencies
            .edit_toml_table(&mut table["dependencies"]);
    }
}

// toml_edit::Value::From<Dependency>

impl From<&Dependency> for Value {
    fn from(dep: &Dependency) -> Self {
        if dep.tag.is_some() && dep.commit.is_none() {
            return dep.tag.as_ref().unwrap().into();
        }

        if let Some(commit) = &dep.commit {
            let mut tbl = InlineTable::new();
            tbl.insert("commit", commit.into());
            if let Some(tag) = &dep.tag {
                tbl.insert("tag", tag.into());
            }
            return Value::InlineTable(tbl);
        }
        unreachable!()
    }
}
