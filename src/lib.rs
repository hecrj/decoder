//! A decoder library for your types.
//!
//! When using [`serde`], your types become entangled with serialization logic due to the [`Serialize`] and [`Deserialize`] traits.
//!
//! This crate lets you decouple serialization logic by leveraging simple functions, at some performance cost:
//!
//! ```rust,no_run
//! use decoder::{Map, Result, Value};
//!
//! struct Person {
//!     name: String,
//!     projects: Vec<Project>
//! }
//!
//! struct Project {
//!     name: String,
//!     url: String,
//! }
//!
//! impl Person {
//!     fn decode(value: Value) -> Result<Self> {
//!         use decoder::decode::{map, sequence, string};
//!
//!         let mut person = map(value)?;
//!
//!         Ok(Self {
//!             name: person.required("name", string)?,
//!             projects: person.required("projects", sequence(Project::decode))?,
//!         })
//!     }
//!
//!     fn encode(&self) -> Value {
//!         use decoder::encode::{map, sequence, string};
//!
//!         map([
//!             ("name", string(&self.name)),
//!             ("projects", sequence(Project::encode, &self.projects)),
//!         ])
//!         .into()
//!     }
//! }
//!
//! impl Project {
//!     fn decode(value: Value) -> Result<Self> {
//!         use decoder::decode::{map, string};
//!
//!         let mut project = map(value)?;
//!
//!         Ok(Project {
//!             name: project.required("name", string)?,
//!             url: project.required("url", string)?
//!         })
//!     }
//!
//!     fn encode(&self) -> Value {
//!         use decoder::encode::{map, string};
//!
//!         map([
//!             ("name", string(&self.name)),
//!             ("url", string(&self.url)),
//!         ])
//!         .into()
//!     }
//! }
//!
//! let person =
//!    decoder::run(serde_json::from_str, Person::decode, "{ ... }").expect("Decode person");
//!
//! let _ = serde_json::to_string(&person.encode());
//! ```
//!
//! You can try this crate if the [`serde`] way™ has become painful or it does not resonate with you.
//!
//! [`serde`]: https://serde.rs
//! [`Serialize`]: https://docs.rs/serde/latest/serde/trait.Serialize.html
//! [`Deserialize`]: https://docs.rs/serde/latest/serde/trait.Deserialize.html
pub mod decode;
pub mod encode;

mod error;
mod value;

pub use error::Error;
pub use value::{Map, Value};

/// A decoding result.
pub type Result<T> = std::result::Result<T, Error>;

/// Some logic that turns a [`Value`] into some [`Output`](Self::Output).
pub trait Decoder {
    /// The output of the [`Decoder`].
    type Output;

    /// Runs the [`Decoder`].
    fn run(&self, value: Value) -> Result<Self::Output>;
}

impl<F, T> Decoder for F
where
    F: Fn(Value) -> Result<T>,
{
    type Output = T;

    fn run(&self, value: Value) -> Result<T> {
        self(value)
    }
}

/// Runs a [`Decoder`] using the given function to deserialize a [`Value`]
/// from the given input.
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
