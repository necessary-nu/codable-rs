use std::str::FromStr;

use indexmap::IndexMap;

use codable::{
    dec::{self, CustomError, Decode, Decoder},
    CodingKey, CodingPath, ToCodingKey,
};

use crate::Value;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Key not found")]
    KeyNotFound(String),
    #[error("Invalid type")]
    InvalidType(String),
    #[error("{0}")]
    Custom(String, String),
}

impl CustomError for Error {
    fn custom(coding_path: String, message: String) -> Self {
        Self::Custom(coding_path, message)
    }
}

#[derive(Debug, Clone)]
pub struct JsonDecoder<'a> {
    coding_path: CodingPath<'a>,
    value: &'a Value,
}

impl<'a> JsonDecoder<'a> {
    pub(crate) fn new(coding_path: CodingPath<'a>, value: &'a Value) -> Self {
        Self { coding_path, value }
    }
}

pub struct KeyedContainer<'a> {
    coding_path: CodingPath<'a>,
    value: &'a IndexMap<String, Value>,
}

pub struct ValueContainer<'a> {
    coding_path: CodingPath<'a>,
    value: &'a Value,
}

pub struct SeqContainer<'a> {
    coding_path: CodingPath<'a>,
    value: &'a Vec<Value>,
    cursor_index: usize,
}

impl<'a> KeyedContainer<'a> {
    fn new(coding_path: &CodingPath<'a>, value: &'a IndexMap<String, Value>) -> Self {
        Self {
            coding_path: coding_path.clone(),
            value,
        }
    }
}

impl<'a> ValueContainer<'a> {
    fn new(coding_path: &CodingPath<'a>, value: &'a Value) -> Self {
        Self {
            coding_path: coding_path.clone(),
            value,
        }
    }
}

impl<'a> SeqContainer<'a> {
    fn new(coding_path: &CodingPath<'a>, value: &'a Vec<Value>) -> Self {
        Self {
            coding_path: coding_path.join(CodingKey::Int(0)),
            value,
            cursor_index: 0,
        }
    }
}

#[inline(always)]
fn decode_int<T, E>(value: &IndexMap<String, Value>, key: &str) -> Result<T, E>
where
    T: FromStr,
{
    match value.get(&*key.as_str()) {
        Some(Value::Number(x)) => {
            let n = match x.parse() {
                Ok(v) => v,
                Err(_) => panic!(),
            };
            Ok(n)
        }
        Some(_other) => {
            todo!()
        }
        None => {
            todo!()
        }
    }
}

impl<'c> dec::KeyedContainer for KeyedContainer<'c> {
    type Error = Error;
    type Value = Value;

    type Decoder = JsonDecoder<'c>;
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all, fields(coding_path = ?self.coding_path)))]
    fn coding_path(&self) -> &CodingPath<'_> {
        &self.coding_path
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all, fields(coding_path = ?self.coding_path)))]
    fn contains(&self, coding_key: &impl ToCodingKey) -> bool {
        let k = coding_key.as_str();
        self.value.contains_key(&*k)
    }

    type Keys<'a> = indexmap::map::Keys<'a, String, Value>
    where Self: 'a;
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all, fields(coding_path = ?self.coding_path)))]
    fn keys<'a>(&'a self) -> Self::Keys<'a> {
        self.value.keys()
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all, fields(coding_path = ?self.coding_path)))]
    fn decode_u8(&mut self, key: &impl ToCodingKey) -> Result<u8, Self::Error> {
        decode_int(self.value, &key.as_str())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all, fields(coding_path = ?self.coding_path)))]
    fn decode_u16(&mut self, key: &impl ToCodingKey) -> Result<u16, Self::Error> {
        decode_int(self.value, &key.as_str())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all, fields(coding_path = ?self.coding_path)))]
    fn decode_u32(&mut self, key: &impl ToCodingKey) -> Result<u32, Self::Error> {
        decode_int(self.value, &key.as_str())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all, fields(coding_path = ?self.coding_path)))]
    fn decode_u64(&mut self, key: &impl ToCodingKey) -> Result<u64, Self::Error> {
        decode_int(self.value, &key.as_str())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all, fields(coding_path = ?self.coding_path)))]
    fn decode_u128(&mut self, key: &impl ToCodingKey) -> Result<u128, Self::Error> {
        decode_int(self.value, &key.as_str())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all, fields(coding_path = ?self.coding_path)))]
    fn decode_usize(&mut self, key: &impl ToCodingKey) -> Result<usize, Self::Error> {
        decode_int(self.value, &key.as_str())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all, fields(coding_path = ?self.coding_path)))]
    fn decode_i8(&mut self, key: &impl ToCodingKey) -> Result<i8, Self::Error> {
        decode_int(self.value, &key.as_str())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all, fields(coding_path = ?self.coding_path)))]
    fn decode_i16(&mut self, key: &impl ToCodingKey) -> Result<i16, Self::Error> {
        decode_int(self.value, &key.as_str())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all, fields(coding_path = ?self.coding_path)))]
    fn decode_i32(&mut self, key: &impl ToCodingKey) -> Result<i32, Self::Error> {
        decode_int(self.value, &key.as_str())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all, fields(coding_path = ?self.coding_path)))]
    fn decode_i64(&mut self, key: &impl ToCodingKey) -> Result<i64, Self::Error> {
        decode_int(self.value, &key.as_str())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all, fields(coding_path = ?self.coding_path)))]
    fn decode_i128(&mut self, key: &impl ToCodingKey) -> Result<i128, Self::Error> {
        decode_int(self.value, &key.as_str())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all, fields(coding_path = ?self.coding_path)))]
    fn decode_isize(&mut self, key: &impl ToCodingKey) -> Result<isize, Self::Error> {
        decode_int(self.value, &key.as_str())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all, fields(coding_path = ?self.coding_path)))]
    fn decode_string(&mut self, key: &impl ToCodingKey) -> Result<String, Self::Error> {
        match self.value.get(&*key.as_str()) {
            Some(Value::String(x)) => Ok(x.to_string()),
            Some(Value::Null) => Err(Error::KeyNotFound(self.coding_path.to_string())),
            Some(_unknown) => Err(Error::InvalidType(self.coding_path.to_string())),
            None => Err(Error::KeyNotFound(self.coding_path.to_string())),
        }
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all, fields(coding_path = ?self.coding_path)))]
    fn decode_f32(&mut self, key: &impl ToCodingKey) -> Result<f32, Self::Error> {
        decode_int(self.value, &key.as_str())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all, fields(coding_path = ?self.coding_path)))]
    fn decode_f64(&mut self, key: &impl ToCodingKey) -> Result<f64, Self::Error> {
        decode_int(self.value, &key.as_str())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all, fields(coding_path = ?self.coding_path)))]
    fn decode_bool(&mut self, key: &impl ToCodingKey) -> Result<bool, Self::Error> {
        match self.value.get(&*key.as_str()) {
            Some(Value::Bool(x)) => Ok(*x),
            Some(_other) => {
                todo!()
            }
            None => {
                todo!()
            }
        }
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all, fields(coding_path = ?self.coding_path)))]
    fn decode_option<T: Decode>(
        &mut self,
        key: &impl ToCodingKey,
    ) -> Result<Option<T>, Self::Error> {
        match self.decode(key) {
            Ok(x) => Ok(Some(x)),
            Err(Error::KeyNotFound(_)) => Ok(None),
            Err(x) => Err(x),
        }
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all, fields(coding_path = ?self.coding_path)))]
    fn decode<T: Decode>(&mut self, key: &impl ToCodingKey) -> Result<T, Self::Error> {
        let key = key.to_coding_key();
        let path = self.coding_path.join(key.clone());
        let obj = self
            .value
            .get(&*key.as_str())
            .ok_or_else(|| Error::KeyNotFound(path.to_string()))?;
        T::decode(&mut JsonDecoder::new(path, obj))
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all, fields(coding_path = ?self.coding_path)))]
    fn nested_container<'a>(
        &'a mut self,
        _key: &impl ToCodingKey,
    ) -> Result<<Self::Decoder as dec::Decoder>::KeyedContainer, Self::Error> {
        todo!()
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all, fields(coding_path = ?self.coding_path)))]
    fn nested_seq_container<'a>(
        &'a mut self,
        _key: &impl ToCodingKey,
    ) -> Result<<Self::Decoder as dec::Decoder>::SeqContainer, Self::Error> {
        todo!()
    }
}

impl<'c> dec::ValueContainer for ValueContainer<'c> {
    type Error = Error;
    type Value = Value;

    type Decoder = JsonDecoder<'c>;
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all, fields(coding_path = ?self.coding_path)))]
    fn coding_path(&self) -> &CodingPath<'_> {
        &self.coding_path
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all, fields(coding_path = ?self.coding_path)))]
    fn decode_u8(&mut self) -> Result<u8, Self::Error> {
        match self.value {
            Value::Number(x) => Ok(x.parse().unwrap()),
            Value::Null => Err(Error::KeyNotFound(self.coding_path.to_string())),
            _unknown => Err(Error::InvalidType(self.coding_path.to_string())),
        }
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all, fields(coding_path = ?self.coding_path)))]
    fn decode_u16(&mut self) -> Result<u16, Self::Error> {
        match self.value {
            Value::Number(x) => Ok(x.parse().unwrap()),
            Value::Null => Err(Error::KeyNotFound(self.coding_path.to_string())),
            _unknown => Err(Error::InvalidType(self.coding_path.to_string())),
        }
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all, fields(coding_path = ?self.coding_path)))]
    fn decode_u32(&mut self) -> Result<u32, Self::Error> {
        match self.value {
            Value::Number(x) => Ok(x.parse().unwrap()),
            Value::Null => Err(Error::KeyNotFound(self.coding_path.to_string())),
            _unknown => Err(Error::InvalidType(self.coding_path.to_string())),
        }
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all, fields(coding_path = ?self.coding_path)))]
    fn decode_u64(&mut self) -> Result<u64, Self::Error> {
        match self.value {
            Value::Number(x) => Ok(x.parse().unwrap()),
            Value::Null => Err(Error::KeyNotFound(self.coding_path.to_string())),
            _unknown => Err(Error::InvalidType(self.coding_path.to_string())),
        }
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all, fields(coding_path = ?self.coding_path)))]
    fn decode_u128(&mut self) -> Result<u128, Self::Error> {
        match self.value {
            Value::Number(x) => Ok(x.parse().unwrap()),
            Value::Null => Err(Error::KeyNotFound(self.coding_path.to_string())),
            _unknown => Err(Error::InvalidType(self.coding_path.to_string())),
        }
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all, fields(coding_path = ?self.coding_path)))]
    fn decode_usize(&mut self) -> Result<usize, Self::Error> {
        match self.value {
            Value::Number(x) => Ok(x.parse().unwrap()),
            Value::Null => Err(Error::KeyNotFound(self.coding_path.to_string())),
            _unknown => Err(Error::InvalidType(self.coding_path.to_string())),
        }
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all, fields(coding_path = ?self.coding_path)))]
    fn decode_i8(&mut self) -> Result<i8, Self::Error> {
        match self.value {
            Value::Number(x) => Ok(x.parse().unwrap()),
            Value::Null => Err(Error::KeyNotFound(self.coding_path.to_string())),
            _unknown => Err(Error::InvalidType(self.coding_path.to_string())),
        }
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all, fields(coding_path = ?self.coding_path)))]
    fn decode_i16(&mut self) -> Result<i16, Self::Error> {
        match self.value {
            Value::Number(x) => Ok(x.parse().unwrap()),
            Value::Null => Err(Error::KeyNotFound(self.coding_path.to_string())),
            _unknown => Err(Error::InvalidType(self.coding_path.to_string())),
        }
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all, fields(coding_path = ?self.coding_path)))]
    fn decode_i32(&mut self) -> Result<i32, Self::Error> {
        match self.value {
            Value::Number(x) => Ok(x.parse().unwrap()),
            Value::Null => Err(Error::KeyNotFound(self.coding_path.to_string())),
            _unknown => Err(Error::InvalidType(self.coding_path.to_string())),
        }
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all, fields(coding_path = ?self.coding_path)))]
    fn decode_i64(&mut self) -> Result<i64, Self::Error> {
        match self.value {
            Value::Number(x) => Ok(x.parse().unwrap()),
            Value::Null => Err(Error::KeyNotFound(self.coding_path.to_string())),
            _unknown => Err(Error::InvalidType(self.coding_path.to_string())),
        }
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all, fields(coding_path = ?self.coding_path)))]
    fn decode_i128(&mut self) -> Result<i128, Self::Error> {
        match self.value {
            Value::Number(x) => Ok(x.parse().unwrap()),
            Value::Null => Err(Error::KeyNotFound(self.coding_path.to_string())),
            _unknown => Err(Error::InvalidType(self.coding_path.to_string())),
        }
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all, fields(coding_path = ?self.coding_path)))]
    fn decode_isize(&mut self) -> Result<isize, Self::Error> {
        match self.value {
            Value::Number(x) => Ok(x.parse().unwrap()),
            Value::Null => Err(Error::KeyNotFound(self.coding_path.to_string())),
            _unknown => Err(Error::InvalidType(self.coding_path.to_string())),
        }
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all, fields(coding_path = ?self.coding_path)))]
    fn decode_string(&mut self) -> Result<String, Self::Error> {
        match self.value {
            Value::String(x) => Ok(x.to_string()),
            Value::Null => Err(Error::KeyNotFound(self.coding_path.to_string())),
            _unknown => Err(Error::InvalidType(self.coding_path.to_string())),
        }
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all, fields(coding_path = ?self.coding_path)))]
    fn decode_f32(&mut self) -> Result<f32, Self::Error> {
        match self.value {
            Value::Number(x) => Ok(x.parse().unwrap()),
            Value::Null => Err(Error::KeyNotFound(self.coding_path.to_string())),
            _unknown => Err(Error::InvalidType(self.coding_path.to_string())),
        }
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all, fields(coding_path = ?self.coding_path)))]
    fn decode_f64(&mut self) -> Result<f64, Self::Error> {
        match self.value {
            Value::Number(x) => Ok(x.parse().unwrap()),
            Value::Null => Err(Error::KeyNotFound(self.coding_path.to_string())),
            _unknown => Err(Error::InvalidType(self.coding_path.to_string())),
        }
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all, fields(coding_path = ?self.coding_path)))]
    fn decode_bool(&mut self) -> Result<bool, Self::Error> {
        match self.value {
            Value::Bool(x) => Ok(*x),
            Value::Null => Err(Error::KeyNotFound(self.coding_path.to_string())),
            _unknown => Err(Error::InvalidType(self.coding_path.to_string())),
        }
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all, fields(coding_path = ?self.coding_path)))]
    fn decode_option<T: Decode>(&mut self) -> Result<Option<T>, Self::Error> {
        match self.decode() {
            Ok(v) => Ok(Some(v)),
            Err(Error::KeyNotFound(_)) => Ok(None),
            Err(e) => Err(e),
        }
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all, fields(coding_path = ?self.coding_path)))]
    fn decode<T: Decode>(&mut self) -> Result<T, Self::Error> {
        T::decode(&mut JsonDecoder::new(
            self.coding_path().clone(),
            self.value,
        ))
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all, fields(coding_path = ?self.coding_path)))]
    fn decode_null(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl SeqContainer<'_> {
    #[inline]
    fn increment_cursor(&mut self) {
        self.cursor_index += 1;
        // self.coding_path.join(CodingKey::Int(self.cursor_index));
    }
}

impl<'c> dec::SeqContainer for SeqContainer<'c> {
    type Error = Error;
    type Value = Value;

    type Decoder = JsonDecoder<'c>;
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all, fields(coding_path = ?self.coding_path)))]
    fn len(&self) -> usize {
        self.value.len()
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all, fields(coding_path = ?self.coding_path)))]
    fn cursor_index(&self) -> usize {
        self.cursor_index
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all, fields(coding_path = ?self.coding_path)))]
    fn coding_path(&self) -> &CodingPath<'_> {
        &self.coding_path
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all, fields(coding_path = ?self.coding_path)))]
    fn decode_u8(&mut self) -> Result<u8, Self::Error> {
        let result = match &self.value[self.cursor_index] {
            Value::Number(x) => x.parse().unwrap(),
            Value::Null => return Err(Error::KeyNotFound(self.coding_path.to_string())),
            _unknown => return Err(Error::InvalidType(self.coding_path.to_string())),
        };
        self.increment_cursor();
        Ok(result)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all, fields(coding_path = ?self.coding_path)))]
    fn decode_u16(&mut self) -> Result<u16, Self::Error> {
        let result = match &self.value[self.cursor_index] {
            Value::Number(x) => x.parse().unwrap(),
            Value::Null => return Err(Error::KeyNotFound(self.coding_path.to_string())),
            _unknown => return Err(Error::InvalidType(self.coding_path.to_string())),
        };
        self.increment_cursor();
        Ok(result)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all, fields(coding_path = ?self.coding_path)))]
    fn decode_u32(&mut self) -> Result<u32, Self::Error> {
        let result = match &self.value[self.cursor_index] {
            Value::Number(x) => x.parse().unwrap(),
            Value::Null => return Err(Error::KeyNotFound(self.coding_path.to_string())),
            _unknown => return Err(Error::InvalidType(self.coding_path.to_string())),
        };
        self.increment_cursor();
        Ok(result)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all, fields(coding_path = ?self.coding_path)))]
    fn decode_u64(&mut self) -> Result<u64, Self::Error> {
        let result = match &self.value[self.cursor_index] {
            Value::Number(x) => x.parse().unwrap(),
            Value::Null => return Err(Error::KeyNotFound(self.coding_path.to_string())),
            _unknown => return Err(Error::InvalidType(self.coding_path.to_string())),
        };
        self.increment_cursor();
        Ok(result)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all, fields(coding_path = ?self.coding_path)))]
    fn decode_u128(&mut self) -> Result<u128, Self::Error> {
        let result = match &self.value[self.cursor_index] {
            Value::Number(x) => x.parse().unwrap(),
            Value::Null => return Err(Error::KeyNotFound(self.coding_path.to_string())),
            _unknown => return Err(Error::InvalidType(self.coding_path.to_string())),
        };
        self.increment_cursor();
        Ok(result)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all, fields(coding_path = ?self.coding_path)))]
    fn decode_usize(&mut self) -> Result<usize, Self::Error> {
        let result = match &self.value[self.cursor_index] {
            Value::Number(x) => x.parse().unwrap(),
            Value::Null => return Err(Error::KeyNotFound(self.coding_path.to_string())),
            _unknown => return Err(Error::InvalidType(self.coding_path.to_string())),
        };
        self.increment_cursor();
        Ok(result)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all, fields(coding_path = ?self.coding_path)))]
    fn decode_i8(&mut self) -> Result<i8, Self::Error> {
        let result = match &self.value[self.cursor_index] {
            Value::Number(x) => x.parse().unwrap(),
            Value::Null => return Err(Error::KeyNotFound(self.coding_path.to_string())),
            _unknown => return Err(Error::InvalidType(self.coding_path.to_string())),
        };
        self.increment_cursor();
        Ok(result)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all, fields(coding_path = ?self.coding_path)))]
    fn decode_i16(&mut self) -> Result<i16, Self::Error> {
        let result = match &self.value[self.cursor_index] {
            Value::Number(x) => x.parse().unwrap(),
            Value::Null => return Err(Error::KeyNotFound(self.coding_path.to_string())),
            _unknown => return Err(Error::InvalidType(self.coding_path.to_string())),
        };
        self.increment_cursor();
        Ok(result)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all, fields(coding_path = ?self.coding_path)))]
    fn decode_i32(&mut self) -> Result<i32, Self::Error> {
        let result = match &self.value[self.cursor_index] {
            Value::Number(x) => x.parse().unwrap(),
            Value::Null => return Err(Error::KeyNotFound(self.coding_path.to_string())),
            _unknown => return Err(Error::InvalidType(self.coding_path.to_string())),
        };
        self.increment_cursor();
        Ok(result)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all, fields(coding_path = ?self.coding_path)))]
    fn decode_i64(&mut self) -> Result<i64, Self::Error> {
        let result = match &self.value[self.cursor_index] {
            Value::Number(x) => x.parse().unwrap(),
            Value::Null => return Err(Error::KeyNotFound(self.coding_path.to_string())),
            _unknown => return Err(Error::InvalidType(self.coding_path.to_string())),
        };
        self.increment_cursor();
        Ok(result)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all, fields(coding_path = ?self.coding_path)))]
    fn decode_i128(&mut self) -> Result<i128, Self::Error> {
        let result = match &self.value[self.cursor_index] {
            Value::Number(x) => x.parse().unwrap(),
            Value::Null => return Err(Error::KeyNotFound(self.coding_path.to_string())),
            _unknown => return Err(Error::InvalidType(self.coding_path.to_string())),
        };
        self.increment_cursor();
        Ok(result)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all, fields(coding_path = ?self.coding_path)))]
    fn decode_isize(&mut self) -> Result<isize, Self::Error> {
        let result = match &self.value[self.cursor_index] {
            Value::Number(x) => x.parse().unwrap(),
            Value::Null => return Err(Error::KeyNotFound(self.coding_path.to_string())),
            _unknown => return Err(Error::InvalidType(self.coding_path.to_string())),
        };
        self.increment_cursor();
        Ok(result)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all, fields(coding_path = ?self.coding_path)))]
    fn decode_string(&mut self) -> Result<String, Self::Error> {
        let result = match &self.value[self.cursor_index] {
            Value::String(x) => x.to_string(),
            Value::Null => return Err(Error::KeyNotFound(self.coding_path.to_string())),
            _unknown => return Err(Error::InvalidType(self.coding_path.to_string())),
        };
        self.increment_cursor();
        Ok(result)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all, fields(coding_path = ?self.coding_path)))]
    fn decode_f32(&mut self) -> Result<f32, Self::Error> {
        let result = match &self.value[self.cursor_index] {
            Value::Number(x) => x.parse().unwrap(),
            Value::Null => return Err(Error::KeyNotFound(self.coding_path.to_string())),
            _unknown => return Err(Error::InvalidType(self.coding_path.to_string())),
        };
        self.increment_cursor();
        Ok(result)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all, fields(coding_path = ?self.coding_path)))]
    fn decode_f64(&mut self) -> Result<f64, Self::Error> {
        let result = match &self.value[self.cursor_index] {
            Value::Number(x) => x.parse().unwrap(),
            Value::Null => return Err(Error::KeyNotFound(self.coding_path.to_string())),
            _unknown => return Err(Error::InvalidType(self.coding_path.to_string())),
        };
        self.increment_cursor();
        Ok(result)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all, fields(coding_path = ?self.coding_path)))]
    fn decode_bool(&mut self) -> Result<bool, Self::Error> {
        let result = match &self.value[self.cursor_index] {
            Value::Bool(x) => *x,
            Value::Null => return Err(Error::KeyNotFound(self.coding_path.to_string())),
            _unknown => return Err(Error::InvalidType(self.coding_path.to_string())),
        };
        self.increment_cursor();
        Ok(result)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all, fields(coding_path = ?self.coding_path)))]
    fn decode_option<T: Decode>(&mut self) -> Result<Option<T>, Self::Error> {
        match self.decode() {
            Ok(v) => Ok(Some(v)),
            Err(Error::KeyNotFound(_)) => Ok(None),
            Err(e) => Err(e),
        }
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all, fields(coding_path = ?self.coding_path)))]
    fn decode<T: Decode>(&mut self) -> Result<T, Self::Error> {
        let item = &self.value[self.cursor_index];
        let result = T::decode(&mut JsonDecoder::new(self.coding_path().clone(), item))?;
        self.increment_cursor();
        Ok(result)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all, fields(coding_path = ?self.coding_path)))]
    fn nested_container<'a>(
        &'a mut self,
    ) -> Result<<Self::Decoder as dec::Decoder>::KeyedContainer, Self::Error> {
        todo!()
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all, fields(coding_path = ?self.coding_path)))]
    fn nested_seq_container<'a>(
        &'a mut self,
    ) -> Result<<Self::Decoder as dec::Decoder>::SeqContainer, Self::Error> {
        todo!()
    }
}

impl<'r> Decoder for JsonDecoder<'r> {
    type Value = Value;
    type Error = Error;

    type KeyedContainer = KeyedContainer<'r> where Self: 'r;
    type ValueContainer = ValueContainer<'r> where Self: 'r;
    type SeqContainer = SeqContainer<'r> where Self: 'r;

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all, fields(coding_path = ?self.coding_path)))]
    fn as_container(&mut self) -> Result<Self::KeyedContainer, Self::Error> {
        let map = self.value.as_map(&self.coding_path)?;
        Ok(KeyedContainer::new(&self.coding_path, map))
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all, fields(coding_path = ?self.coding_path)))]
    fn as_value_container(&mut self) -> Result<Self::ValueContainer, Self::Error> {
        if !self.value.is_scalar() {
            return Err(Error::InvalidType(self.coding_path.to_string()));
        }

        Ok(ValueContainer::new(&self.coding_path, self.value))
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all, fields(coding_path = ?self.coding_path)))]
    fn as_seq_container(&mut self) -> Result<Self::SeqContainer, Self::Error> {
        Ok(SeqContainer::new(
            &self.coding_path,
            self.value.as_array(&self.coding_path)?,
        ))
    }
}
