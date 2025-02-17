pub use decoder_value::Unexpected;

use crate::Value;

use std::collections::BTreeMap;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub enum Error {
    InvalidType {
        expected: &'static str,
        got: Unexpected,
    },
    FieldMissing {
        key: String,
        map: BTreeMap<Value, Value>,
    },
    Custom(String),
    Deserializer(Arc<dyn std::error::Error + Send + Sync>),
}

impl Error {
    pub fn custom(error: impl Into<String>) -> Self {
        Self::Custom(error.into())
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
