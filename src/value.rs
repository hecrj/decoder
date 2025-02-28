mod map;

pub use map::Map;

use crate::Error;

use decoder_value::Value as Raw;
use serde::Serialize;
use serde::de;
use serde::ser;

/// A generic value.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Value(Raw);

impl Value {
    /// Returns the [`Value`] as a string slice, if it is a string.
    pub fn as_str(&self) -> Option<&str> {
        match &self {
            Self(Raw::String(string)) => Some(string),
            _ => None,
        }
    }

    /// Converts the [`Value`] into a sequence, if it is a sequence.
    pub fn into_sequence(self) -> Result<impl Iterator<Item = Value>, Error> {
        match self.0 {
            Raw::Seq(values) => Ok(values.into_iter().map(Self)),
            _ => Err(Error::InvalidType {
                expected: "sequence",
                got: self.0.unexpected(),
            }),
        }
    }

    /// Converts the [`Value`] into a [`Map`], if it is a map.
    pub fn into_map(self) -> Result<Map, Error> {
        match self.0 {
            Raw::Map(map) => Ok(Map { raw: map }),
            _ => Err(Error::InvalidType {
                expected: "map",
                got: self.0.unexpected(),
            }),
        }
    }
}

pub(crate) fn to_value(data: impl Serialize) -> Result<Value, Error> {
    Ok(Value(
        decoder_value::to_value(data).map_err(Error::deserializer)?,
    ))
}

impl From<Raw> for Value {
    fn from(value: Raw) -> Self {
        Self(value)
    }
}

impl From<Map> for Value {
    fn from(map: Map) -> Self {
        Self(Raw::Map(map.raw))
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Self(Raw::String(value))
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Self::from(value.to_owned())
    }
}

impl From<&String> for Value {
    fn from(value: &String) -> Self {
        Self::from(value.to_owned())
    }
}

impl From<Option<String>> for Value {
    fn from(value: Option<String>) -> Self {
        Self(Raw::Option(value.map(Raw::String).map(Box::new)))
    }
}

impl From<Option<Value>> for Value {
    fn from(value: Option<Value>) -> Self {
        Self(Raw::Option(value.map(|value| value.0).map(Box::new)))
    }
}

impl FromIterator<Value> for Value {
    fn from_iter<T: IntoIterator<Item = Value>>(iter: T) -> Self {
        Self(Raw::Seq(iter.into_iter().map(|value| value.0).collect()))
    }
}

impl<'de> de::Deserialize<'de> for Value {
    fn deserialize<D: de::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Raw::deserialize(d).map(Self)
    }
}

impl de::IntoDeserializer<'_, decoder_value::DeserializerError> for Value {
    type Deserializer = Raw;

    fn into_deserializer(self) -> Raw {
        self.0
    }
}

impl ser::Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        self.0.serialize(serializer)
    }
}
