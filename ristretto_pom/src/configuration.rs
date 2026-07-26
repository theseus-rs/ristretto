//! Arbitrary plugin and report configuration XML.

use serde::de::{MapAccess, Visitor};
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

/// Ordered plugin configuration that preserves nested and repeated XML elements.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Configuration {
    /// Configuration elements in declaration order.
    pub elements: Vec<ConfigurationElement>,
}

impl Configuration {
    /// Returns whether the configuration contains no elements.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    /// Appends a text-valued configuration element.
    pub fn insert(&mut self, name: impl Into<String>, value: impl Into<String>) {
        self.elements.push(ConfigurationElement {
            name: name.into(),
            value: ConfigurationValue::Text(value.into()),
        });
    }

    /// Returns the first element with the requested name.
    #[must_use]
    pub fn get(&self, name: &str) -> Option<&ConfigurationValue> {
        self.elements
            .iter()
            .find(|element| element.name == name)
            .map(|element| &element.value)
    }
}

/// One named plugin configuration element.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigurationElement {
    /// XML element name.
    pub name: String,
    /// Text or nested element content.
    pub value: ConfigurationValue,
}

/// Plugin configuration element content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(untagged)]
pub enum ConfigurationValue {
    /// Text content.
    Text(String),
    /// Nested configuration elements.
    Nested(Configuration),
}

impl<'de> Deserialize<'de> for ConfigurationValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum RawValue {
            Text(String),
            Nested(Configuration),
        }

        match RawValue::deserialize(deserializer)? {
            RawValue::Text(value) => Ok(Self::Text(value)),
            RawValue::Nested(mut configuration)
                if configuration.elements.first().is_some_and(|element| {
                    configuration.elements.len() == 1 && element.name == "$text"
                }) =>
            {
                Ok(configuration.elements.swap_remove(0).value)
            }
            RawValue::Nested(configuration) => Ok(Self::Nested(configuration)),
        }
    }
}

impl<'de> Deserialize<'de> for Configuration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ConfigurationVisitor;

        impl<'de> Visitor<'de> for ConfigurationVisitor {
            type Value = Configuration;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("plugin configuration XML elements")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut elements = Vec::new();
                while let Some(name) = map.next_key()? {
                    elements.push(ConfigurationElement {
                        name,
                        value: map.next_value()?,
                    });
                }
                Ok(Configuration { elements })
            }
        }

        deserializer.deserialize_map(ConfigurationVisitor)
    }
}

impl Serialize for Configuration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.elements.len()))?;
        for element in &self.elements {
            map.serialize_entry(&element.name, &element.value)?;
        }
        map.end()
    }
}
