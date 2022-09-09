#![feature(generic_associated_types)]

use std::borrow::Cow;

use indexmap::IndexMap;

use codable::{
    enc::{self, Encode, Encoder},
    CodingKey, Cons,
};

#[derive(Debug)]
struct JsonEncoder<'a> {
    coding_path: Cons<'a, CodingKey>,
}

pub fn to_value<T: Encode>(input: &T) -> Result<Value, Error> {
    let encoder = JsonEncoder::<'static>::new(Cons::root(CodingKey::Root));
    input.encode(encoder)
}

// impl JsonEncoder {
//     #[inline]
//     pub(crate) fn new() -> Self {
//         Self {
//             coding_path: Cons::root(CodingKey::Root),
//             value: Default::default(),
//         }
//     }

//     #[inline]
//     fn as_ref<'a>(&self) -> &JsonEncoderRef<'a> {
//         unsafe { std::mem::transmute(self) }
//     }

//     #[inline]
//     fn as_ref_mut<'a>(&mut self) -> &mut JsonEncoderRef<'a> {
//         unsafe { std::mem::transmute(self) }
//     }
// }

impl<'a> JsonEncoder<'a> {
    pub(crate) fn new(coding_path: Cons<'a, CodingKey>) -> Self {
        Self {
            coding_path,
        }
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

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Internal encoder error")]
    Encoder(#[from] enc::Error),
}

struct KeyedContainer<'a> {
    encoder: JsonEncoder<'a>,
    coding_path: Cons<'a, CodingKey>,
    value: IndexMap<String, Value>,
    // _marker: PhantomData<&'a ()>,
}

impl<'a> KeyedContainer<'a> {
    fn new(encoder: JsonEncoder<'a>, coding_path: Cons<'a, CodingKey>) -> KeyedContainer<'a> {
        KeyedContainer {
            encoder,
            coding_path,
            value: Default::default(),
        }
    }
}

impl<'container> enc::KeyedContainer for KeyedContainer<'container> {
    type Value = Value;
    type Error = Error;
    type Encoder<'a> = JsonEncoder<'a> where Self: 'a;

    fn coding_path(&self) -> &Cons<'_, CodingKey> {
        &self.coding_path
    }

    fn encode_u8(&mut self, value: u8, key: &CodingKey) -> Result<(), Self::Error> {
        self.value.insert(
            key.as_str()?.to_string(),
            Value::Number(value.to_string().into()),
        );
        Ok(())
    }

    fn encode_u16(&mut self, value: u16, key: &CodingKey) -> Result<(), Self::Error> {
        self.value.insert(
            key.as_str()?.to_string(),
            Value::Number(value.to_string().into()),
        );
        Ok(())
    }

    fn encode_u32(&mut self, value: u32, key: &CodingKey) -> Result<(), Self::Error> {
        self.value.insert(
            key.as_str()?.to_string(),
            Value::Number(value.to_string().into()),
        );
        Ok(())
    }

    fn encode_u64(&mut self, value: u64, key: &CodingKey) -> Result<(), Self::Error> {
        self.value.insert(
            key.as_str()?.to_string(),
            Value::Number(value.to_string().into()),
        );
        Ok(())
    }

    fn encode_u128(&mut self, value: u128, key: &CodingKey) -> Result<(), Self::Error> {
        self.value.insert(
            key.as_str()?.to_string(),
            Value::Number(value.to_string().into()),
        );
        Ok(())
    }

    fn encode_usize(&mut self, value: usize, key: &CodingKey) -> Result<(), Self::Error> {
        self.value.insert(
            key.as_str()?.to_string(),
            Value::Number(value.to_string().into()),
        );
        Ok(())
    }

    fn encode_i8(&mut self, value: i8, key: &CodingKey) -> Result<(), Self::Error> {
        self.value.insert(
            key.as_str()?.to_string(),
            Value::Number(value.to_string().into()),
        );
        Ok(())
    }

    fn encode_i16(&mut self, value: i16, key: &CodingKey) -> Result<(), Self::Error> {
        self.value.insert(
            key.as_str()?.to_string(),
            Value::Number(value.to_string().into()),
        );
        Ok(())
    }

    fn encode_i32(&mut self, value: i32, key: &CodingKey) -> Result<(), Self::Error> {
        self.value.insert(
            key.as_str()?.to_string(),
            Value::Number(value.to_string().into()),
        );
        Ok(())
    }

    fn encode_i64(&mut self, value: i64, key: &CodingKey) -> Result<(), Self::Error> {
        self.value.insert(
            key.as_str()?.to_string(),
            Value::Number(value.to_string().into()),
        );
        Ok(())
    }

    fn encode_i128(&mut self, value: i128, key: &CodingKey) -> Result<(), Self::Error> {
        self.value.insert(
            key.as_str()?.to_string(),
            Value::Number(value.to_string().into()),
        );
        Ok(())
    }

    fn encode_isize(&mut self, value: isize, key: &CodingKey) -> Result<(), Self::Error> {
        self.value.insert(
            key.as_str()?.to_string(),
            Value::Number(value.to_string().into()),
        );
        Ok(())
    }

    fn encode_str<'x, S: Into<Cow<'x, str>>>(
        &mut self,
        value: S,
        key: &CodingKey,
    ) -> Result<(), Self::Error> {
        let s = match value.into() {
            Cow::Borrowed(x) => x.to_string(),
            Cow::Owned(x) => x,
        };
        self.value
            .insert(key.as_str()?.to_string(), Value::String(s));
        Ok(())
    }

    fn encode_f32(&mut self, value: f32, key: &CodingKey) -> Result<(), Self::Error> {
        self.value
            .insert(key.as_str()?.to_string(), Value::Number(value.to_string()));
        Ok(())
    }

    fn encode_f64(&mut self, value: f64, key: &CodingKey) -> Result<(), Self::Error> {
        self.value
            .insert(key.as_str()?.to_string(), Value::Number(value.to_string()));
        Ok(())
    }

    fn encode_bool(&mut self, value: bool, key: &CodingKey) -> Result<(), Self::Error> {
        self.value
            .insert(key.as_str()?.to_string(), Value::Bool(value));
        Ok(())
    }

    fn encode_option<T: Encode>(
        &mut self,
        value: Option<T>,
        key: &CodingKey,
    ) -> Result<(), Self::Error> {
        match value {
            Some(x) => self.encode(&x, key),
            None => {
                self.value.insert(key.as_str()?.to_string(), Value::Null);
                Ok(())
            }
        }
    }

    fn encode<'a, T: Encode>(&'a mut self, value: &T, key: &CodingKey) -> Result<(), Self::Error> {
        let coding_path = self.coding_path.join(key.clone());
        let key = key.as_str()?.to_string();
        let encoder = JsonEncoder::<'a>::new(coding_path);
        let value = value.encode(encoder)?;
        self.value.insert(key, value);
        Ok(())
    }

    fn opt_encode_u8(&mut self, value: Option<u8>, key: &CodingKey) -> Result<(), Self::Error> {
        if let Some(value) = value {
            self.encode_u8(value, key)
        } else {
            Ok(())
        }
    }

    fn opt_encode_u16(&mut self, value: Option<u16>, key: &CodingKey) -> Result<(), Self::Error> {
        if let Some(value) = value {
            self.encode_u16(value, key)
        } else {
            Ok(())
        }
    }

    fn opt_encode_u32(&mut self, value: Option<u32>, key: &CodingKey) -> Result<(), Self::Error> {
        if let Some(value) = value {
            self.encode_u32(value, key)
        } else {
            Ok(())
        }
    }

    fn opt_encode_u64(&mut self, value: Option<u64>, key: &CodingKey) -> Result<(), Self::Error> {
        if let Some(value) = value {
            self.encode_u64(value, key)
        } else {
            Ok(())
        }
    }

    fn opt_encode_u128(&mut self, value: Option<u128>, key: &CodingKey) -> Result<(), Self::Error> {
        if let Some(value) = value {
            self.encode_u128(value, key)
        } else {
            Ok(())
        }
    }

    fn opt_encode_usize(
        &mut self,
        value: Option<usize>,
        key: &CodingKey,
    ) -> Result<(), Self::Error> {
        if let Some(value) = value {
            self.encode_usize(value, key)
        } else {
            Ok(())
        }
    }

    fn opt_encode_i8(&mut self, value: Option<i8>, key: &CodingKey) -> Result<(), Self::Error> {
        if let Some(value) = value {
            self.encode_i8(value, key)
        } else {
            Ok(())
        }
    }

    fn opt_encode_i16(&mut self, value: Option<i16>, key: &CodingKey) -> Result<(), Self::Error> {
        if let Some(value) = value {
            self.encode_i16(value, key)
        } else {
            Ok(())
        }
    }

    fn opt_encode_i32(&mut self, value: Option<i32>, key: &CodingKey) -> Result<(), Self::Error> {
        if let Some(value) = value {
            self.encode_i32(value, key)
        } else {
            Ok(())
        }
    }

    fn opt_encode_i64(&mut self, value: Option<i64>, key: &CodingKey) -> Result<(), Self::Error> {
        if let Some(value) = value {
            self.encode_i64(value, key)
        } else {
            Ok(())
        }
    }

    fn opt_encode_i128(&mut self, value: Option<i128>, key: &CodingKey) -> Result<(), Self::Error> {
        if let Some(value) = value {
            self.encode_i128(value, key)
        } else {
            Ok(())
        }
    }

    fn opt_encode_isize(
        &mut self,
        value: Option<isize>,
        key: &CodingKey,
    ) -> Result<(), Self::Error> {
        if let Some(value) = value {
            self.encode_isize(value, key)
        } else {
            Ok(())
        }
    }

    fn opt_encode_str(&mut self, value: Option<&str>, key: &CodingKey) -> Result<(), Self::Error> {
        if let Some(value) = value {
            self.encode_str(value, key)
        } else {
            Ok(())
        }
    }

    fn opt_encode_f32(&mut self, value: Option<f32>, key: &CodingKey) -> Result<(), Self::Error> {
        if let Some(value) = value {
            self.encode_f32(value, key)
        } else {
            Ok(())
        }
    }

    fn opt_encode_f64(&mut self, value: Option<f64>, key: &CodingKey) -> Result<(), Self::Error> {
        if let Some(value) = value {
            self.encode_f64(value, key)
        } else {
            Ok(())
        }
    }

    fn nested_container<'a>(
        &'a mut self,
        key: &CodingKey,
    ) -> Result<<Self::Encoder<'a> as Encoder>::KeyedContainer, Self::Error> {
        let p = self.coding_path().join(key.clone());
        let encoder = JsonEncoder::new(p);
        Ok(encoder.into_container())
    }

    fn nested_seq_container<'a>(
        &'a mut self,
        key: &CodingKey,
    ) -> Result<<Self::Encoder<'a> as Encoder>::SeqContainer, Self::Error> {
        todo!()
    }

    fn finish(self) -> Self::Value {
        Value::Object(self.value)
    }
}

#[derive(Debug)]
struct ValueContainer<'en> {
    encoder: JsonEncoder<'en>,
    coding_path: Cons<'en, CodingKey>,
    value: Option<Value>,
}

impl<'en> ValueContainer<'en> {
    pub fn new(encoder: JsonEncoder<'en>, coding_path: Cons<'en, CodingKey>) -> Self {
        Self {
            encoder,
            coding_path,
            value: None,
        }
    }
}

impl<'container> enc::ValueContainer for ValueContainer<'container> {
    type Value = Value;
    type Error = Error;
    type Encoder<'a> = JsonEncoder<'a>;

    fn coding_path(&self) -> &Cons<'_, CodingKey> {
        &self.coding_path
    }

    fn encode_u8(&mut self, value: u8) -> Result<(), Self::Error> {
        println!("WOOT {:?}", &self.coding_path());
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

    fn encode_option<T>(&mut self, value: Option<T>) -> Result<(), Self::Error> {
        todo!()
    }

    fn encode<T>(&mut self, value: T) -> Result<(), Self::Error> {
        todo!()
    }

    fn encode_bool(&mut self, value: bool) -> Result<(), Self::Error> {
        self.value = Some(Value::Bool(value));
        Ok(())
    }

    fn finish(self) -> Self::Value {
        self.value.unwrap_or(Value::Null)
    }
}

struct SeqContainer<'a> {
    coding_path: Cons<'a, CodingKey>,
    values: Vec<Value>,
}

impl<'container> enc::SeqContainer for SeqContainer<'container> {
    type Error = Error;
    type Value = Value;
    type Encoder<'a> = JsonEncoder<'a> where Self: 'a;

    fn coding_path(&self) -> &Cons<'_, CodingKey> {
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

    fn encode_option<T>(&mut self, value: Option<T>) -> Result<(), Self::Error> {
        todo!()
    }

    fn encode<T>(&mut self, value: T) -> Result<(), Self::Error> {
        todo!()
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

}

impl<'r> enc::Encoder<'r> for JsonEncoder<'r> {
    type Error = Error;
    type Value = Value;

    type KeyedContainer = KeyedContainer<'r> where Self: 'r;
    type ValueContainer = ValueContainer<'r> where Self: 'r;
    type SeqContainer = SeqContainer<'r> where Self: 'r;

    fn into_value_container(self) -> Self::ValueContainer {
        let p = self.coding_path.clone();
        ValueContainer::new(self, p)
    }

    fn into_seq_container(self) -> Self::SeqContainer {
        // SeqContainer::new(self.coding_path.clone());
        todo!()
    }

    fn into_container(self) -> Self::KeyedContainer {
        let p = self.coding_path.clone();
        KeyedContainer::new(self, p)
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn basic_int() {
        let value = to_value(&32_u8).unwrap();
        println!("{:?}", value);
    }

    #[test]
    fn basic_obj() {
        let mut input = HashMap::new();
        input.insert("hello", "hi");
        input.insert("interesting", "yes");
        let value = to_value(&input).unwrap();
        println!("{:?}", value);
    }
}
