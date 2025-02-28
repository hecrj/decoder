//! Encode your types.
use crate::value::to_value;
use crate::{Map, Value};

use serde::Serialize;

/// Serializes some data into a [`Value`] using the [`Serialize`] trait.
pub fn value(data: impl Serialize) -> Value {
    to_value(data).expect("Serialize value")
}

/// Creates a [`Map`] of fields from the given iterator.
pub fn map<'a>(fields: impl IntoIterator<Item = (&'a str, Value)>) -> Map {
    Map::from_iter(
        fields
            .into_iter()
            .map(|(key, value)| (Value::from(key), value)),
    )
}

/// Encodes an optional [`Value`] using the given encoder, if present.
pub fn optional<T, V>(f: impl Fn(T) -> V, value: Option<T>) -> Value
where
    V: Into<Value>,
{
    Value::from(value.map(f).map(V::into))
}

/// Uses the given encoder to encode a sequence of values.
pub fn sequence<T, V>(f: impl Fn(T) -> V, items: impl IntoIterator<Item = T>) -> Value
where
    V: Into<Value>,
{
    Value::from_iter(items.into_iter().map(f).map(V::into))
}

/// Creates a [`Map`] using a syntax similar to `format!`.
///
/// ```
/// use decoder::encode::map;
///
/// let cowboy = map!(name = "Spike", surname = "Spiegel");
/// ```
#[doc(hidden)]
#[macro_export]
macro_rules! __map__ {
    ($($key:ident=$value:expr),* $(,)?) => {
        $crate::encode::map([$((stringify!($key), $crate::Value::from($value))),*])
    };
}

#[doc(inline)]
pub use crate::__map__ as map;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_macro_creates_a_map() {
        let person = map!(name = "Spike", surname = "Spiegel");

        assert_eq!(
            person,
            map([("name", value("Spike")), ("surname", value("Spiegel"))])
        );
    }
}
