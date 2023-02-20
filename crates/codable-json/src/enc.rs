use std::borrow::Cow;

use indexmap::IndexMap;

use codable::{
    enc::{self, Encode, Encoder},
    CodingPath, ToCodingKey,
};

use crate::Value;

#[derive(Debug)]
pub enum Error {
    KeyNotFound,
    InvalidType,
}

#[derive(Debug, Clone)]
pub struct JsonEncoder<'a> {
    coding_path: CodingPath<'a>,
}

impl<'a> JsonEncoder<'a> {
    pub fn new() -> Self {
        Self {
            coding_path: CodingPath::root(),
        }
    }

    pub(crate) fn with_path(coding_path: CodingPath<'a>) -> Self {
        Self { coding_path }
    }
}

impl<'r> enc::Encoder<'r> for JsonEncoder<'r> {
    type Error = Error;
    type Value = Value;

    type KeyedContainer = KeyedContainer<'r> where Self: 'r;
    type ValueContainer = ValueContainer<'r> where Self: 'r;
    type SeqContainer = SeqContainer<'r> where Self: 'r;

    fn as_value_container(&mut self) -> Self::ValueContainer {
        ValueContainer::new(self.coding_path.clone())
    }

    fn as_seq_container(&mut self) -> Self::SeqContainer {
        SeqContainer::new(self.coding_path.clone())
    }

    fn as_container(&mut self) -> Self::KeyedContainer {
        KeyedContainer::new(self.coding_path.clone())
    }
}

pub struct KeyedContainer<'a> {
    coding_path: CodingPath<'a>,
    value: IndexMap<String, Value>,
}

impl<'a> KeyedContainer<'a> {
    fn new(coding_path: CodingPath<'a>) -> Self {
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

    fn coding_path(&self) -> &CodingPath<'_> {
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
        value: Option<&T>,
        key: &impl ToCodingKey,
    ) -> Result<(), Self::Error> {
        match value {
            Some(x) => self.encode(x, key),
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
        let mut encoder = JsonEncoder::with_path(coding_path);
        let value = value.encode(&mut encoder)?;
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
        key: &'a impl ToCodingKey,
    ) -> Result<<Self::Encoder<'a> as Encoder<'a>>::KeyedContainer, Self::Error> {
        let p = self.coding_path().join(key.to_coding_key());
        let mut encoder = JsonEncoder::with_path(p);
        Ok(encoder.as_container())
    }

    fn nested_seq_container<'a>(
        &'a mut self,
        key: &'a impl ToCodingKey,
    ) -> Result<<Self::Encoder<'a> as Encoder<'a>>::SeqContainer, Self::Error> {
        let p = self.coding_path().join(key.to_coding_key());
        let mut encoder = JsonEncoder::with_path(p);
        Ok(encoder.as_seq_container())
    }

    fn finish(self) -> Self::Value {
        Value::Object(self.value)
    }
}

#[derive(Debug)]
pub struct ValueContainer<'en> {
    coding_path: CodingPath<'en>,
    value: Option<Value>,
}

impl<'en> ValueContainer<'en> {
    pub fn new(coding_path: CodingPath<'en>) -> Self {
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

    fn coding_path(&self) -> &CodingPath<'_> {
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

    fn encode_null(&mut self) -> Result<(), Self::Error> {
        self.value = Some(Value::Null);
        Ok(())
    }

    fn encode_option<T: Encode>(&mut self, value: Option<&T>) -> Result<(), Self::Error> {
        let value = match value {
            Some(v) => v.encode(&mut JsonEncoder::with_path(self.coding_path.clone()))?,
            None => Value::Null,
        };
        self.value = Some(value);
        Ok(())
    }

    fn encode<T: Encode>(&mut self, value: &T) -> Result<(), Self::Error> {
        let value = value.encode(&mut JsonEncoder::with_path(self.coding_path.clone()))?;
        self.value = Some(value);
        Ok(())
    }

    fn finish(self) -> Self::Value {
        self.value.unwrap_or(Value::Null)
    }
}

pub struct SeqContainer<'a> {
    coding_path: CodingPath<'a>,
    values: Vec<Value>,
}

impl<'en> SeqContainer<'en> {
    pub fn new(coding_path: CodingPath<'en>) -> Self {
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

    fn coding_path(&self) -> &CodingPath<'_> {
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

    fn encode_null(&mut self) -> Result<(), Self::Error> {
        self.values.push(Value::Null);
        Ok(())
    }

    fn encode_option<T: Encode>(&mut self, value: Option<&T>) -> Result<(), Self::Error> {
        let value = match value {
            Some(v) => v.encode(&mut JsonEncoder::with_path(self.coding_path.clone()))?,
            None => Value::Null,
        };
        self.values.push(value);
        Ok(())
    }

    fn encode<T: Encode>(&mut self, value: &T) -> Result<(), Self::Error> {
        let value = value.encode(&mut JsonEncoder::with_path(self.coding_path.clone()))?;
        self.values.push(value);
        Ok(())
    }

    fn nested_container<'a>(
        &'a mut self,
    ) -> Result<<Self::Encoder<'a> as Encoder<'_>>::KeyedContainer, Self::Error> {
        todo!()
    }

    fn nested_seq_container<'a>(
        &'a mut self,
    ) -> Result<<Self::Encoder<'a> as Encoder<'_>>::SeqContainer, Self::Error> {
        todo!()
    }

    fn finish(self) -> Self::Value {
        Value::Array(self.values)
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use codable::{
        dec::{Decode, DecodeResult, Decoder, KeyedContainer},
        enc::KeyedContainer as _,
    };

    use crate::{from_value, to_value};

    use super::*;

    #[test]
    fn basic_struct() {
        struct Hmm {
            test: u32,
            a_bool: bool,
            a_str: String,
        }

        impl Encode for Hmm {
            fn encode<'e, E>(&self, encoder: &mut E) -> enc::EncodeResult<'e, E>
            where
                E: Encoder<'e>,
            {
                let mut con = encoder.as_container();
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
    fn encode_prim() {
        let mut encoder = JsonEncoder::new();
        assert_eq!(
            123u8.encode(&mut encoder).unwrap(),
            Value::Number("123".into())
        )
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
            fn encode<'e, E>(&self, encoder: &mut E) -> enc::EncodeResult<'e, E>
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
            fn encode<'e, E>(&self, encoder: &mut E) -> enc::EncodeResult<'e, E>
            where
                E: Encoder<'e>,
            {
                let mut c = encoder.as_container();
                c.encode_str(&self.tag, &"tag")?;
                c.encode_u32(self.pew, &"pew")?;
                Ok(c.finish())
            }
        }

        impl Encode for ThingB {
            fn encode<'e, E>(&self, encoder: &mut E) -> enc::EncodeResult<'e, E>
            where
                E: Encoder<'e>,
            {
                let mut c = encoder.as_container();
                c.encode_str(&self.tag, &"tag")?;
                c.encode_str(&self.another, &"another")?;
                Ok(c.finish())
            }
        }

        impl Encode for Tagged {
            fn encode<'e, E>(&self, encoder: &mut E) -> enc::EncodeResult<'e, E>
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
            fn encode<'e, E>(&self, encoder: &mut E) -> enc::EncodeResult<'e, E>
            where
                E: Encoder<'e>,
            {
                let mut c = encoder.as_container();
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
}
