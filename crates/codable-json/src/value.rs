use std::str::FromStr;

use indexmap::IndexMap;

use codable::{
    dec::{Decode, DecodeResult, Decoder, SeqContainer as _, ValueContainer as _},
    enc::{Encode, ValueContainer as _},
    CodingKey, CodingPath,
};

use crate::{
    dec::{self, Error},
    enc, JsonDecoder, JsonEncoder,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    String(String),
    Number(String),
    Bool(bool),
    Null,
    Array(Vec<Value>),
    Object(IndexMap<String, Value>),
}

impl Default for Value {
    fn default() -> Self {
        Self::Null
    }
}

#[inline(always)]
pub fn to_value<T: Encode>(input: &T) -> Result<Value, enc::Error> {
    let mut encoder = JsonEncoder::with_path(CodingPath::root(CodingKey::Root));
    input.encode(&mut encoder)
}

#[inline(always)]
pub fn from_value<T: Decode>(input: &Value) -> Result<T, dec::Error> {
    let mut decoder = JsonDecoder::new(CodingPath::root(CodingKey::Root), input);
    T::decode(&mut decoder)
}

impl Value {
    pub fn is_scalar(&self) -> bool {
        match self {
            Value::Array(_) | Value::Object(_) => false,
            _ => true,
        }
    }

    pub fn as_map(&self) -> Result<&IndexMap<String, Value>, Error> {
        match self {
            Value::Object(ref x) => Ok(x),
            _ => Err(Error::InvalidType),
        }
    }

    pub fn as_array(&self) -> Result<&Vec<Value>, Error> {
        match self {
            Value::Array(ref x) => Ok(x),
            _ => Err(Error::InvalidType),
        }
    }

    pub fn into_map(self) -> Result<IndexMap<String, Value>, Error> {
        match self {
            Value::Object(x) => Ok(x),
            _ => Err(Error::InvalidType),
        }
    }

    pub fn into_array(self) -> Result<Vec<Value>, Error> {
        match self {
            Value::Array(x) => Ok(x),
            _ => Err(Error::InvalidType),
        }
    }
}

impl Encode for Value {
    fn encode<'e, E>(&self, encoder: &mut E) -> codable::enc::EncodeResult<'e, E>
    where
        E: codable::enc::Encoder<'e>,
    {
        match self {
            Value::String(x) => x.encode(encoder),
            Value::Number(x) => x.encode(encoder),
            Value::Bool(x) => x.encode(encoder),
            Value::Array(x) => x.encode(encoder),
            Value::Object(x) => x.encode(encoder),
            Value::Null => {
                let mut c = encoder.as_value_container();
                c.encode_null()?;
                Ok(c.finish())
            }
        }
    }
}

impl Decode for Value {
    fn decode<'d, D>(decoder: &mut D) -> DecodeResult<'d, Self, D>
    where
        Self: Sized,
        D: Decoder + 'd,
    {
        if let Ok(mut d) = decoder.as_seq_container() {
            return Ok(Value::Array(d.decode()?));
        }

        if let Ok(_) = decoder.as_container() {
            return Ok(Value::Object(Decode::decode(decoder)?));
        }

        let mut d = decoder.as_value_container()?;

        if let Ok(x) = d.decode_f64() {
            return Ok(Value::Number(x.to_string()));
        }

        if let Ok(x) = d.decode_string() {
            return Ok(Value::String(x));
        }

        if let Ok(x) = d.decode_bool() {
            return Ok(Value::Bool(x));
        }

        Ok(Value::Null)
    }
}

#[cfg(feature = "serde-compat")]
impl From<serde_json::Value> for Value {
    fn from(value: serde_json::Value) -> Self {
        match value {
            serde_json::Value::Null => Value::Null,
            serde_json::Value::Bool(x) => Value::Bool(x),
            serde_json::Value::Number(x) => Value::Number(x.to_string()),
            serde_json::Value::String(x) => Value::String(x),
            serde_json::Value::Array(x) => Value::Array(x.into_iter().map(Into::into).collect()),
            serde_json::Value::Object(x) => {
                Value::Object(x.into_iter().map(|(k, v)| (k, v.into())).collect())
            }
        }
    }
}

#[cfg(feature = "serde-compat")]
impl From<Value> for serde_json::Value {
    fn from(value: Value) -> Self {
        match value {
            Value::Null => serde_json::Value::Null,
            Value::Bool(x) => serde_json::Value::Bool(x),
            Value::Number(x) => {
                serde_json::Value::Number(serde_json::Number::from_str(&x).unwrap())
            }
            Value::String(x) => serde_json::Value::String(x),
            Value::Array(x) => serde_json::Value::Array(x.into_iter().map(Into::into).collect()),
            Value::Object(x) => {
                serde_json::Value::Object(x.into_iter().map(|(k, v)| (k, v.into())).collect())
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic_int() {
        let value = to_value(&32_u8).unwrap();
        println!("{:?}", value);
    }
}
