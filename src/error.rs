pub use decoder_value::Unexpected;

use crate::Value;

use std::collections::BTreeMap;
use std::sync::Arc;

/// A decoder error.
#[allow(missing_docs)]
#[derive(Debug, Clone, thiserror::Error)]
pub enum Error {
    /// A type was invalid.
    #[error("invalid type (expected: {expected}, got: {got:?})")]
    InvalidType {
        expected: &'static str,
        got: Unexpected,
    },
    /// A field was missing.
    #[error("missing field (key: {key}, map: {map:?})")]
    FieldMissing {
        key: String,
        map: BTreeMap<Value, Value>,
    },
    /// A custom error.
    #[error("custom error: {0}")]
    Custom(String),
    /// A deserializer error.
    #[error("deserializer error: {0}")]
    Deserializer(Arc<dyn std::error::Error + Send + Sync>),
}

impl Error {
    /// Creates a custom [`Error`].
    pub fn custom(error: impl ToString) -> Self {
        Self::Custom(error.to_string())
    }

    pub(crate) fn deserializer(error: impl std::error::Error + Send + Sync + 'static) -> Self {
        Self::Deserializer(Arc::new(error))
    }
}

impl From<decoder_value::DeserializerError> for Error {
    fn from(error: decoder_value::DeserializerError) -> Self {
        Self::Deserializer(Arc::new(error))
    }
}
