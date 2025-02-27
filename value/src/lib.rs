mod de;
mod ser;

pub use de::{DeserializerError, Unexpected};
pub use ser::{to_value, SerializerError};

use indexmap::IndexMap;
use ordered_float::OrderedFloat;

use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug)]
pub enum Value {
    Bool(bool),

    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),

    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),

    F32(f32),
    F64(f64),

    Char(char),
    String(String),

    Unit,
    Option(Option<Box<Value>>),
    Newtype(Box<Value>),
    Seq(Vec<Value>),
    Map(IndexMap<Value, Value>),
    Bytes(Vec<u8>),
}

impl Value {
    fn discriminant(&self) -> usize {
        match *self {
            Value::Bool(..) => 0,
            Value::U8(..) => 1,
            Value::U16(..) => 2,
            Value::U32(..) => 3,
            Value::U64(..) => 4,
            Value::I8(..) => 5,
            Value::I16(..) => 6,
            Value::I32(..) => 7,
            Value::I64(..) => 8,
            Value::F32(..) => 9,
            Value::F64(..) => 10,
            Value::Char(..) => 11,
            Value::String(..) => 12,
            Value::Unit => 13,
            Value::Option(..) => 14,
            Value::Newtype(..) => 15,
            Value::Seq(..) => 16,
            Value::Map(..) => 17,
            Value::Bytes(..) => 18,
        }
    }

    pub fn unexpected(&self) -> de::Unexpected {
        self.unexpected_().into()
    }

    fn unexpected_(&self) -> serde::de::Unexpected {
        match *self {
            Value::Bool(b) => serde::de::Unexpected::Bool(b),
            Value::U8(n) => serde::de::Unexpected::Unsigned(n as u64),
            Value::U16(n) => serde::de::Unexpected::Unsigned(n as u64),
            Value::U32(n) => serde::de::Unexpected::Unsigned(n as u64),
            Value::U64(n) => serde::de::Unexpected::Unsigned(n),
            Value::I8(n) => serde::de::Unexpected::Signed(n as i64),
            Value::I16(n) => serde::de::Unexpected::Signed(n as i64),
            Value::I32(n) => serde::de::Unexpected::Signed(n as i64),
            Value::I64(n) => serde::de::Unexpected::Signed(n),
            Value::F32(n) => serde::de::Unexpected::Float(n as f64),
            Value::F64(n) => serde::de::Unexpected::Float(n),
            Value::Char(c) => serde::de::Unexpected::Char(c),
            Value::String(ref s) => serde::de::Unexpected::Str(s),
            Value::Unit => serde::de::Unexpected::Unit,
            Value::Option(_) => serde::de::Unexpected::Option,
            Value::Newtype(_) => serde::de::Unexpected::NewtypeStruct,
            Value::Seq(_) => serde::de::Unexpected::Seq,
            Value::Map(_) => serde::de::Unexpected::Map,
            Value::Bytes(ref b) => serde::de::Unexpected::Bytes(b),
        }
    }
}

impl Hash for Value {
    fn hash<H>(&self, hasher: &mut H)
    where
        H: Hasher,
    {
        self.discriminant().hash(hasher);
        match *self {
            Value::Bool(v) => v.hash(hasher),
            Value::U8(v) => v.hash(hasher),
            Value::U16(v) => v.hash(hasher),
            Value::U32(v) => v.hash(hasher),
            Value::U64(v) => v.hash(hasher),
            Value::I8(v) => v.hash(hasher),
            Value::I16(v) => v.hash(hasher),
            Value::I32(v) => v.hash(hasher),
            Value::I64(v) => v.hash(hasher),
            Value::F32(v) => OrderedFloat(v).hash(hasher),
            Value::F64(v) => OrderedFloat(v).hash(hasher),
            Value::Char(v) => v.hash(hasher),
            Value::String(ref v) => v.hash(hasher),
            Value::Unit => {}
            Value::Option(ref v) => v.hash(hasher),
            Value::Newtype(ref v) => v.hash(hasher),
            Value::Seq(ref v) => v.hash(hasher),
            Value::Map(ref v) => {
                for (key, value) in v.iter() {
                    key.hash(hasher);
                    value.hash(hasher);
                }
            }
            Value::Bytes(ref v) => v.hash(hasher),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, rhs: &Self) -> bool {
        match (self, rhs) {
            (&Value::Bool(v0), &Value::Bool(v1)) if v0 == v1 => true,
            (&Value::U8(v0), &Value::U8(v1)) if v0 == v1 => true,
            (&Value::U16(v0), &Value::U16(v1)) if v0 == v1 => true,
            (&Value::U32(v0), &Value::U32(v1)) if v0 == v1 => true,
            (&Value::U64(v0), &Value::U64(v1)) if v0 == v1 => true,
            (&Value::I8(v0), &Value::I8(v1)) if v0 == v1 => true,
            (&Value::I16(v0), &Value::I16(v1)) if v0 == v1 => true,
            (&Value::I32(v0), &Value::I32(v1)) if v0 == v1 => true,
            (&Value::I64(v0), &Value::I64(v1)) if v0 == v1 => true,
            (&Value::F32(v0), &Value::F32(v1)) if OrderedFloat(v0) == OrderedFloat(v1) => true,
            (&Value::F64(v0), &Value::F64(v1)) if OrderedFloat(v0) == OrderedFloat(v1) => true,
            (&Value::Char(v0), &Value::Char(v1)) if v0 == v1 => true,
            (Value::String(v0), Value::String(v1)) if v0 == v1 => true,
            (&Value::Unit, &Value::Unit) => true,
            (Value::Option(v0), Value::Option(v1)) if v0 == v1 => true,
            (Value::Newtype(v0), Value::Newtype(v1)) if v0 == v1 => true,
            (Value::Seq(v0), Value::Seq(v1)) if v0 == v1 => true,
            (Value::Map(v0), Value::Map(v1)) if v0 == v1 => true,
            (Value::Bytes(v0), Value::Bytes(v1)) if v0 == v1 => true,
            _ => false,
        }
    }
}

impl Ord for Value {
    fn cmp(&self, rhs: &Self) -> Ordering {
        match (self, rhs) {
            (&Value::Bool(v0), Value::Bool(v1)) => v0.cmp(v1),
            (&Value::U8(v0), Value::U8(v1)) => v0.cmp(v1),
            (&Value::U16(v0), Value::U16(v1)) => v0.cmp(v1),
            (&Value::U32(v0), Value::U32(v1)) => v0.cmp(v1),
            (&Value::U64(v0), Value::U64(v1)) => v0.cmp(v1),
            (&Value::I8(v0), Value::I8(v1)) => v0.cmp(v1),
            (&Value::I16(v0), Value::I16(v1)) => v0.cmp(v1),
            (&Value::I32(v0), Value::I32(v1)) => v0.cmp(v1),
            (&Value::I64(v0), Value::I64(v1)) => v0.cmp(v1),
            (&Value::F32(v0), &Value::F32(v1)) => OrderedFloat(v0).cmp(&OrderedFloat(v1)),
            (&Value::F64(v0), &Value::F64(v1)) => OrderedFloat(v0).cmp(&OrderedFloat(v1)),
            (&Value::Char(v0), Value::Char(v1)) => v0.cmp(v1),
            (Value::String(v0), Value::String(v1)) => v0.cmp(v1),
            (&Value::Unit, &Value::Unit) => Ordering::Equal,
            (Value::Option(v0), Value::Option(v1)) => v0.cmp(v1),
            (Value::Newtype(v0), Value::Newtype(v1)) => v0.cmp(v1),
            (Value::Seq(v0), Value::Seq(v1)) => v0.cmp(v1),
            (Value::Map(v0), Value::Map(v1)) => v0.iter().cmp(v1.iter()),
            (Value::Bytes(v0), Value::Bytes(v1)) => v0.cmp(v1),
            (v0, v1) => v0.discriminant().cmp(&v1.discriminant()),
        }
    }
}

impl Eq for Value {}

impl PartialOrd for Value {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}
