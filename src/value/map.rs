use crate::{Decoder, Error, Result, Value};

use decoder_value::Value as Raw;
use indexmap::IndexMap;

/// A map of fields and their values, sorted by order of insertion.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Map {
    pub(crate) raw: IndexMap<Raw, Raw>,
}

impl Default for Map {
    fn default() -> Self {
        Self::new()
    }
}

impl Map {
    /// Creates a new empty [`Map`].
    pub fn new() -> Self {
        Self {
            raw: IndexMap::new(),
        }
    }

    /// Decodes the given field of the [`Map`] using the given [`Decoder`].
    pub fn required<T>(&mut self, key: &str, decoder: impl Decoder<Output = T>) -> Result<T> {
        let value = self.get(key)?;

        decoder.run(value)
    }

    /// Decodes the given field of the [`Map`] using the given [`Decoder`], if present.
    pub fn optional<T>(
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

    /// Inserts a field in the [`Map`] before all the other fields.
    pub fn tag(mut self, key: &str, value: impl Into<String>) -> Self {
        let _ = self
            .raw
            .insert_before(0, Raw::String(key.to_owned()), Raw::String(value.into()));
        self
    }

    /// Extends the [`Map`] with the fields of the given one.
    pub fn extend(mut self, other: Self) -> Self {
        self.raw.extend(other.raw);
        self
    }

    /// Converts the [`Map`] into a [`Value`].
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
