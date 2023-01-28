use std::collections::{BTreeMap, HashMap};

use indexmap::IndexMap;

use crate::{CodingPath, ToCodingKey};

use super::CodingKey;

pub trait Decoder {
    type Value;
    type Error;

    type KeyedContainer: KeyedContainer<Decoder = Self, Value = Self::Value, Error = Self::Error>;
    type ValueContainer: ValueContainer<Decoder = Self, Value = Self::Value, Error = Self::Error>;
    type SeqContainer: SeqContainer<Decoder = Self, Value = Self::Value, Error = Self::Error>;

    fn as_container(&mut self) -> Result<Self::KeyedContainer, Self::Error>;
    fn as_value_container(&mut self) -> Result<Self::ValueContainer, Self::Error>;
    fn as_seq_container(&mut self) -> Result<Self::SeqContainer, Self::Error>;
}

pub trait KeyedContainer {
    type Error;
    type Value;
    type Keys<'a>: Iterator<Item = &'a String>
    where
        Self: 'a;
    type Decoder: Decoder;

    fn coding_path(&self) -> &CodingPath<'_, CodingKey<'_>>;
    fn contains(&self, coding_key: &impl ToCodingKey) -> bool;
    fn keys<'a>(&'a self) -> Self::Keys<'a>;

    fn decode_u8(&mut self, key: &impl ToCodingKey) -> Result<u8, Self::Error>;
    fn decode_u16(&mut self, key: &impl ToCodingKey) -> Result<u16, Self::Error>;
    fn decode_u32(&mut self, key: &impl ToCodingKey) -> Result<u32, Self::Error>;
    fn decode_u64(&mut self, key: &impl ToCodingKey) -> Result<u64, Self::Error>;
    fn decode_u128(&mut self, key: &impl ToCodingKey) -> Result<u128, Self::Error>;
    fn decode_usize(&mut self, key: &impl ToCodingKey) -> Result<usize, Self::Error>;
    fn decode_i8(&mut self, key: &impl ToCodingKey) -> Result<i8, Self::Error>;
    fn decode_i16(&mut self, key: &impl ToCodingKey) -> Result<i16, Self::Error>;
    fn decode_i32(&mut self, key: &impl ToCodingKey) -> Result<i32, Self::Error>;
    fn decode_i64(&mut self, key: &impl ToCodingKey) -> Result<i64, Self::Error>;
    fn decode_i128(&mut self, key: &impl ToCodingKey) -> Result<i128, Self::Error>;
    fn decode_isize(&mut self, key: &impl ToCodingKey) -> Result<isize, Self::Error>;
    fn decode_string(&mut self, key: &impl ToCodingKey) -> Result<String, Self::Error>;
    fn decode_f32(&mut self, key: &impl ToCodingKey) -> Result<f32, Self::Error>;
    fn decode_f64(&mut self, key: &impl ToCodingKey) -> Result<f64, Self::Error>;
    fn decode_bool(&mut self, key: &impl ToCodingKey) -> Result<bool, Self::Error>;
    fn decode_option<T: Decode>(
        &mut self,
        key: &impl ToCodingKey,
    ) -> Result<Option<T>, Self::Error>;

    fn decode<T: Decode>(&mut self, key: &impl ToCodingKey) -> Result<T, Self::Error>;

    fn nested_container<'a>(
        &'a mut self,
        key: &impl ToCodingKey,
    ) -> Result<<Self::Decoder as Decoder>::KeyedContainer, Self::Error>;

    fn nested_seq_container<'a>(
        &'a mut self,
        key: &impl ToCodingKey,
    ) -> Result<<Self::Decoder as Decoder>::SeqContainer, Self::Error>;

    fn opt_decode_u8(&mut self, key: &impl ToCodingKey) -> Result<Option<u8>, Self::Error> {
        if self.contains(key) {
            Ok(Some(self.decode_u8(key)?))
        } else {
            Ok(None)
        }
    }
    fn opt_decode_u16(&mut self, key: &impl ToCodingKey) -> Result<Option<u16>, Self::Error> {
        if self.contains(key) {
            Ok(Some(self.decode_u16(key)?))
        } else {
            Ok(None)
        }
    }
    fn opt_decode_u32(&mut self, key: &impl ToCodingKey) -> Result<Option<u32>, Self::Error> {
        if self.contains(key) {
            Ok(Some(self.decode_u32(key)?))
        } else {
            Ok(None)
        }
    }
    fn opt_decode_u64(&mut self, key: &impl ToCodingKey) -> Result<Option<u64>, Self::Error> {
        if self.contains(key) {
            Ok(Some(self.decode_u64(key)?))
        } else {
            Ok(None)
        }
    }
    fn opt_decode_u128(&mut self, key: &impl ToCodingKey) -> Result<Option<u128>, Self::Error> {
        if self.contains(key) {
            Ok(Some(self.decode_u128(key)?))
        } else {
            Ok(None)
        }
    }
    fn opt_decode_usize(&mut self, key: &impl ToCodingKey) -> Result<Option<usize>, Self::Error> {
        if self.contains(key) {
            Ok(Some(self.decode_usize(key)?))
        } else {
            Ok(None)
        }
    }
    fn opt_decode_i8(&mut self, key: &impl ToCodingKey) -> Result<Option<i8>, Self::Error> {
        if self.contains(key) {
            Ok(Some(self.decode_i8(key)?))
        } else {
            Ok(None)
        }
    }
    fn opt_decode_i16(&mut self, key: &impl ToCodingKey) -> Result<Option<i16>, Self::Error> {
        if self.contains(key) {
            Ok(Some(self.decode_i16(key)?))
        } else {
            Ok(None)
        }
    }
    fn opt_decode_i32(&mut self, key: &impl ToCodingKey) -> Result<Option<i32>, Self::Error> {
        if self.contains(key) {
            Ok(Some(self.decode_i32(key)?))
        } else {
            Ok(None)
        }
    }
    fn opt_decode_i64(&mut self, key: &impl ToCodingKey) -> Result<Option<i64>, Self::Error> {
        if self.contains(key) {
            Ok(Some(self.decode_i64(key)?))
        } else {
            Ok(None)
        }
    }
    fn opt_decode_i128(&mut self, key: &impl ToCodingKey) -> Result<Option<i128>, Self::Error> {
        if self.contains(key) {
            Ok(Some(self.decode_i128(key)?))
        } else {
            Ok(None)
        }
    }
    fn opt_decode_isize(&mut self, key: &impl ToCodingKey) -> Result<Option<isize>, Self::Error> {
        if self.contains(key) {
            Ok(Some(self.decode_isize(key)?))
        } else {
            Ok(None)
        }
    }
    fn opt_decode_string(&mut self, key: &impl ToCodingKey) -> Result<Option<String>, Self::Error> {
        if self.contains(key) {
            Ok(Some(self.decode_string(key)?))
        } else {
            Ok(None)
        }
    }
    fn opt_decode_f32(&mut self, key: &impl ToCodingKey) -> Result<Option<f32>, Self::Error> {
        if self.contains(key) {
            Ok(Some(self.decode_f32(key)?))
        } else {
            Ok(None)
        }
    }

    fn opt_decode_f64(&mut self, key: &impl ToCodingKey) -> Result<Option<f64>, Self::Error> {
        if self.contains(key) {
            Ok(Some(self.decode_f64(key)?))
        } else {
            Ok(None)
        }
    }
}

pub trait ValueContainer {
    type Error;
    type Value;
    type Decoder: Decoder;

    fn coding_path(&self) -> &CodingPath<'_, CodingKey<'_>>;

    fn decode_u8(&mut self) -> Result<u8, Self::Error>;
    fn decode_u16(&mut self) -> Result<u16, Self::Error>;
    fn decode_u32(&mut self) -> Result<u32, Self::Error>;
    fn decode_u64(&mut self) -> Result<u64, Self::Error>;
    fn decode_u128(&mut self) -> Result<u128, Self::Error>;
    fn decode_usize(&mut self) -> Result<usize, Self::Error>;
    fn decode_i8(&mut self) -> Result<i8, Self::Error>;
    fn decode_i16(&mut self) -> Result<i16, Self::Error>;
    fn decode_i32(&mut self) -> Result<i32, Self::Error>;
    fn decode_i64(&mut self) -> Result<i64, Self::Error>;
    fn decode_i128(&mut self) -> Result<i128, Self::Error>;
    fn decode_isize(&mut self) -> Result<isize, Self::Error>;
    fn decode_string(&mut self) -> Result<String, Self::Error>;
    fn decode_f32(&mut self) -> Result<f32, Self::Error>;
    fn decode_f64(&mut self) -> Result<f64, Self::Error>;
    fn decode_bool(&mut self) -> Result<bool, Self::Error>;
    fn decode_null(&mut self) -> Result<(), Self::Error>;
    fn decode_option<T: Decode>(&mut self) -> Result<Option<T>, Self::Error>;
    fn decode<T: Decode>(&mut self) -> Result<T, Self::Error>;
}

pub trait SeqContainer {
    type Error;
    type Value;
    type Decoder: Decoder;

    fn coding_path(&self) -> &CodingPath<'_, CodingKey<'_>>;
    fn len(&self) -> usize;
    fn cursor_index(&self) -> usize;

    fn decode_u8(&mut self) -> Result<u8, Self::Error>;
    fn decode_u16(&mut self) -> Result<u16, Self::Error>;
    fn decode_u32(&mut self) -> Result<u32, Self::Error>;
    fn decode_u64(&mut self) -> Result<u64, Self::Error>;
    fn decode_u128(&mut self) -> Result<u128, Self::Error>;
    fn decode_usize(&mut self) -> Result<usize, Self::Error>;
    fn decode_i8(&mut self) -> Result<i8, Self::Error>;
    fn decode_i16(&mut self) -> Result<i16, Self::Error>;
    fn decode_i32(&mut self) -> Result<i32, Self::Error>;
    fn decode_i64(&mut self) -> Result<i64, Self::Error>;
    fn decode_i128(&mut self) -> Result<i128, Self::Error>;
    fn decode_isize(&mut self) -> Result<isize, Self::Error>;
    fn decode_string(&mut self) -> Result<String, Self::Error>;
    fn decode_f32(&mut self) -> Result<f32, Self::Error>;
    fn decode_f64(&mut self) -> Result<f64, Self::Error>;
    fn decode_bool(&mut self) -> Result<bool, Self::Error>;
    fn decode_option<T: Decode>(&mut self) -> Result<Option<T>, Self::Error>;
    fn decode<T: Decode>(&mut self) -> Result<T, Self::Error>;

    fn nested_container<'a>(
        &'a mut self,
    ) -> Result<<Self::Decoder as Decoder>::KeyedContainer, Self::Error>;

    fn nested_seq_container<'a>(
        &'a mut self,
    ) -> Result<<Self::Decoder as Decoder>::SeqContainer, Self::Error>;
}

pub type DecodeResult<'d, T, D> = Result<T, <D as Decoder>::Error>;

pub trait Decode {
    fn decode<'d, D>(decoder: &mut D) -> DecodeResult<'d, Self, D>
    where
        Self: Sized,
        D: Decoder + 'd;
}

macro_rules! decode_prim {
    ($ty:ident, $func:ident) => {
        impl Decode for $ty {
            fn decode<'d, D>(decoder: &mut D) -> DecodeResult<'d, Self, D>
            where
                D: Decoder,
            {
                let mut con = decoder.as_value_container()?;
                con.$func()
            }
        }
    };
}

decode_prim!(u8, decode_u8);
decode_prim!(u16, decode_u16);
decode_prim!(u32, decode_u32);
decode_prim!(u64, decode_u64);
decode_prim!(u128, decode_u128);
decode_prim!(usize, decode_usize);
decode_prim!(i8, decode_i8);
decode_prim!(i16, decode_i16);
decode_prim!(i32, decode_i32);
decode_prim!(i64, decode_i64);
decode_prim!(i128, decode_i128);
decode_prim!(isize, decode_isize);
decode_prim!(f32, decode_f32);
decode_prim!(f64, decode_f64);
decode_prim!(bool, decode_bool);

impl Decode for String {
    fn decode<'d, D>(decoder: &mut D) -> DecodeResult<'d, Self, D>
    where
        D: Decoder,
    {
        let mut con = decoder.as_value_container()?;
        con.decode_string()
    }
}

macro_rules! decode_map {
    ($ty:ident) => {
        impl<V: Decode> Decode for $ty<String, V> {
            fn decode<'d, D>(decoder: &mut D) -> DecodeResult<'d, Self, D>
            where
                D: Decoder,
            {
                let mut out = Self::new();
                let mut con = decoder.as_container()?;
                let keys = con.keys().cloned().collect::<Vec<_>>();
                for k in keys {
                    out.insert(k.to_string(), con.decode(&k)?);
                }
                Ok(out)
            }
        }
    };
}

decode_map!(HashMap);
decode_map!(BTreeMap);
decode_map!(IndexMap);

impl<T: Decode> Decode for Vec<T> {
    fn decode<'d, D>(decoder: &mut D) -> DecodeResult<'d, Self, D>
    where
        D: Decoder,
    {
        let mut con = decoder.as_seq_container()?;
        let mut out: Vec<T> = vec![];
        while con.cursor_index() != con.len() {
            out.push(con.decode()?);
        }
        Ok(out)
    }
}
