use crate::{to_value, Map, Value};

use serde::Serialize;

#[doc(hidden)]
#[macro_export]
macro_rules! __map__ {
    ($($key:ident=$value:expr),* $(,)?) => {
        $crate::encode::map([$((stringify!($key), $crate::Value::from($value))),*])
    };
}

#[doc(inline)]
pub use crate::__map__ as map;

pub fn value(data: impl Serialize) -> Value {
    to_value(data).expect("Serialize value")
}

pub fn map<'a>(fields: impl IntoIterator<Item = (&'a str, Value)>) -> Map {
    Map::from_iter(
        fields
            .into_iter()
            .map(|(key, value)| (Value::from(key), value)),
    )
}

pub fn optional<T, V>(f: impl Fn(T) -> V, value: Option<T>) -> Value
where
    V: Into<Value>,
{
    Value::from(value.map(f).map(V::into))
}

pub fn sequence<T, V>(f: impl Fn(T) -> V, items: impl IntoIterator<Item = T>) -> Value
where
    V: Into<Value>,
{
    Value::from_iter(items.into_iter().map(f).map(V::into))
}

pub fn with<A, B, C>(f: impl Fn(A, B) -> C, first: A) -> impl Fn(B) -> C
where
    A: Copy,
{
    move |second| f(first, second)
}

#[cfg(feature = "json")]
pub fn to_json(value: impl Into<Value>) -> Result<String, serde_json::Error> {
    serde_json::to_string(&value.into())
}

#[cfg(feature = "json")]
pub fn to_json_pretty(value: impl Into<Value>) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(&value.into())
}

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
