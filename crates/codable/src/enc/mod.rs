use std::{
    borrow::Cow,
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
    fmt::Debug,
};

use indexmap::IndexMap;

use crate::{CodingPath, ToCodingKey};

pub trait KeyedContainer {
    type Error;
    type Value;
    type Encoder<'a>: Encoder<'a>
    where
        Self: 'a;

    fn coding_path(&self) -> &CodingPath<'_>;

    fn encode_u8(&mut self, value: u8, key: &impl ToCodingKey) -> Result<(), Self::Error>;
    fn encode_u16(&mut self, value: u16, key: &impl ToCodingKey) -> Result<(), Self::Error>;
    fn encode_u32(&mut self, value: u32, key: &impl ToCodingKey) -> Result<(), Self::Error>;
    fn encode_u64(&mut self, value: u64, key: &impl ToCodingKey) -> Result<(), Self::Error>;
    fn encode_u128(&mut self, value: u128, key: &impl ToCodingKey) -> Result<(), Self::Error>;
    fn encode_usize(&mut self, value: usize, key: &impl ToCodingKey) -> Result<(), Self::Error>;
    fn encode_i8(&mut self, value: i8, key: &impl ToCodingKey) -> Result<(), Self::Error>;
    fn encode_i16(&mut self, value: i16, key: &impl ToCodingKey) -> Result<(), Self::Error>;
    fn encode_i32(&mut self, value: i32, key: &impl ToCodingKey) -> Result<(), Self::Error>;
    fn encode_i64(&mut self, value: i64, key: &impl ToCodingKey) -> Result<(), Self::Error>;
    fn encode_i128(&mut self, value: i128, key: &impl ToCodingKey) -> Result<(), Self::Error>;
    fn encode_isize(&mut self, value: isize, key: &impl ToCodingKey) -> Result<(), Self::Error>;
    fn encode_str<'s, S: Into<Cow<'s, str>>>(
        &mut self,
        value: S,
        key: &impl ToCodingKey,
    ) -> Result<(), Self::Error>;
    fn encode_f32(&mut self, value: f32, key: &impl ToCodingKey) -> Result<(), Self::Error>;
    fn encode_f64(&mut self, value: f64, key: &impl ToCodingKey) -> Result<(), Self::Error>;
    fn encode_bool(&mut self, value: bool, key: &impl ToCodingKey) -> Result<(), Self::Error>;
    fn encode_option<T: Encode>(
        &mut self,
        value: Option<&T>,
        key: &impl ToCodingKey,
    ) -> Result<(), Self::Error>;
    fn encode<T: Encode>(&mut self, value: &T, key: &impl ToCodingKey) -> Result<(), Self::Error>;

    fn nested_container<'a>(
        &'a mut self,
        key: &'a impl ToCodingKey,
    ) -> Result<<Self::Encoder<'a> as Encoder<'_>>::KeyedContainer, Self::Error>;

    fn nested_seq_container<'a>(
        &'a mut self,
        key: &'a impl ToCodingKey,
    ) -> Result<<Self::Encoder<'a> as Encoder<'_>>::SeqContainer, Self::Error>;

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

    fn finish(self) -> Self::Value;
}

pub trait ValueContainer {
    type Error;
    type Value;
    type Encoder<'a>: Encoder<'a>;

    fn coding_path(&self) -> &CodingPath<'_>;

    fn encode_u8(&mut self, value: u8) -> Result<(), Self::Error>;
    fn encode_u16(&mut self, value: u16) -> Result<(), Self::Error>;
    fn encode_u32(&mut self, value: u32) -> Result<(), Self::Error>;
    fn encode_u64(&mut self, value: u64) -> Result<(), Self::Error>;
    fn encode_u128(&mut self, value: u128) -> Result<(), Self::Error>;
    fn encode_usize(&mut self, value: usize) -> Result<(), Self::Error>;
    fn encode_i8(&mut self, value: i8) -> Result<(), Self::Error>;
    fn encode_i16(&mut self, value: i16) -> Result<(), Self::Error>;
    fn encode_i32(&mut self, value: i32) -> Result<(), Self::Error>;
    fn encode_i64(&mut self, value: i64) -> Result<(), Self::Error>;
    fn encode_i128(&mut self, value: i128) -> Result<(), Self::Error>;
    fn encode_isize(&mut self, value: isize) -> Result<(), Self::Error>;
    fn encode_str(&mut self, value: &str) -> Result<(), Self::Error>;
    fn encode_f32(&mut self, value: f32) -> Result<(), Self::Error>;
    fn encode_f64(&mut self, value: f64) -> Result<(), Self::Error>;
    fn encode_bool(&mut self, value: bool) -> Result<(), Self::Error>;
    fn encode_null(&mut self) -> Result<(), Self::Error>;
    fn encode_option<T: Encode>(&mut self, value: Option<&T>) -> Result<(), Self::Error>;
    fn encode<T: Encode>(&mut self, value: &T) -> Result<(), Self::Error>;

    fn finish(self) -> Self::Value;
}

pub trait SeqContainer {
    type Error;
    type Value;
    type Encoder<'a>: Encoder<'a>
    where
        Self: 'a;

    fn coding_path(&self) -> &CodingPath<'_>;

    fn encode_u8(&mut self, value: u8) -> Result<(), Self::Error>;
    fn encode_u16(&mut self, value: u16) -> Result<(), Self::Error>;
    fn encode_u32(&mut self, value: u32) -> Result<(), Self::Error>;
    fn encode_u64(&mut self, value: u64) -> Result<(), Self::Error>;
    fn encode_u128(&mut self, value: u128) -> Result<(), Self::Error>;
    fn encode_usize(&mut self, value: usize) -> Result<(), Self::Error>;
    fn encode_i8(&mut self, value: i8) -> Result<(), Self::Error>;
    fn encode_i16(&mut self, value: i16) -> Result<(), Self::Error>;
    fn encode_i32(&mut self, value: i32) -> Result<(), Self::Error>;
    fn encode_i64(&mut self, value: i64) -> Result<(), Self::Error>;
    fn encode_i128(&mut self, value: i128) -> Result<(), Self::Error>;
    fn encode_isize(&mut self, value: isize) -> Result<(), Self::Error>;
    fn encode_str(&mut self, value: &str) -> Result<(), Self::Error>;
    fn encode_f32(&mut self, value: f32) -> Result<(), Self::Error>;
    fn encode_f64(&mut self, value: f64) -> Result<(), Self::Error>;
    fn encode_bool(&mut self, value: bool) -> Result<(), Self::Error>;
    fn encode_null(&mut self) -> Result<(), Self::Error>;
    fn encode_option<T: Encode>(&mut self, value: Option<&T>) -> Result<(), Self::Error>;
    fn encode<T: Encode>(&mut self, value: &T) -> Result<(), Self::Error>;

    fn nested_container<'a>(
        &'a mut self,
    ) -> Result<<Self::Encoder<'a> as Encoder<'_>>::KeyedContainer, Self::Error>;

    fn nested_seq_container<'a>(
        &'a mut self,
    ) -> Result<<Self::Encoder<'a> as Encoder<'_>>::SeqContainer, Self::Error>;

    fn finish(self) -> Self::Value;
}

pub type EncodeResult<'e, E> = Result<<E as Encoder<'e>>::Value, <E as Encoder<'e>>::Error>;

pub trait Encode {
    fn encode<'e, E>(&self, encoder: &mut E) -> EncodeResult<'e, E>
    where
        E: Encoder<'e>;
}

macro_rules! encode_prim {
    ($ty:ident, $func:ident) => {
        impl Encode for $ty {
            fn encode<'e, E>(&self, encoder: &mut E) -> EncodeResult<'e, E>
            where
                E: Encoder<'e>,
            {
                let mut con = encoder.as_value_container();
                con.$func(*self)?;
                Ok(con.finish())
            }
        }
    };
}

encode_prim!(u8, encode_u8);
encode_prim!(u16, encode_u16);
encode_prim!(u32, encode_u32);
encode_prim!(u64, encode_u64);
encode_prim!(u128, encode_u128);
encode_prim!(usize, encode_usize);
encode_prim!(i8, encode_i8);
encode_prim!(i16, encode_i16);
encode_prim!(i32, encode_i32);
encode_prim!(i64, encode_i64);
encode_prim!(i128, encode_i128);
encode_prim!(isize, encode_isize);
encode_prim!(f32, encode_f32);
encode_prim!(f64, encode_f64);
encode_prim!(bool, encode_bool);

impl Encode for &str {
    fn encode<'e, E>(&self, encoder: &mut E) -> EncodeResult<'e, E>
    where
        E: Encoder<'e>,
    {
        let mut con = encoder.as_value_container();
        con.encode_str(self)?;
        Ok(con.finish())
    }
}

impl Encode for &String {
    fn encode<'e, E>(&self, encoder: &mut E) -> EncodeResult<'e, E>
    where
        E: Encoder<'e>,
    {
        let mut con = encoder.as_value_container();
        con.encode_str(self)?;
        Ok(con.finish())
    }
}

impl Encode for String {
    fn encode<'e, E>(&self, encoder: &mut E) -> EncodeResult<'e, E>
    where
        E: Encoder<'e>,
    {
        let mut con = encoder.as_value_container();
        con.encode_str(&self)?;
        Ok(con.finish())
    }
}

impl<T: Encode + ToOwned> Encode for Cow<'_, T> {
    fn encode<'e, E>(&self, encoder: &mut E) -> EncodeResult<'e, E>
    where
        E: Encoder<'e>,
    {
        let mut con = encoder.as_value_container();
        con.encode(&*self)?;
        Ok(con.finish())
    }
}

macro_rules! encode_map {
    ($ty:ident) => {
        impl<K: ToCodingKey, V: Encode> Encode for $ty<K, V> {
            fn encode<'e, E>(&self, encoder: &mut E) -> EncodeResult<'e, E>
            where
                E: Encoder<'e>,
            {
                let mut con = encoder.as_container();
                for (k, v) in self.iter() {
                    con.encode(v, k)?;
                }
                Ok(con.finish())
            }
        }

        impl<K: ToCodingKey, V: Encode> Encode for &$ty<K, V> {
            fn encode<'e, E>(&self, encoder: &mut E) -> EncodeResult<'e, E>
            where
                E: Encoder<'e>,
            {
                let mut con = encoder.as_container();
                for (k, v) in self.iter() {
                    con.encode(v, k)?;
                }
                Ok(con.finish())
            }
        }
    };
}

encode_map!(HashMap);
encode_map!(BTreeMap);
encode_map!(IndexMap);

impl<T: Encode> Encode for &Vec<T> {
    fn encode<'e, E>(&self, encoder: &mut E) -> EncodeResult<'e, E>
    where
        E: Encoder<'e>,
    {
        let mut con = encoder.as_seq_container();
        for v in self.iter() {
            con.encode(v)?;
        }
        Ok(con.finish())
    }
}

impl<T: Encode> Encode for Vec<T> {
    fn encode<'e, E>(&self, encoder: &mut E) -> EncodeResult<'e, E>
    where
        E: Encoder<'e>,
    {
        let mut con = encoder.as_seq_container();
        for v in self.iter() {
            con.encode(v)?;
        }
        Ok(con.finish())
    }
}

impl<T: Encode> Encode for &[T] {
    fn encode<'e, E>(&self, encoder: &mut E) -> EncodeResult<'e, E>
    where
        E: Encoder<'e>,
    {
        let mut con = encoder.as_seq_container();
        for v in self.iter() {
            con.encode(v)?;
        }
        Ok(con.finish())
    }
}

impl<T: Encode> Encode for HashSet<T> {
    fn encode<'e, E>(&self, encoder: &mut E) -> EncodeResult<'e, E>
    where
        E: Encoder<'e>,
    {
        let mut con = encoder.as_seq_container();
        for v in self.iter() {
            con.encode(v)?;
        }
        Ok(con.finish())
    }
}

impl<T: Encode> Encode for &HashSet<T> {
    fn encode<'e, E>(&self, encoder: &mut E) -> EncodeResult<'e, E>
    where
        E: Encoder<'e>,
    {
        let mut con = encoder.as_seq_container();
        for v in self.iter() {
            con.encode(v)?;
        }
        Ok(con.finish())
    }
}

impl<T: Encode> Encode for BTreeSet<T> {
    fn encode<'e, E>(&self, encoder: &mut E) -> EncodeResult<'e, E>
    where
        E: Encoder<'e>,
    {
        let mut con = encoder.as_seq_container();
        for v in self.iter() {
            con.encode(v)?;
        }
        Ok(con.finish())
    }
}

impl<T: Encode> Encode for &BTreeSet<T> {
    fn encode<'e, E>(&self, encoder: &mut E) -> EncodeResult<'e, E>
    where
        E: Encoder<'e>,
    {
        let mut con = encoder.as_seq_container();
        for v in self.iter() {
            con.encode(v)?;
        }
        Ok(con.finish())
    }
}

pub trait Encoder<'a>: 'a {
    type Value;
    type Error: Debug + 'static;

    type KeyedContainer: KeyedContainer<
        Encoder<'a> = Self,
        Value = Self::Value,
        Error = Self::Error,
    >
    where
        Self: 'a;

    type ValueContainer: ValueContainer<
        Encoder<'a> = Self,
        Value = Self::Value,
        Error = Self::Error,
    >
    where
        Self: 'a;

    type SeqContainer: SeqContainer<Encoder<'a> = Self, Value = Self::Value, Error = Self::Error>
    where
        Self: 'a;

    fn as_container(&mut self) -> Self::KeyedContainer;
    fn as_value_container(&mut self) -> Self::ValueContainer;
    fn as_seq_container(&mut self) -> Self::SeqContainer;
}
