use crate::decode;
use crate::{Decoder, Error, Result, Value};

use decoder_value::Value as Raw;
use indexmap::IndexMap;
use serde::de::DeserializeOwned;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Map {
    pub(crate) raw: IndexMap<Raw, Raw>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            raw: IndexMap::new(),
        }
    }

    pub fn required<T: DeserializeOwned>(&mut self, key: &str) -> Result<T> {
        self.required_with(key, decode::value)
    }

    pub fn required_with<T>(&mut self, key: &str, decoder: impl Decoder<Output = T>) -> Result<T> {
        let value = self.get(key)?;

        decoder.run(value)
    }

    pub fn optional<T: DeserializeOwned>(&mut self, key: &str) -> Result<Option<T>> {
        self.optional_with(key, decode::value)
    }

    pub fn optional_with<T>(
        &mut self,
        key: &str,
        decoder: impl Decoder<Output = T>,
    ) -> Result<Option<T>> {
        let Ok(value) = self.get(key) else {
            return Ok(None);
        };

        if let Raw::Unit = value.0 {
            return Ok(None);
        }

        decoder.run(value).map(Some)
    }

    pub fn tag(mut self, key: &str, value: impl Into<Value>) -> Self {
        self.raw
            .insert_before(0, Raw::String(key.to_owned()), value.into().0);
        self
    }

    pub fn extend(mut self, other: Self) -> Self {
        self.raw.extend(other.raw);
        self
    }

    pub fn into_value(self) -> Value {
        Value::from(Raw::Map(self.raw))
    }

    fn get(&mut self, key: &str) -> Result<Value> {
        self.raw
            .shift_remove(&Raw::String(key.to_owned()))
            .ok_or_else(|| Error::FieldMissing {
                key: key.to_owned(),
                map: self
                    .raw
                    .iter()
                    .map(|(key, value)| (Value::from(key.clone()), Value::from(value.clone())))
                    .collect(),
            })
            .map(Value::from)
    }
}

impl FromIterator<(Value, Value)> for Map {
    fn from_iter<T: IntoIterator<Item = (Value, Value)>>(iter: T) -> Self {
        Self {
            raw: iter
                .into_iter()
                .map(|(key, value)| (key.0, value.0))
                .collect(),
        }
    }
}
