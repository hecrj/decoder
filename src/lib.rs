pub mod decode;
pub mod encode;

mod error;
mod value;

pub use error::Error;
pub use value::{Map, Value, to_value};

pub type Result<T> = std::result::Result<T, Error>;

pub trait Decoder {
    type Output;

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
