use crate::{Decoder, Error, Result, Value};

use serde::de::{DeserializeOwned, IntoDeserializer};

pub fn value<T: DeserializeOwned>(value: Value) -> Result<T> {
    Ok(T::deserialize(value.into_deserializer())?)
}

pub fn sequence<T, B: FromIterator<T>>(
    decoder: impl Decoder<Output = T>,
) -> impl Decoder<Output = B> {
    move |value: Value| {
        value
            .into_sequence()?
            .into_iter()
            .map(|value| decoder.run(value))
            .collect()
    }
}

pub fn run<T, I, E>(
    deserialize: impl Fn(I) -> std::result::Result<Value, E>,
    decoder: impl Decoder<Output = T>,
    input: I,
) -> Result<T>
where
    E: std::error::Error + Send + Sync + 'static,
{
    decoder.run(deserialize(input).map_err(Error::deserializer)?)
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
        )
    }
}
