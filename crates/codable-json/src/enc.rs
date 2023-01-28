use std::borrow::Cow;

use indexmap::IndexMap;

use codable::{
    enc::{self, Encode, Encoder},
    CodingKey, CodingPath, ToCodingKey,
};

use crate::Value;

#[derive(Debug)]
pub enum Error {
    KeyNotFound,
    InvalidType,
}

#[derive(Debug, Clone)]
pub struct JsonEncoder<'a> {
    coding_path: CodingPath<'a, CodingKey>,
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

pub struct KeyedContainer<'a> {
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

impl<'c> enc::KeyedContainer for KeyedContainer<'c> {
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
pub struct ValueContainer<'en> {
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

impl<'c> enc::ValueContainer for ValueContainer<'c> {
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

pub struct SeqContainer<'a> {
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

impl<'c> enc::SeqContainer for SeqContainer<'c> {
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
