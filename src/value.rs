mod map;

pub use map::Map;

use crate::Error;
pub(crate) use decoder_value::Value as Raw;

use serde::Serialize;
use serde::de;
use serde::ser;

/// A generic value.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Value(pub(crate) Raw);

pub(crate) fn to_value(data: impl Serialize) -> Result<Value, Error> {
    decoder_value::to_value(data)
        .map(Value)
        .map_err(Error::deserializer)
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
