<div align="center">

# Decoder

[![Documentation](https://docs.rs/decoder/badge.svg)](https://docs.rs/decoder)
[![Crates.io](https://img.shields.io/crates/v/decoder.svg)](https://crates.io/crates/decoder)
[![License](https://img.shields.io/crates/l/decoder.svg)](https://github.com/hecrj/decoder/blob/master/LICENSE)
[![Downloads](https://img.shields.io/crates/d/decoder.svg)](https://crates.io/crates/decoder)
[![Test Status](https://img.shields.io/github/actions/workflow/status/hecrj/decoder/test.yml?branch=master&event=push&label=test)](https://github.com/hecrj/decoder/actions)

A decoder library for your types.
</div>

When using [`serde`], your types become entangled with serialization logic due to the [`Serialize`] and [`Deserialize`] traits.

This crate lets you decouple serialization logic by leveraging simple functions, at some performance cost:

```rust
use decoder::{Map, Result, Value};

struct Person {
    name: String,
    projects: Vec<Project>,
}

struct Project {
    name: String,
    url: String,
}

impl Person {
    fn decode(value: Value) -> Result<Self> {
        use decoder::decode::{map, sequence, string};

        let mut person = map(value)?;

        Ok(Self {
            name: person.required("name", string)?,
            projects: person.required("projects", sequence(Project::decode))?,
        })
    }

    fn encode(&self) -> Value {
        use decoder::encode::{map, sequence, string};

        map([
            ("name", string(&self.name)),
            ("projects", sequence(Project::encode, &self.projects)),
        ])
        .into()
    }
}

impl Project {
    fn decode(value: Value) -> Result<Self> {
        use decoder::decode::{map, string};

        let mut project = map(value)?;

        Ok(Project {
            name: project.required("name", string)?,
            url: project.required("url", string)?
        })
    }

    fn encode(&self) -> Value {
        use decoder::encode::{map, string};

        map([
            ("name", string(&self.name)),
            ("url", string(&self.url)),
        ])
        .into()
    }
}

let person =
    decoder::run(serde_json::from_str, Person::decode, "{ ... }").expect("Decode person");

let _ = serde_json::to_string(&person.encode());
```

You can try this crate if the [`serde`] way™ has become painful or it does not resonate with you.

[`serde`]: https://serde.rs
[`Serialize`]: https://docs.rs/serde/latest/serde/trait.Serialize.html
[`Deserialize`]: https://docs.rs/serde/latest/serde/trait.Deserialize.html
