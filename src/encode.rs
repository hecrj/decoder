//! Encode your types.
use crate::value::to_value;
use crate::{Map, Value};

use decoder_value::Value as Raw;
use serde::Serialize;
use std::time::Duration;

/// Encodes a [`bool`](prim@bool).
pub fn bool(value: bool) -> Value {
    serialize(value)
}

/// Encodes a [`u8`](prim@u8).
pub fn u8(value: u8) -> Value {
    serialize(value)
}

/// Encodes a [`u16`](prim@u16).
pub fn u16(value: u16) -> Value {
    serialize(value)
}

/// Encodes a [`u32`](prim@u32).
pub fn u32(value: u32) -> Value {
    serialize(value)
}

/// Encodes a [`u64`](prim@u64).
pub fn u64(value: u64) -> Value {
    serialize(value)
}

/// Encodes an [`i8`](prim@i8).
pub fn i8(value: i8) -> Value {
    serialize(value)
}

/// Encodes an [`i16`](prim@i16).
pub fn i16(value: i16) -> Value {
    serialize(value)
}

/// Encodes an [`i32`](prim@i32).
pub fn i32(value: i32) -> Value {
    serialize(value)
}

/// Encodes an [`i64`](prim@i64).
pub fn i64(value: i64) -> Value {
    serialize(value)
}

/// Encodes an [`f32`](prim@f32).
pub fn f32(value: f32) -> Value {
    serialize(value)
}

/// Encodes an [`f64`](prim@f64).
pub fn f64(value: f64) -> Value {
    serialize(value)
}

/// Encodes a [`String`].
pub fn string(value: impl Into<String>) -> Value {
    serialize(value.into())
}

/// Encodes a [`Duration`].
pub fn duration(duration: Duration) -> Value {
    map([
        ("secs", u64(duration.as_secs())),
        ("nanos", u32(duration.subsec_nanos())),
    ])
    .into()
}

/// Creates a [`Map`] of fields from the given iterator.
pub fn map<'a>(fields: impl IntoIterator<Item = (&'a str, Value)>) -> Map {
    Map::from_iter(fields.into_iter().map(|(key, value)| (string(key), value)))
}

/// Encodes an optional [`Value`] using the given encoder, if present.
pub fn optional<T, V>(f: impl Fn(T) -> V, value: Option<T>) -> Value
where
    V: Into<Value>,
{
    Value(Raw::Option(
        value.map(|value| f(value).into().0).map(Box::new),
    ))
}

/// Uses the given encoder to encode a sequence of values.
pub fn sequence<T, V>(f: impl Fn(T) -> V, items: impl IntoIterator<Item = T>) -> Value
where
    V: Into<Value>,
{
    Value(Raw::Seq(
        items.into_iter().map(|value| f(value).into().0).collect(),
    ))
}

/// Serializes some data into a [`Value`] using the [`Serialize`] trait.
fn serialize(data: impl Serialize) -> Value {
    to_value(data).expect("Serialize value")
}
