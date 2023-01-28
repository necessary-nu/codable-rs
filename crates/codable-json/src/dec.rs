use std::str::FromStr;

use indexmap::IndexMap;

use codable::{
    dec::{self, Decode, Decoder},
    CodingKey, CodingPath, ToCodingKey,
};

use crate::Value;

#[derive(Debug)]
pub enum Error {
    KeyNotFound,
    InvalidType,
}

#[derive(Debug, Clone)]
pub struct JsonDecoder<'a> {
    coding_path: CodingPath<'a, CodingKey<'a>>,
    value: &'a Value,
}

impl<'a> JsonDecoder<'a> {
    pub(crate) fn new(coding_path: CodingPath<'a, CodingKey<'a>>, value: &'a Value) -> Self {
        Self { coding_path, value }
    }
}

pub struct KeyedContainer<'a> {
    coding_path: CodingPath<'a, CodingKey<'a>>,
    value: &'a IndexMap<String, Value>,
}

pub struct ValueContainer<'a> {
    coding_path: CodingPath<'a, CodingKey<'a>>,
    value: &'a Value,
}

pub struct SeqContainer<'a> {
    coding_path: CodingPath<'a, CodingKey<'a>>,
    value: &'a Vec<Value>,
    cursor_index: usize,
}

impl<'a> KeyedContainer<'a> {
    fn new(coding_path: CodingPath<'a, CodingKey<'a>>, value: &'a IndexMap<String, Value>) -> Self {
        Self { coding_path, value }
    }
}

impl<'a> ValueContainer<'a> {
    fn new(coding_path: CodingPath<'a, CodingKey<'a>>, value: &'a Value) -> Self {
        Self { coding_path, value }
    }
}

impl<'a> SeqContainer<'a> {
    fn new(coding_path: CodingPath<'a, CodingKey<'a>>, value: &'a Vec<Value>) -> Self {
        Self {
            coding_path,
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

    fn coding_path(&self) -> &CodingPath<'_, CodingKey<'_>> {
        &self.coding_path
    }

    fn contains(&self, coding_key: &impl ToCodingKey) -> bool {
        let k = coding_key.as_str();
        self.value.contains_key(&*k)
    }

    type Keys<'a> = indexmap::map::Keys<'a, String, Value>
    where Self: 'a;

    fn keys<'a>(&'a self) -> Self::Keys<'a> {
        self.value.keys()
    }

    fn decode_u8(&mut self, key: &impl ToCodingKey) -> Result<u8, Self::Error> {
        decode_int(self.value, &key.as_str())
    }

    fn decode_u16(&mut self, key: &impl ToCodingKey) -> Result<u16, Self::Error> {
        decode_int(self.value, &key.as_str())
    }

    fn decode_u32(&mut self, key: &impl ToCodingKey) -> Result<u32, Self::Error> {
        decode_int(self.value, &key.as_str())
    }

    fn decode_u64(&mut self, key: &impl ToCodingKey) -> Result<u64, Self::Error> {
        decode_int(self.value, &key.as_str())
    }

    fn decode_u128(&mut self, key: &impl ToCodingKey) -> Result<u128, Self::Error> {
        decode_int(self.value, &key.as_str())
    }

    fn decode_usize(&mut self, key: &impl ToCodingKey) -> Result<usize, Self::Error> {
        decode_int(self.value, &key.as_str())
    }

    fn decode_i8(&mut self, key: &impl ToCodingKey) -> Result<i8, Self::Error> {
        decode_int(self.value, &key.as_str())
    }

    fn decode_i16(&mut self, key: &impl ToCodingKey) -> Result<i16, Self::Error> {
        decode_int(self.value, &key.as_str())
    }

    fn decode_i32(&mut self, key: &impl ToCodingKey) -> Result<i32, Self::Error> {
        decode_int(self.value, &key.as_str())
    }

    fn decode_i64(&mut self, key: &impl ToCodingKey) -> Result<i64, Self::Error> {
        decode_int(self.value, &key.as_str())
    }

    fn decode_i128(&mut self, key: &impl ToCodingKey) -> Result<i128, Self::Error> {
        decode_int(self.value, &key.as_str())
    }

    fn decode_isize(&mut self, key: &impl ToCodingKey) -> Result<isize, Self::Error> {
        decode_int(self.value, &key.as_str())
    }

    fn decode_string(&mut self, key: &impl ToCodingKey) -> Result<String, Self::Error> {
        match self.value.get(&*key.as_str()) {
            Some(Value::String(x)) => Ok(x.to_string()),
            Some(_other) => {
                todo!()
            }
            None => {
                todo!()
            }
        }
    }

    fn decode_f32(&mut self, key: &impl ToCodingKey) -> Result<f32, Self::Error> {
        decode_int(self.value, &key.as_str())
    }

    fn decode_f64(&mut self, key: &impl ToCodingKey) -> Result<f64, Self::Error> {
        decode_int(self.value, &key.as_str())
    }

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

    fn decode_option<T: Decode>(
        &mut self,
        key: &impl ToCodingKey,
    ) -> Result<Option<T>, Self::Error> {
        match self.decode(key) {
            Ok(x) => Ok(Some(x)),
            Err(Error::KeyNotFound) => Ok(None),
            Err(x) => Err(x),
        }
    }

    fn decode<T: Decode>(&mut self, key: &impl ToCodingKey) -> Result<T, Self::Error> {
        let obj = self
            .value
            .get(&*key.as_str())
            .ok_or_else(|| Error::KeyNotFound)?;
        T::decode(&mut JsonDecoder::new(self.coding_path().clone(), obj))
    }

    fn nested_container<'a>(
        &'a mut self,
        _key: &impl ToCodingKey,
    ) -> Result<<Self::Decoder as dec::Decoder>::KeyedContainer, Self::Error> {
        todo!()
    }

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

    fn coding_path(&self) -> &CodingPath<'_, CodingKey<'_>> {
        &self.coding_path
    }

    fn decode_u8(&mut self) -> Result<u8, Self::Error> {
        match self.value {
            Value::Number(x) => Ok(x.parse().unwrap()),
            _ => panic!(),
        }
    }

    fn decode_u16(&mut self) -> Result<u16, Self::Error> {
        match self.value {
            Value::Number(x) => Ok(x.parse().unwrap()),
            _ => panic!(),
        }
    }

    fn decode_u32(&mut self) -> Result<u32, Self::Error> {
        match self.value {
            Value::Number(x) => Ok(x.parse().unwrap()),
            _ => panic!(),
        }
    }

    fn decode_u64(&mut self) -> Result<u64, Self::Error> {
        match self.value {
            Value::Number(x) => Ok(x.parse().unwrap()),
            _ => panic!(),
        }
    }

    fn decode_u128(&mut self) -> Result<u128, Self::Error> {
        match self.value {
            Value::Number(x) => Ok(x.parse().unwrap()),
            _ => panic!(),
        }
    }

    fn decode_usize(&mut self) -> Result<usize, Self::Error> {
        match self.value {
            Value::Number(x) => Ok(x.parse().unwrap()),
            _ => panic!(),
        }
    }

    fn decode_i8(&mut self) -> Result<i8, Self::Error> {
        match self.value {
            Value::Number(x) => Ok(x.parse().unwrap()),
            _ => panic!(),
        }
    }

    fn decode_i16(&mut self) -> Result<i16, Self::Error> {
        match self.value {
            Value::Number(x) => Ok(x.parse().unwrap()),
            _ => panic!(),
        }
    }

    fn decode_i32(&mut self) -> Result<i32, Self::Error> {
        match self.value {
            Value::Number(x) => Ok(x.parse().unwrap()),
            _ => panic!(),
        }
    }

    fn decode_i64(&mut self) -> Result<i64, Self::Error> {
        match self.value {
            Value::Number(x) => Ok(x.parse().unwrap()),
            _ => panic!(),
        }
    }

    fn decode_i128(&mut self) -> Result<i128, Self::Error> {
        match self.value {
            Value::Number(x) => Ok(x.parse().unwrap()),
            _ => panic!(),
        }
    }

    fn decode_isize(&mut self) -> Result<isize, Self::Error> {
        match self.value {
            Value::Number(x) => Ok(x.parse().unwrap()),
            _ => panic!(),
        }
    }

    fn decode_string(&mut self) -> Result<String, Self::Error> {
        match self.value {
            Value::String(x) => Ok(x.to_string()),
            _ => Err(Error::InvalidType),
        }
    }

    fn decode_f32(&mut self) -> Result<f32, Self::Error> {
        match self.value {
            Value::Number(x) => Ok(x.parse().unwrap()),
            _ => panic!(),
        }
    }

    fn decode_f64(&mut self) -> Result<f64, Self::Error> {
        match self.value {
            Value::Number(x) => Ok(x.parse().unwrap()),
            _ => Err(Error::InvalidType),
        }
    }

    fn decode_bool(&mut self) -> Result<bool, Self::Error> {
        match self.value {
            Value::Bool(x) => Ok(*x),
            _ => panic!(),
        }
    }

    fn decode_option<T: Decode>(&mut self) -> Result<Option<T>, Self::Error> {
        todo!()
    }

    fn decode<T: Decode>(&mut self) -> Result<T, Self::Error> {
        T::decode(&mut JsonDecoder::new(
            self.coding_path().clone(),
            self.value,
        ))
    }

    fn decode_null(&mut self) -> Result<(), Self::Error> {
        todo!()
    }
}

impl<'c> dec::SeqContainer for SeqContainer<'c> {
    type Error = Error;
    type Value = Value;

    type Decoder = JsonDecoder<'c>;

    fn len(&self) -> usize {
        self.value.len()
    }

    fn cursor_index(&self) -> usize {
        self.cursor_index
    }

    fn coding_path(&self) -> &CodingPath<'_, CodingKey<'_>> {
        &self.coding_path
    }

    fn decode_u8(&mut self) -> Result<u8, Self::Error> {
        let result = match &self.value[self.cursor_index] {
            Value::Number(x) => x.parse().unwrap(),
            _ => return Err(Error::InvalidType),
        };
        self.cursor_index += 1;
        Ok(result)
    }

    fn decode_u16(&mut self) -> Result<u16, Self::Error> {
        let result = match &self.value[self.cursor_index] {
            Value::Number(x) => x.parse().unwrap(),
            _ => return Err(Error::InvalidType),
        };
        self.cursor_index += 1;
        Ok(result)
    }

    fn decode_u32(&mut self) -> Result<u32, Self::Error> {
        let result = match &self.value[self.cursor_index] {
            Value::Number(x) => x.parse().unwrap(),
            _ => return Err(Error::InvalidType),
        };
        self.cursor_index += 1;
        Ok(result)
    }

    fn decode_u64(&mut self) -> Result<u64, Self::Error> {
        let result = match &self.value[self.cursor_index] {
            Value::Number(x) => x.parse().unwrap(),
            _ => return Err(Error::InvalidType),
        };
        self.cursor_index += 1;
        Ok(result)
    }

    fn decode_u128(&mut self) -> Result<u128, Self::Error> {
        let result = match &self.value[self.cursor_index] {
            Value::Number(x) => x.parse().unwrap(),
            _ => return Err(Error::InvalidType),
        };
        self.cursor_index += 1;
        Ok(result)
    }

    fn decode_usize(&mut self) -> Result<usize, Self::Error> {
        let result = match &self.value[self.cursor_index] {
            Value::Number(x) => x.parse().unwrap(),
            _ => return Err(Error::InvalidType),
        };
        self.cursor_index += 1;
        Ok(result)
    }

    fn decode_i8(&mut self) -> Result<i8, Self::Error> {
        let result = match &self.value[self.cursor_index] {
            Value::Number(x) => x.parse().unwrap(),
            _ => return Err(Error::InvalidType),
        };
        self.cursor_index += 1;
        Ok(result)
    }

    fn decode_i16(&mut self) -> Result<i16, Self::Error> {
        let result = match &self.value[self.cursor_index] {
            Value::Number(x) => x.parse().unwrap(),
            _ => return Err(Error::InvalidType),
        };
        self.cursor_index += 1;
        Ok(result)
    }

    fn decode_i32(&mut self) -> Result<i32, Self::Error> {
        let result = match &self.value[self.cursor_index] {
            Value::Number(x) => x.parse().unwrap(),
            _ => return Err(Error::InvalidType),
        };
        self.cursor_index += 1;
        Ok(result)
    }

    fn decode_i64(&mut self) -> Result<i64, Self::Error> {
        let result = match &self.value[self.cursor_index] {
            Value::Number(x) => x.parse().unwrap(),
            _ => return Err(Error::InvalidType),
        };
        self.cursor_index += 1;
        Ok(result)
    }

    fn decode_i128(&mut self) -> Result<i128, Self::Error> {
        let result = match &self.value[self.cursor_index] {
            Value::Number(x) => x.parse().unwrap(),
            _ => return Err(Error::InvalidType),
        };
        self.cursor_index += 1;
        Ok(result)
    }

    fn decode_isize(&mut self) -> Result<isize, Self::Error> {
        let result = match &self.value[self.cursor_index] {
            Value::Number(x) => x.parse().unwrap(),
            _ => return Err(Error::InvalidType),
        };
        self.cursor_index += 1;
        Ok(result)
    }

    fn decode_string(&mut self) -> Result<String, Self::Error> {
        let result = match &self.value[self.cursor_index] {
            Value::String(x) => x.to_string(),
            _ => return Err(Error::InvalidType),
        };
        self.cursor_index += 1;
        Ok(result)
    }

    fn decode_f32(&mut self) -> Result<f32, Self::Error> {
        let result = match &self.value[self.cursor_index] {
            Value::Number(x) => x.parse().unwrap(),
            _ => return Err(Error::InvalidType),
        };
        self.cursor_index += 1;
        Ok(result)
    }

    fn decode_f64(&mut self) -> Result<f64, Self::Error> {
        let result = match &self.value[self.cursor_index] {
            Value::Number(x) => x.parse().unwrap(),
            _ => return Err(Error::InvalidType),
        };
        self.cursor_index += 1;
        Ok(result)
    }

    fn decode_bool(&mut self) -> Result<bool, Self::Error> {
        let result = match &self.value[self.cursor_index] {
            Value::Bool(x) => *x,
            _ => return Err(Error::InvalidType),
        };
        self.cursor_index += 1;
        Ok(result)
    }

    fn decode_option<T: Decode>(&mut self) -> Result<Option<T>, Self::Error> {
        match self.decode() {
            Ok(v) => Ok(Some(v)),
            Err(Error::KeyNotFound) => Ok(None),
            Err(e) => Err(e),
        }
    }

    fn decode<T: Decode>(&mut self) -> Result<T, Self::Error> {
        let item = &self.value[self.cursor_index];
        let result = T::decode(&mut JsonDecoder::new(self.coding_path().clone(), item))?;
        self.cursor_index += 1;
        Ok(result)
    }

    fn nested_container<'a>(
        &'a mut self,
    ) -> Result<<Self::Decoder as dec::Decoder>::KeyedContainer, Self::Error> {
        todo!()
    }

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

    fn as_container(&mut self) -> Result<Self::KeyedContainer, Self::Error> {
        let map = self.value.as_map()?;
        Ok(KeyedContainer::new(self.coding_path.clone(), map))
    }

    fn as_value_container(&mut self) -> Result<Self::ValueContainer, Self::Error> {
        if !self.value.is_scalar() {
            return Err(Error::InvalidType);
        }

        Ok(ValueContainer::new(self.coding_path.clone(), self.value))
    }

    fn as_seq_container(&mut self) -> Result<Self::SeqContainer, Self::Error> {
        Ok(SeqContainer::new(
            self.coding_path.clone(),
            self.value.as_array()?,
        ))
    }
}
