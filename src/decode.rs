//! Decode your types.
use crate::value::Raw;
use crate::{Decoder, Error, Map, Result, Value};

use serde::de::{DeserializeOwned, IntoDeserializer};
use std::time::Duration;

/// Decodes a [`bool`](prim@bool).
pub fn bool(value: Value) -> Result<bool> {
    deserialize(value)
}

/// Decodes a [`u8`](prim@u8).
pub fn u8(value: Value) -> Result<u8> {
    deserialize(value)
}

/// Decodes a [`u16`](prim@u16).
pub fn u16(value: Value) -> Result<u16> {
    deserialize(value)
}

/// Decodes a [`u32`](prim@u32).
pub fn u32(value: Value) -> Result<u32> {
    deserialize(value)
}

/// Decodes a [`u64`](prim@u64).
pub fn u64(value: Value) -> Result<u64> {
    deserialize(value)
}

/// Decodes an [`i8`](prim@i8).
pub fn i8(value: Value) -> Result<i8> {
    deserialize(value)
}

/// Decodes an [`i16`](prim@i16).
pub fn i16(value: Value) -> Result<i16> {
    deserialize(value)
}

/// Decodes an [`i32`](prim@i32).
pub fn i32(value: Value) -> Result<i32> {
    deserialize(value)
}

/// Decodes an [`i64`](prim@i64).
pub fn i64(value: Value) -> Result<i64> {
    deserialize(value)
}

/// Decodes an [`f32`](prim@f32).
pub fn f32(value: Value) -> Result<f32> {
    deserialize(value)
}

/// Decodes an [`f64`](prim@f64).
pub fn f64(value: Value) -> Result<f64> {
    deserialize(value)
}

/// Decodes a [`String`].
pub fn string(value: Value) -> Result<String> {
    deserialize(value)
}

/// Decodes a [`str`](prim@str).
pub fn str(value: &Value) -> Result<&str> {
    match &value.0 {
        Raw::String(string) => Ok(string),
        value => Err(Error::InvalidType {
            expected: "string",
            got: value.unexpected(),
        }),
    }
}

/// Decodes a [`Map`].
pub fn map(value: Value) -> Result<Map> {
    match value.0 {
        Raw::Map(map) => Ok(Map { raw: map }),
        value => Err(Error::InvalidType {
            expected: "map",
            got: value.unexpected(),
        }),
    }
}

/// Decodes a [`Duration`].
pub fn duration(value: Value) -> Result<Duration> {
    let mut duration = map(value)?;

    let secs = duration.required("secs", u64)?;
    let nanos = duration.required("nanos", u32)?;

    Ok(Duration::new(secs, nanos))
}

/// Takes a [`Decoder`] of a certain type `T` and produces a new [`Decoder`] that
/// decodes an `Option<T>`.
pub fn optional<T>(decoder: impl Decoder<Output = T>) -> impl Decoder<Output = Option<T>> {
    move |value: Value| match value.0 {
        Raw::Unit | Raw::Option(None) => Ok(None),
        Raw::Option(Some(value)) => decoder.run(Value(*value)).map(Some),
        raw => decoder.run(Value(raw)).map(Some),
    }
}

/// Takes a [`Decoder`] of a certain type `T` and produces a new [`Decoder`] that
/// decodes a sequence of `T` values.
pub fn sequence<T, B: FromIterator<T>>(
    decoder: impl Decoder<Output = T>,
) -> impl Decoder<Output = B> {
    move |value: Value| match value.0 {
        Raw::Seq(sequence) => sequence
            .into_iter()
            .map(Value)
            .map(|value| decoder.run(value))
            .collect(),
        value => Err(Error::InvalidType {
            expected: "sequence",
            got: value.unexpected(),
        }),
    }
}

fn deserialize<T: DeserializeOwned>(value: Value) -> Result<T> {
    Ok(T::deserialize(value.into_deserializer())?)
}

#[cfg(test)]
mod tests {
    use crate::run;

    use super::*;

    struct User {
        name: String,
        age: u32,
        projects: Vec<Project>,
    }

    impl User {
        fn decode(value: Value) -> Result<Self> {
            let mut user = map(value)?;
            let name = user.required("name", string)?;
            let age = user.required("age", u32)?;
            let projects = user.required("projects", sequence(Project::decode))?;

            Ok(User {
                name,
                age,
                projects,
            })
        }
    }

    #[derive(Debug, PartialEq, Eq)]
    struct Project {
        name: String,
        repository: String,
    }

    impl Project {
        fn decode(value: Value) -> Result<Self> {
            if let Ok(repository) = str(&value) {
                return Ok(Project {
                    name: repository.split("/").last().unwrap_or("Unknown").to_owned(),
                    repository: repository.to_owned(),
                });
            }

            let mut project = map(value)?;
            let name = project.required("name", string)?;
            let repository = project.required("repository", string)?;

            Ok(Project { name, repository })
        }
    }

    #[test]
    fn it_works() {
        let user = run(
            serde_json::from_str,
            User::decode,
            r#"
            {
                "name": "Héctor",
                "age": 32,
                "projects": [ 
                    "https://github.com/iced-rs/iced",
                    { "name": "Sipper", "repository": "https://github.com/hecrj/sipper" }
                 ]
            }"#,
        )
        .expect("Decode user");

        assert_eq!(user.name, "Héctor");
        assert_eq!(user.age, 32);
        assert_eq!(
            user.projects,
            vec![
                Project {
                    name: "iced".to_owned(),
                    repository: "https://github.com/iced-rs/iced".to_owned()
                },
                Project {
                    name: "Sipper".to_owned(),
                    repository: "https://github.com/hecrj/sipper".to_owned()
                },
            ]
        );
    }
}
