use crate::{Decoder, Result, Value};

use serde::de::{DeserializeOwned, IntoDeserializer};

pub fn value<T: DeserializeOwned>(value: Value) -> Result<T> {
    Ok(T::deserialize(value.into_deserializer())?)
}

pub fn sequence<D: Decoder, B: FromIterator<D::Output>>(decoder: D) -> impl Decoder<Output = B> {
    move |value: Value| {
        value
            .into_sequence()?
            .into_iter()
            .map(|value| decoder.run(value))
            .collect()
    }
}

#[cfg(feature = "json")]
pub fn from_json<D: Decoder>(decoder: D, json: &str) -> Result<D::Output> {
    use serde::Deserialize;

    let value = Value::deserialize(&mut serde_json::Deserializer::from_str(json))
        .map_err(crate::Error::deserializer)?;

    decoder.run(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct User {
        name: String,
        age: u32,
        projects: Vec<Project>,
    }

    impl User {
        fn decode(value: Value) -> Result<Self> {
            let mut user = value.into_map()?;
            let name = user.required("name")?;
            let age = user.required("age")?;
            let projects = user.required_with("projects", sequence(Project::decode))?;

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
            if let Some(repository) = value.as_str() {
                return Ok(Project {
                    name: repository.split("/").last().unwrap_or("Unknown").to_owned(),
                    repository: repository.to_owned(),
                });
            }

            let mut project = value.into_map()?;
            let name = project.required("name")?;
            let repository = project.required("repository")?;

            Ok(Project { name, repository })
        }
    }

    #[test]
    fn it_works() {
        let user = from_json(
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
        )
    }
}
