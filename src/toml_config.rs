use std::{collections::HashMap, fmt};

use serde::{de::{self, Visitor}, ser::SerializeSeq, Deserialize, Serialize};

// Plugin Manager Enum with serializer and deserializer trait implementations
#[derive(Debug)]
pub enum PluginManager {
    Lazy,
}

struct PluginManagerVisitor;

impl<'de> Visitor<'de> for PluginManagerVisitor {
    type Value = PluginManager;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string representing the plugin manager")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match value {
            "lazy" | "lazynvim" => Ok(PluginManager::Lazy),
            _ => Err(serde::de::Error::custom("Invalid plugin manager")),
        }
    }
}

impl<'de> Deserialize<'de> for PluginManager {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(PluginManagerVisitor)
    }
}

impl Serialize for PluginManager {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            PluginManager::Lazy => serializer.serialize_str("lazy"),
        }
    }
}

// DependencyMap holds a map of dependencies created from a dependency array
#[derive(Debug)]
pub struct DependencyMap(HashMap<String, Dependency>);

struct DependencyMapVisitor;

impl Serialize for DependencyMap {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.0.len()))?;
        for (_, dependency) in &self.0 {
            seq.serialize_element(dependency)?;
        }
        seq.end()
    }
}

impl<'de> Visitor<'de> for DependencyMapVisitor {
    type Value = DependencyMap;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a list of dependencies")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let mut dep_map = DependencyMap(HashMap::new());
        while let Some(val) = seq.next_element::<Dependency>()? {
            dep_map.0.insert(val.name.clone(), val);
        }
        Ok(dep_map)
    }
}

impl<'de> Deserialize<'de> for DependencyMap {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_seq(DependencyMapVisitor)
    }
}

// Dependency Deserializer

struct DependencyVisitor;

impl<'de> Visitor<'de> for DependencyVisitor {
    type Value = Dependency;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a dependency entry")
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
        where
            E: serde::de::Error, {
        Ok(Dependency {name: v, version: None})
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::MapAccess<'de>, {

        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Fields { Name, Version }

        let mut name = None;
        let mut version = None;

        while let Some(key) = map.next_key()? {
            match key {
                Fields::Name => { 
                    name = Some(map.next_value()?);
                },
                Fields::Version => {
                    version = Some(map.next_value()?);
                }
            }
        }

        let name = name.ok_or_else(|| de::Error::missing_field("name"))?;

        Ok(Dependency {name, version})
    }
}

impl<'de> Deserialize<'de> for Dependency {
    

    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {
        deserializer.deserialize_any(DependencyVisitor)
    }
}

// toml file structure

#[derive(Serialize, Deserialize, Debug)]
pub struct EnvimConfig {
    pub workspace: Workspace,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Workspace {
    pub dependencies: DependencyMap,
    pub plugin_manager: PluginManager,
}

#[derive(Serialize, Debug)]
pub struct Dependency {
    pub name: String,
    pub version: Option<String>,
}
