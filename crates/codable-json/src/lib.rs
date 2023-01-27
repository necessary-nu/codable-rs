use std::{borrow::Cow, marker::PhantomData, str::FromStr};

use indexmap::IndexMap;

use codable::{
    dec::{self, Decode, Decoder, KeyedContainer as _, SeqContainer as _, ValueContainer as _},
    enc::{self, Encode, Encoder},
    CodingKey, CodingPath, ToCodingKey,
};

#[derive(Debug, Clone)]
struct JsonEncoder<'a> {
    coding_path: CodingPath<'a, CodingKey>,
}

pub fn to_value<T: Encode>(input: &T) -> Result<Value, Error> {
    let encoder = JsonEncoder::with_path(CodingPath::root(CodingKey::Root));
    input.encode(encoder)
}

impl<'a> JsonEncoder<'a> {
    pub fn new() -> Self {
        Self {
            coding_path: CodingPath::root(CodingKey::Root),
        }
    }

    pub(crate) fn with_path(coding_path: CodingPath<'a, CodingKey>) -> Self {
        Self { coding_path }
    }
}

#[derive(Debug, Clone)]
struct JsonDecoder<'a> {
    coding_path: CodingPath<'a, CodingKey>,
    value: &'a Value,
}

pub fn from_value<T: Decode>(input: &Value) -> Result<T, Error> {
    let mut decoder = JsonDecoder::new(CodingPath::root(CodingKey::Root), input);
    T::decode(&mut decoder)
}

impl<'a> JsonDecoder<'a> {
    pub(crate) fn new(coding_path: CodingPath<'a, CodingKey>, value: &'a Value) -> Self {
        Self { coding_path, value }
    }
}

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

impl Value {
    fn is_scalar(&self) -> bool {
        match self {
            Value::Array(_) |
            Value::Object(_) => false,
            _ => true
        }
    }
    fn as_map(&self) -> Result<&IndexMap<String, Value>, Error> {
        match self {
            Value::Object(ref x) => Ok(x),
            _ => Err(Error::InvalidType),
        }
    }

    fn as_array(&self) -> Result<&Vec<Value>, Error> {
        match self {
            Value::Array(ref x) => Ok(x),
            _ => Err(Error::InvalidType),
        }
    }

    fn into_map(self) -> Result<IndexMap<String, Value>, Error> {
        match self {
            Value::Object(x) => Ok(x),
            _ => Err(Error::InvalidType),
        }
    }

    fn into_array(self) -> Result<Vec<Value>, Error> {
        match self {
            Value::Array(x) => Ok(x),
            _ => Err(Error::InvalidType),
        }
    }
}

impl Decode for Value {
    fn decode<'d, D>(decoder: &mut D) -> dec::DecodeResult<'d, Self, D>
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

#[derive(Debug)]
pub enum Error {
    KeyNotFound,
    InvalidType,
}

struct KeyedContainer<'a> {
    coding_path: CodingPath<'a, CodingKey>,
    value: IndexMap<String, Value>,
}

impl<'a> KeyedContainer<'a> {
    fn new(coding_path: CodingPath<'a, CodingKey>) -> Self {
        Self {
            coding_path,
            value: Default::default(),
        }
    }
}

struct KeyedDecodingContainer<'a> {
    coding_path: CodingPath<'a, CodingKey>,
    value: &'a IndexMap<String, Value>,
}

struct ValueDecodingContainer<'a> {
    coding_path: CodingPath<'a, CodingKey>,
    value: &'a Value,
}

struct SeqDecodingContainer<'a> {
    coding_path: CodingPath<'a, CodingKey>,
    value: &'a Vec<Value>,
    cursor_index: usize,
}

impl<'a> KeyedDecodingContainer<'a> {
    fn new(coding_path: CodingPath<'a, CodingKey>, value: &'a IndexMap<String, Value>) -> Self {
        Self { coding_path, value }
    }
}

impl<'a> ValueDecodingContainer<'a> {
    fn new(coding_path: CodingPath<'a, CodingKey>, value: &'a Value) -> Self {
        Self { coding_path, value }
    }
}

impl<'a> SeqDecodingContainer<'a> {
    fn new(coding_path: CodingPath<'a, CodingKey>, value: &'a Vec<Value>) -> Self {
        Self { coding_path, value, cursor_index: 0 }
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
        Some(other) => {
            todo!()
        }
        None => {
            todo!()
        }
    }
}

impl<'c> dec::KeyedContainer for KeyedDecodingContainer<'c> {
    type Error = Error;
    type Value = Value;

    type Decoder = JsonDecoder<'c>;

    fn coding_path(&self) -> &CodingPath<'_, CodingKey> {
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
        println!("EHH? {} {:?}", key.as_str(), self.value);
        match self.value.get(&*key.as_str()) {
            Some(Value::String(x)) => Ok(x.to_string()),
            Some(other) => {
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
            Some(other) => {
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
        println!("{:?} -> {:?}", key.as_str(), &self.value);
        let obj = self
            .value
            .get(&*key.as_str())
            .ok_or_else(|| Error::KeyNotFound)?;
        T::decode(&mut JsonDecoder::new(self.coding_path().clone(), obj))
    }

    fn nested_container<'a>(
        &'a mut self,
        key: &impl ToCodingKey,
    ) -> Result<<Self::Decoder as dec::Decoder>::KeyedContainer, Self::Error> {
        todo!()
    }

    fn nested_seq_container<'a>(
        &'a mut self,
        key: &impl ToCodingKey,
    ) -> Result<<Self::Decoder as dec::Decoder>::SeqContainer, Self::Error> {
        todo!()
    }
}

impl<'c> dec::ValueContainer for ValueDecodingContainer<'c> {
    type Error = Error;
    type Value = Value;

    type Decoder = JsonDecoder<'c>;

    fn coding_path(&self) -> &CodingPath<'_, CodingKey> {
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
        println!("value itself -> {:?}", &self.value);
        T::decode(&mut JsonDecoder::new(self.coding_path().clone(), self.value))
    }

    fn decode_null(&mut self) -> Result<(), Self::Error> {
        todo!()
    }
}

impl<'c> dec::SeqContainer for SeqDecodingContainer<'c> {
    type Error = Error;
    type Value = Value;

    type Decoder = JsonDecoder<'c>;

    fn len(&self) -> usize {
        self.value.len()
    }

    fn cursor_index(&self) -> usize {
        self.cursor_index
    }

    fn coding_path(&self) -> &CodingPath<'_, CodingKey> {
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
            Err(e) => Err(e)
        }
    }

    fn decode<T: Decode>(&mut self) -> Result<T, Self::Error> {
        println!("seq[{}] -> {:?} <{}>", self.cursor_index, &self.value[self.cursor_index], std::any::type_name::<T>());
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

impl<'container> enc::KeyedContainer for KeyedContainer<'container> {
    type Value = Value;
    type Error = Error;
    type Encoder<'a> = JsonEncoder<'a> where Self: 'a;

    fn coding_path(&self) -> &CodingPath<'_, CodingKey> {
        &self.coding_path
    }

    fn encode_u8(&mut self, value: u8, key: &impl ToCodingKey) -> Result<(), Self::Error> {
        self.value.insert(
            key.as_str().to_string(),
            Value::Number(value.to_string().into()),
        );
        Ok(())
    }

    fn encode_u16(&mut self, value: u16, key: &impl ToCodingKey) -> Result<(), Self::Error> {
        self.value.insert(
            key.as_str().to_string(),
            Value::Number(value.to_string().into()),
        );
        Ok(())
    }

    fn encode_u32(&mut self, value: u32, key: &impl ToCodingKey) -> Result<(), Self::Error> {
        self.value.insert(
            key.as_str().to_string(),
            Value::Number(value.to_string().into()),
        );
        Ok(())
    }

    fn encode_u64(&mut self, value: u64, key: &impl ToCodingKey) -> Result<(), Self::Error> {
        self.value.insert(
            key.as_str().to_string(),
            Value::Number(value.to_string().into()),
        );
        Ok(())
    }

    fn encode_u128(&mut self, value: u128, key: &impl ToCodingKey) -> Result<(), Self::Error> {
        self.value.insert(
            key.as_str().to_string(),
            Value::Number(value.to_string().into()),
        );
        Ok(())
    }

    fn encode_usize(&mut self, value: usize, key: &impl ToCodingKey) -> Result<(), Self::Error> {
        self.value.insert(
            key.as_str().to_string(),
            Value::Number(value.to_string().into()),
        );
        Ok(())
    }

    fn encode_i8(&mut self, value: i8, key: &impl ToCodingKey) -> Result<(), Self::Error> {
        self.value.insert(
            key.as_str().to_string(),
            Value::Number(value.to_string().into()),
        );
        Ok(())
    }

    fn encode_i16(&mut self, value: i16, key: &impl ToCodingKey) -> Result<(), Self::Error> {
        self.value.insert(
            key.as_str().to_string(),
            Value::Number(value.to_string().into()),
        );
        Ok(())
    }

    fn encode_i32(&mut self, value: i32, key: &impl ToCodingKey) -> Result<(), Self::Error> {
        self.value.insert(
            key.as_str().to_string(),
            Value::Number(value.to_string().into()),
        );
        Ok(())
    }

    fn encode_i64(&mut self, value: i64, key: &impl ToCodingKey) -> Result<(), Self::Error> {
        self.value.insert(
            key.as_str().to_string(),
            Value::Number(value.to_string().into()),
        );
        Ok(())
    }

    fn encode_i128(&mut self, value: i128, key: &impl ToCodingKey) -> Result<(), Self::Error> {
        self.value.insert(
            key.as_str().to_string(),
            Value::Number(value.to_string().into()),
        );
        Ok(())
    }

    fn encode_isize(&mut self, value: isize, key: &impl ToCodingKey) -> Result<(), Self::Error> {
        self.value.insert(
            key.as_str().to_string(),
            Value::Number(value.to_string().into()),
        );
        Ok(())
    }

    fn encode_str<'x, S: Into<Cow<'x, str>>>(
        &mut self,
        value: S,
        key: &impl ToCodingKey,
    ) -> Result<(), Self::Error> {
        let s = match value.into() {
            Cow::Borrowed(x) => x.to_string(),
            Cow::Owned(x) => x,
        };
        self.value
            .insert(key.as_str().to_string(), Value::String(s));
        Ok(())
    }

    fn encode_f32(&mut self, value: f32, key: &impl ToCodingKey) -> Result<(), Self::Error> {
        self.value
            .insert(key.as_str().to_string(), Value::Number(value.to_string()));
        Ok(())
    }

    fn encode_f64(&mut self, value: f64, key: &impl ToCodingKey) -> Result<(), Self::Error> {
        self.value
            .insert(key.as_str().to_string(), Value::Number(value.to_string()));
        Ok(())
    }

    fn encode_bool(&mut self, value: bool, key: &impl ToCodingKey) -> Result<(), Self::Error> {
        self.value
            .insert(key.as_str().to_string(), Value::Bool(value));
        Ok(())
    }

    fn encode_option<T: Encode>(
        &mut self,
        value: Option<T>,
        key: &impl ToCodingKey,
    ) -> Result<(), Self::Error> {
        match value {
            Some(x) => self.encode(&x, key),
            None => {
                self.value.insert(key.as_str().to_string(), Value::Null);
                Ok(())
            }
        }
    }

    fn encode<'a, T: Encode>(
        &'a mut self,
        value: &T,
        key: &impl ToCodingKey,
    ) -> Result<(), Self::Error> {
        let coding_path = self.coding_path.join(key.to_coding_key());
        let key = key.as_str().to_string();
        let encoder = JsonEncoder::<'a>::with_path(coding_path);
        let value = value.encode(encoder)?;
        self.value.insert(key, value);
        Ok(())
    }

    fn opt_encode_u8(
        &mut self,
        value: Option<u8>,
        key: &impl ToCodingKey,
    ) -> Result<(), Self::Error> {
        if let Some(value) = value {
            self.encode_u8(value, key)
        } else {
            Ok(())
        }
    }

    fn opt_encode_u16(
        &mut self,
        value: Option<u16>,
        key: &impl ToCodingKey,
    ) -> Result<(), Self::Error> {
        if let Some(value) = value {
            self.encode_u16(value, key)
        } else {
            Ok(())
        }
    }

    fn opt_encode_u32(
        &mut self,
        value: Option<u32>,
        key: &impl ToCodingKey,
    ) -> Result<(), Self::Error> {
        if let Some(value) = value {
            self.encode_u32(value, key)
        } else {
            Ok(())
        }
    }

    fn opt_encode_u64(
        &mut self,
        value: Option<u64>,
        key: &impl ToCodingKey,
    ) -> Result<(), Self::Error> {
        if let Some(value) = value {
            self.encode_u64(value, key)
        } else {
            Ok(())
        }
    }

    fn opt_encode_u128(
        &mut self,
        value: Option<u128>,
        key: &impl ToCodingKey,
    ) -> Result<(), Self::Error> {
        if let Some(value) = value {
            self.encode_u128(value, key)
        } else {
            Ok(())
        }
    }

    fn opt_encode_usize(
        &mut self,
        value: Option<usize>,
        key: &impl ToCodingKey,
    ) -> Result<(), Self::Error> {
        if let Some(value) = value {
            self.encode_usize(value, key)
        } else {
            Ok(())
        }
    }

    fn opt_encode_i8(
        &mut self,
        value: Option<i8>,
        key: &impl ToCodingKey,
    ) -> Result<(), Self::Error> {
        if let Some(value) = value {
            self.encode_i8(value, key)
        } else {
            Ok(())
        }
    }

    fn opt_encode_i16(
        &mut self,
        value: Option<i16>,
        key: &impl ToCodingKey,
    ) -> Result<(), Self::Error> {
        if let Some(value) = value {
            self.encode_i16(value, key)
        } else {
            Ok(())
        }
    }

    fn opt_encode_i32(
        &mut self,
        value: Option<i32>,
        key: &impl ToCodingKey,
    ) -> Result<(), Self::Error> {
        if let Some(value) = value {
            self.encode_i32(value, key)
        } else {
            Ok(())
        }
    }

    fn opt_encode_i64(
        &mut self,
        value: Option<i64>,
        key: &impl ToCodingKey,
    ) -> Result<(), Self::Error> {
        if let Some(value) = value {
            self.encode_i64(value, key)
        } else {
            Ok(())
        }
    }

    fn opt_encode_i128(
        &mut self,
        value: Option<i128>,
        key: &impl ToCodingKey,
    ) -> Result<(), Self::Error> {
        if let Some(value) = value {
            self.encode_i128(value, key)
        } else {
            Ok(())
        }
    }

    fn opt_encode_isize(
        &mut self,
        value: Option<isize>,
        key: &impl ToCodingKey,
    ) -> Result<(), Self::Error> {
        if let Some(value) = value {
            self.encode_isize(value, key)
        } else {
            Ok(())
        }
    }

    fn opt_encode_str(
        &mut self,
        value: Option<&str>,
        key: &impl ToCodingKey,
    ) -> Result<(), Self::Error> {
        if let Some(value) = value {
            self.encode_str(value, key)
        } else {
            Ok(())
        }
    }

    fn opt_encode_f32(
        &mut self,
        value: Option<f32>,
        key: &impl ToCodingKey,
    ) -> Result<(), Self::Error> {
        if let Some(value) = value {
            self.encode_f32(value, key)
        } else {
            Ok(())
        }
    }

    fn opt_encode_f64(
        &mut self,
        value: Option<f64>,
        key: &impl ToCodingKey,
    ) -> Result<(), Self::Error> {
        if let Some(value) = value {
            self.encode_f64(value, key)
        } else {
            Ok(())
        }
    }

    fn nested_container<'a>(
        &'a mut self,
        key: &impl ToCodingKey,
    ) -> Result<<Self::Encoder<'a> as Encoder>::KeyedContainer, Self::Error> {
        let p = self.coding_path().join(key.to_coding_key());
        let encoder = JsonEncoder::with_path(p);
        Ok(encoder.into_container())
    }

    fn nested_seq_container<'a>(
        &'a mut self,
        key: &impl ToCodingKey,
    ) -> Result<<Self::Encoder<'a> as Encoder>::SeqContainer, Self::Error> {
        let p = self.coding_path().join(key.to_coding_key());
        let encoder = JsonEncoder::with_path(p);
        Ok(encoder.into_seq_container())
    }

    fn finish(self) -> Self::Value {
        Value::Object(self.value)
    }
}

#[derive(Debug)]
struct ValueContainer<'en> {
    coding_path: CodingPath<'en, CodingKey>,
    value: Option<Value>,
}

impl<'en> ValueContainer<'en> {
    pub fn new(coding_path: CodingPath<'en, CodingKey>) -> Self {
        Self {
            coding_path,
            value: None,
        }
    }
}

impl<'container> enc::ValueContainer for ValueContainer<'container> {
    type Value = Value;
    type Error = Error;
    type Encoder<'a> = JsonEncoder<'a>;

    fn coding_path(&self) -> &CodingPath<'_, CodingKey> {
        &self.coding_path
    }

    fn encode_u8(&mut self, value: u8) -> Result<(), Self::Error> {
        self.value = Some(Value::Number(value.to_string()));
        Ok(())
    }

    fn encode_u16(&mut self, value: u16) -> Result<(), Self::Error> {
        self.value = Some(Value::Number(value.to_string()));
        Ok(())
    }

    fn encode_u32(&mut self, value: u32) -> Result<(), Self::Error> {
        self.value = Some(Value::Number(value.to_string()));
        Ok(())
    }

    fn encode_u64(&mut self, value: u64) -> Result<(), Self::Error> {
        self.value = Some(Value::Number(value.to_string()));
        Ok(())
    }

    fn encode_u128(&mut self, value: u128) -> Result<(), Self::Error> {
        self.value = Some(Value::Number(value.to_string()));
        Ok(())
    }

    fn encode_usize(&mut self, value: usize) -> Result<(), Self::Error> {
        self.value = Some(Value::Number(value.to_string()));
        Ok(())
    }

    fn encode_i8(&mut self, value: i8) -> Result<(), Self::Error> {
        self.value = Some(Value::Number(value.to_string()));
        Ok(())
    }

    fn encode_i16(&mut self, value: i16) -> Result<(), Self::Error> {
        self.value = Some(Value::Number(value.to_string()));
        Ok(())
    }

    fn encode_i32(&mut self, value: i32) -> Result<(), Self::Error> {
        self.value = Some(Value::Number(value.to_string()));
        Ok(())
    }

    fn encode_i64(&mut self, value: i64) -> Result<(), Self::Error> {
        self.value = Some(Value::Number(value.to_string()));
        Ok(())
    }

    fn encode_i128(&mut self, value: i128) -> Result<(), Self::Error> {
        self.value = Some(Value::Number(value.to_string()));
        Ok(())
    }

    fn encode_isize(&mut self, value: isize) -> Result<(), Self::Error> {
        self.value = Some(Value::Number(value.to_string()));
        Ok(())
    }

    fn encode_str(&mut self, value: &str) -> Result<(), Self::Error> {
        self.value = Some(Value::String(value.to_string()));
        Ok(())
    }

    fn encode_f32(&mut self, value: f32) -> Result<(), Self::Error> {
        self.value = Some(Value::Number(value.to_string()));
        Ok(())
    }

    fn encode_f64(&mut self, value: f64) -> Result<(), Self::Error> {
        self.value = Some(Value::Number(value.to_string()));
        Ok(())
    }

    fn encode_bool(&mut self, value: bool) -> Result<(), Self::Error> {
        self.value = Some(Value::Bool(value));
        Ok(())
    }

    fn finish(self) -> Self::Value {
        self.value.unwrap_or(Value::Null)
    }

    fn encode_option<T: Encode>(&mut self, value: Option<T>) -> Result<(), Self::Error> {
        todo!()
    }

    fn encode<T: Encode>(&mut self, value: &T) -> Result<(), Self::Error> {
        let value = value.encode(JsonEncoder::with_path(self.coding_path.clone()))?;
        self.value = Some(value);
        Ok(())
    }
}

struct SeqContainer<'a> {
    coding_path: CodingPath<'a, CodingKey>,
    values: Vec<Value>,
}

impl<'en> SeqContainer<'en> {
    pub fn new(coding_path: CodingPath<'en, CodingKey>) -> Self {
        Self {
            coding_path,
            values: vec![],
        }
    }
}

impl<'container> enc::SeqContainer for SeqContainer<'container> {
    type Error = Error;
    type Value = Value;
    type Encoder<'a> = JsonEncoder<'a> where Self: 'a;

    fn coding_path(&self) -> &CodingPath<'_, CodingKey> {
        &self.coding_path
    }

    fn encode_u8(&mut self, value: u8) -> Result<(), Self::Error> {
        self.values.push(Value::Number(value.to_string()));
        Ok(())
    }

    fn encode_u16(&mut self, value: u16) -> Result<(), Self::Error> {
        self.values.push(Value::Number(value.to_string()));
        Ok(())
    }

    fn encode_u32(&mut self, value: u32) -> Result<(), Self::Error> {
        self.values.push(Value::Number(value.to_string()));
        Ok(())
    }

    fn encode_u64(&mut self, value: u64) -> Result<(), Self::Error> {
        self.values.push(Value::Number(value.to_string()));
        Ok(())
    }

    fn encode_u128(&mut self, value: u128) -> Result<(), Self::Error> {
        self.values.push(Value::Number(value.to_string()));
        Ok(())
    }

    fn encode_usize(&mut self, value: usize) -> Result<(), Self::Error> {
        self.values.push(Value::Number(value.to_string()));
        Ok(())
    }

    fn encode_i8(&mut self, value: i8) -> Result<(), Self::Error> {
        self.values.push(Value::Number(value.to_string()));
        Ok(())
    }

    fn encode_i16(&mut self, value: i16) -> Result<(), Self::Error> {
        self.values.push(Value::Number(value.to_string()));
        Ok(())
    }

    fn encode_i32(&mut self, value: i32) -> Result<(), Self::Error> {
        self.values.push(Value::Number(value.to_string()));
        Ok(())
    }

    fn encode_i64(&mut self, value: i64) -> Result<(), Self::Error> {
        self.values.push(Value::Number(value.to_string()));
        Ok(())
    }

    fn encode_i128(&mut self, value: i128) -> Result<(), Self::Error> {
        self.values.push(Value::Number(value.to_string()));
        Ok(())
    }

    fn encode_isize(&mut self, value: isize) -> Result<(), Self::Error> {
        self.values.push(Value::Number(value.to_string()));
        Ok(())
    }

    fn encode_str(&mut self, value: &str) -> Result<(), Self::Error> {
        self.values.push(Value::String(value.to_string()));
        Ok(())
    }

    fn encode_f32(&mut self, value: f32) -> Result<(), Self::Error> {
        self.values.push(Value::Number(value.to_string()));
        Ok(())
    }

    fn encode_f64(&mut self, value: f64) -> Result<(), Self::Error> {
        self.values.push(Value::Number(value.to_string()));
        Ok(())
    }

    fn encode_bool(&mut self, value: bool) -> Result<(), Self::Error> {
        self.values.push(Value::Bool(value));
        Ok(())
    }

    fn nested_container<'a>(
        &'a mut self,
    ) -> Result<<Self::Encoder<'a> as Encoder>::KeyedContainer, Self::Error> {
        todo!()
    }

    fn nested_seq_container<'a>(
        &'a mut self,
    ) -> Result<<Self::Encoder<'a> as Encoder>::SeqContainer, Self::Error> {
        todo!()
    }

    fn finish(self) -> Self::Value {
        Value::Array(self.values)
    }

    fn encode_option<T: Encode>(&mut self, value: Option<T>) -> Result<(), Self::Error> {
        todo!()
    }

    fn encode<T: Encode>(&mut self, value: &T) -> Result<(), Self::Error> {
        let value = value.encode(JsonEncoder::with_path(self.coding_path.clone()))?;
        self.values.push(value);
        Ok(())
    }
}

impl<'r> dec::Decoder for JsonDecoder<'r> {
    type Value = Value;
    type Error = Error;

    type KeyedContainer = KeyedDecodingContainer<'r> where Self: 'r;
    type ValueContainer = ValueDecodingContainer<'r> where Self: 'r;
    type SeqContainer = SeqDecodingContainer<'r> where Self: 'r;

    fn as_container(&mut self) -> Result<Self::KeyedContainer, Self::Error> {
        let map = self.value.as_map()?;
        Ok(KeyedDecodingContainer::new(self.coding_path.clone(), map))
    }

    fn as_value_container(&mut self) -> Result<Self::ValueContainer, Self::Error> {
        if !self.value.is_scalar() {
            return Err(Error::InvalidType);
        }

        Ok(ValueDecodingContainer::new(
            self.coding_path.clone(),
            self.value,
        ))
    }

    fn as_seq_container(&mut self) -> Result<Self::SeqContainer, Self::Error> {
        Ok(SeqDecodingContainer::new(
            self.coding_path.clone(),
            self.value.as_array()?,
        ))
    }
}

impl<'r> enc::Encoder<'r> for JsonEncoder<'r> {
    type Error = Error;
    type Value = Value;

    type KeyedContainer = KeyedContainer<'r> where Self: 'r;
    type ValueContainer = ValueContainer<'r> where Self: 'r;
    type SeqContainer = SeqContainer<'r> where Self: 'r;

    fn into_value_container(self) -> Self::ValueContainer {
        ValueContainer::new(self.coding_path)
    }

    fn into_seq_container(self) -> Self::SeqContainer {
        SeqContainer::new(self.coding_path)
    }

    fn into_container(self) -> Self::KeyedContainer {
        KeyedContainer::new(self.coding_path)
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

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use codable::{
        dec::{Decode, DecodeResult, Decoder, KeyedContainer as _},
        enc::KeyedContainer as _,
    };

    use super::*;

    #[test]
    fn basic_int() {
        let value = to_value(&32_u8).unwrap();
        println!("{:?}", value);
    }

    #[test]
    fn basic_struct() {
        struct Hmm {
            test: u32,
            a_bool: bool,
            a_str: String,
        }

        impl Encode for Hmm {
            fn encode<'e, E>(&self, encoder: E) -> enc::EncodeResult<'e, E>
            where
                E: Encoder<'e>,
            {
                let mut con = encoder.into_container();
                con.encode(&self.test, &"test")?;
                con.encode(&self.a_bool, &"a_bool")?;
                con.encode(&self.a_str, &"a_str")?;
                Ok(con.finish())
            }
        }

        let hmm = Hmm {
            test: 1238123,
            a_bool: true,
            a_str: "a test string".into(),
        };

        let value = to_value(&hmm).unwrap();
        println!("{:?}", value);
    }

    #[test]
    fn enums() {
        #[derive(Debug)]
        struct ThingA {
            tag: String,
            pew: u32,
        }

        #[derive(Debug)]
        struct ThingB {
            tag: String,
            another: String,
        }

        #[derive(Debug)]
        struct Base {
            tagged: Tagged,
        }

        #[derive(Debug)]
        enum Tagged {
            ThingA(ThingA),
            ThingB(ThingB),
        }

        impl Encode for ThingA {
            fn encode<'e, E>(&self, encoder: E) -> enc::EncodeResult<'e, E>
            where
                E: Encoder<'e>,
            {
                let mut c = encoder.into_container();
                c.encode_str(&self.tag, &"tag")?;
                c.encode_u32(self.pew, &"pew")?;
                Ok(c.finish())
            }
        }

        impl Encode for ThingB {
            fn encode<'e, E>(&self, encoder: E) -> enc::EncodeResult<'e, E>
            where
                E: Encoder<'e>,
            {
                let mut c = encoder.into_container();
                c.encode_str(&self.tag, &"tag")?;
                c.encode_str(&self.another, &"another")?;
                Ok(c.finish())
            }
        }

        impl Encode for Tagged {
            fn encode<'e, E>(&self, encoder: E) -> enc::EncodeResult<'e, E>
            where
                E: Encoder<'e>,
            {
                match self {
                    Tagged::ThingA(a) => a.encode(encoder),
                    Tagged::ThingB(b) => b.encode(encoder),
                }
            }
        }

        impl Encode for Base {
            fn encode<'e, E>(&self, encoder: E) -> enc::EncodeResult<'e, E>
            where
                E: Encoder<'e>,
            {
                let mut c = encoder.into_container();
                c.encode(&self.tagged, &"tagged")?;
                Ok(c.finish())
            }
        }

        impl Decode for ThingA {
            fn decode<'d, D>(decoder: &mut D) -> DecodeResult<'d, Self, D>
            where
                D: Decoder + 'd,
            {
                let mut d = decoder.as_container()?;
                Ok(ThingA {
                    tag: "a".into(),
                    pew: d.decode_u32(&"pew")?,
                })
            }
        }

        impl Decode for ThingB {
            fn decode<'d, D>(decoder: &mut D) -> DecodeResult<'d, Self, D>
            where
                D: Decoder + 'd,
            {
                let mut d = decoder.as_container()?;
                Ok(ThingB {
                    tag: "b".into(),
                    another: d.decode_string(&"another")?,
                })
            }
        }

        impl Decode for Tagged {
            fn decode<'d, D>(decoder: &mut D) -> DecodeResult<'d, Self, D>
            where
                D: Decoder + 'd,
            {
                let mut d = decoder.as_container()?;
                let tag = d.decode_string(&"tag")?;
                drop(d);
                match &*tag {
                    "a" => Ok(Tagged::ThingA(ThingA::decode(decoder)?)),
                    "b" => Ok(Tagged::ThingB(ThingB::decode(decoder)?)),
                    _ => panic!(),
                }
            }
        }

        impl Decode for Base {
            fn decode<'d, D>(decoder: &mut D) -> DecodeResult<'d, Self, D>
            where
                D: Decoder + 'd,
            {
                let mut d = decoder.as_container()?;
                Ok(Base {
                    tagged: d.decode(&"tagged")?,
                })
            }
        }

        let value = to_value(&Base {
            tagged: Tagged::ThingA(ThingA {
                tag: "a".into(),
                pew: 32,
            }),
        })
        .unwrap();

        println!("{:?}", &value);

        let base = from_value::<Base>(&value).unwrap();

        println!("{:?}", &base);
    }

    #[test]
    fn encode_prim() {
        let encoder = JsonEncoder::new();
        assert_eq!(123u8.encode(encoder).unwrap(), Value::Number("123".into()))
    }

    #[test]
    fn basic_obj() {
        enum Hmm {
            String(&'static str),
            Dict(HashMap<&'static str, Hmm>),
            Arr(Vec<Hmm>),
            Number(u32),
        }
        impl Encode for Hmm {
            fn encode<'e, E>(&self, encoder: E) -> enc::EncodeResult<'e, E>
            where
                E: Encoder<'e>,
            {
                match self {
                    Hmm::String(x) => x.encode(encoder),
                    Hmm::Dict(x) => x.encode(encoder),
                    Hmm::Arr(x) => x.encode(encoder),
                    Hmm::Number(x) => x.encode(encoder),
                }
            }
        }
        let mut input = HashMap::new();
        let mut map = HashMap::new();
        map.insert("test", Hmm::String("another"));
        input.insert("hello", Hmm::String("hi"));
        input.insert("interesting", Hmm::String("yes"));
        input.insert("lolwut", Hmm::Dict(map));
        input.insert(
            "hmm",
            Hmm::Arr(vec![
                Hmm::String("no"),
                Hmm::Number(2),
                Hmm::Number(3),
                Hmm::Number(42),
                Hmm::Number(9),
            ]),
        );
        let value = to_value(&input).unwrap();
        println!("{:?}", value);
    }
}
