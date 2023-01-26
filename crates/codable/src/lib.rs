// #![feature(generic_associated_types)]

use std::{
    borrow::Cow,
    fmt::{Debug, Display},
    marker::PhantomData,
};

#[derive(Debug, Clone)]
pub enum CodingKey {
    Root,
    String(Cow<'static, str>),
    Int(usize),
}

impl CodingKey {
    pub fn as_str(&self) -> Cow<'static, str> {
        match self {
            CodingKey::Root => Cow::Borrowed(""),
            CodingKey::String(x) => x.clone(),
            CodingKey::Int(x) => Cow::Owned(format!("{}", x)),
        }
    }
}

pub trait ToCodingKey {
    fn to_coding_key(&self) -> CodingKey;
    fn as_str(&self) -> Cow<'static, str> {
        self.to_coding_key().as_str()
    }
}

impl ToCodingKey for String {
    fn to_coding_key(&self) -> CodingKey {
        CodingKey::String(Cow::Owned(self.clone()))
    }
}

impl<'a> ToCodingKey for &'a str {
    fn to_coding_key(&self) -> CodingKey {
        CodingKey::String(Cow::Owned(self.to_string()))
    }
}

impl Display for CodingKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CodingKey::Root => f.write_str("<root>"),
            CodingKey::String(x) => Display::fmt(&x, f),
            CodingKey::Int(x) => Display::fmt(&x, f),
        }
    }
}

pub mod enc {
    use std::{
        borrow::Cow,
        collections::{BTreeMap, HashMap},
        fmt::Debug,
    };

    use indexmap::IndexMap;

    use crate::{CodingPath, ToCodingKey};

    use super::CodingKey;

    pub trait KeyedContainer {
        type Error;
        type Value;
        type Encoder<'a>: Encoder<'a>
        where
            Self: 'a;

        fn coding_path(&self) -> &CodingPath<'_, CodingKey>;

        fn encode_u8(&mut self, value: u8, key: &impl ToCodingKey) -> Result<(), Self::Error>;
        fn encode_u16(&mut self, value: u16, key: &impl ToCodingKey) -> Result<(), Self::Error>;
        fn encode_u32(&mut self, value: u32, key: &impl ToCodingKey) -> Result<(), Self::Error>;
        fn encode_u64(&mut self, value: u64, key: &impl ToCodingKey) -> Result<(), Self::Error>;
        fn encode_u128(&mut self, value: u128, key: &impl ToCodingKey) -> Result<(), Self::Error>;
        fn encode_usize(&mut self, value: usize, key: &impl ToCodingKey)
            -> Result<(), Self::Error>;
        fn encode_i8(&mut self, value: i8, key: &impl ToCodingKey) -> Result<(), Self::Error>;
        fn encode_i16(&mut self, value: i16, key: &impl ToCodingKey) -> Result<(), Self::Error>;
        fn encode_i32(&mut self, value: i32, key: &impl ToCodingKey) -> Result<(), Self::Error>;
        fn encode_i64(&mut self, value: i64, key: &impl ToCodingKey) -> Result<(), Self::Error>;
        fn encode_i128(&mut self, value: i128, key: &impl ToCodingKey) -> Result<(), Self::Error>;
        fn encode_isize(&mut self, value: isize, key: &impl ToCodingKey)
            -> Result<(), Self::Error>;
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
            value: Option<T>,
            key: &impl ToCodingKey,
        ) -> Result<(), Self::Error>;
        fn encode<T: Encode>(
            &mut self,
            value: &T,
            key: &impl ToCodingKey,
        ) -> Result<(), Self::Error>;

        fn nested_container<'a>(
            &'a mut self,
            key: &impl ToCodingKey,
        ) -> Result<<Self::Encoder<'a> as Encoder>::KeyedContainer, Self::Error>;

        fn nested_seq_container<'a>(
            &'a mut self,
            key: &impl ToCodingKey,
        ) -> Result<<Self::Encoder<'a> as Encoder>::SeqContainer, Self::Error>;

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

        fn coding_path(&self) -> &CodingPath<'_, CodingKey>;

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
        fn encode_option<T: Encode>(&mut self, value: Option<T>) -> Result<(), Self::Error>;
        fn encode<T: Encode>(&mut self, value: &T) -> Result<(), Self::Error>;

        fn finish(self) -> Self::Value;
    }

    pub trait SeqContainer {
        type Error;
        type Value;
        type Encoder<'a>: Encoder<'a>
        where
            Self: 'a;

        fn coding_path(&self) -> &CodingPath<'_, CodingKey>;

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
        fn encode_option<T: Encode>(&mut self, value: Option<T>) -> Result<(), Self::Error>;
        fn encode<T: Encode>(&mut self, value: &T) -> Result<(), Self::Error>;

        fn nested_container<'a>(
            &'a mut self,
        ) -> Result<<Self::Encoder<'a> as Encoder>::KeyedContainer, Self::Error>;

        fn nested_seq_container<'a>(
            &'a mut self,
        ) -> Result<<Self::Encoder<'a> as Encoder>::SeqContainer, Self::Error>;

        fn finish(self) -> Self::Value;
    }

    pub type EncodeResult<'e, E> = Result<<E as Encoder<'e>>::Value, <E as Encoder<'e>>::Error>;

    pub trait Encode {
        fn encode<'e, E>(&self, encoder: E) -> EncodeResult<'e, E>
        where
            E: Encoder<'e>;
    }

    macro_rules! encode_prim {
        ($ty:ident, $func:ident) => {
            impl Encode for $ty {
                fn encode<'e, E>(&self, encoder: E) -> EncodeResult<'e, E>
                where
                    E: Encoder<'e>,
                {
                    let mut con = encoder.into_value_container();
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
        fn encode<'e, E>(&self, encoder: E) -> EncodeResult<'e, E>
        where
            E: Encoder<'e>,
        {
            let mut con = encoder.into_value_container();
            con.encode_str(self)?;
            Ok(con.finish())
        }
    }

    impl Encode for String {
        fn encode<'e, E>(&self, encoder: E) -> EncodeResult<'e, E>
        where
            E: Encoder<'e>,
        {
            let mut con = encoder.into_value_container();
            con.encode_str(&self)?;
            Ok(con.finish())
        }
    }

    macro_rules! encode_map {
        ($ty:ident) => {
            impl<K: ToCodingKey, V: Encode> Encode for $ty<K, V> {
                fn encode<'e, E>(&self, encoder: E) -> EncodeResult<'e, E>
                where
                    E: Encoder<'e>,
                {
                    let mut con = encoder.into_container();
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
        fn encode<'e, E>(&self, encoder: E) -> EncodeResult<'e, E>
        where
            E: Encoder<'e>,
        {
            let mut con = encoder.into_seq_container();
            for v in self.iter() {
                con.encode(v)?;
            }
            Ok(con.finish())
        }
    }

    impl<T: Encode> Encode for &[T] {
        fn encode<'e, E>(&self, encoder: E) -> EncodeResult<'e, E>
        where
            E: Encoder<'e>,
        {
            let mut con = encoder.into_seq_container();
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

        type SeqContainer: SeqContainer<
            Encoder<'a> = Self,
            Value = Self::Value,
            Error = Self::Error,
        >
        where
            Self: 'a;

        fn into_container(self) -> Self::KeyedContainer;
        fn into_value_container(self) -> Self::ValueContainer;
        fn into_seq_container(self) -> Self::SeqContainer;
    }
}

pub mod dec {
    pub trait Decoder<'a> {
        type Value;
        type Error; //: Debug + 'static;

        type KeyedContainer: KeyedContainer<
            Decoder<'a> = Self,
            Value = Self::Value,
            Error = Self::Error,
        >
        where
            Self: 'a;

        type ValueContainer: ValueContainer<
            Decoder<'a> = Self,
            Value = Self::Value,
            Error = Self::Error,
        >
        where
            Self: 'a;

        type SeqContainer: SeqContainer<
            Decoder<'a> = Self,
            Value = Self::Value,
            Error = Self::Error,
        >
        where
            Self: 'a;

        fn into_container(self) -> Result<Self::KeyedContainer, Self::Error>;
        fn into_value_container(self) -> Result<Self::ValueContainer, Self::Error>;
        fn into_seq_container(self) -> Result<Self::SeqContainer, Self::Error>;
    }

    use std::{
        borrow::Cow,
        collections::{BTreeMap, HashMap},
        fmt::Debug, marker::PhantomData,
    };

    use indexmap::IndexMap;

    use crate::{CodingPath, ToCodingKey};

    use super::CodingKey;

    pub trait KeyedContainer {
        type Error;
        type Value;
        type Decoder<'a>: Decoder<'a>
        where
            Self: 'a;

        fn coding_path(&self) -> &CodingPath<'_, CodingKey>;
        fn contains(&self, coding_key: &impl ToCodingKey) -> bool;

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
        fn decode_option<T: Decode>(&mut self, key: &impl ToCodingKey) -> Result<T, Self::Error>;

        fn decode<T: Decode>(&mut self, key: &impl ToCodingKey) -> Result<T, Self::Error>;

        fn nested_container<'a>(
            &'a mut self,
            key: &impl ToCodingKey,
        ) -> Result<<Self::Decoder<'a> as Decoder>::KeyedContainer, Self::Error>;

        fn nested_seq_container<'a>(
            &'a mut self,
            key: &impl ToCodingKey,
        ) -> Result<<Self::Decoder<'a> as Decoder>::SeqContainer, Self::Error>;

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
        fn opt_decode_usize(
            &mut self,
            key: &impl ToCodingKey,
        ) -> Result<Option<usize>, Self::Error> {
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
        fn opt_decode_isize(
            &mut self,
            key: &impl ToCodingKey,
        ) -> Result<Option<isize>, Self::Error> {
            if self.contains(key) {
                Ok(Some(self.decode_isize(key)?))
            } else {
                Ok(None)
            }
        }
        fn opt_decode_string(
            &mut self,
            key: &impl ToCodingKey,
        ) -> Result<Option<String>, Self::Error> {
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
        type Decoder<'a>: Decoder<'a>;

        fn coding_path(&self) -> &CodingPath<'_, CodingKey>;

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
        fn decode_option<T: Decode>(&mut self) -> Result<T, Self::Error>;
        fn decode<T: Decode>(&mut self) -> Result<T, Self::Error>;
    }

    pub trait SeqContainer {
        type Error;
        type Value;
        type Decoder<'a>: Decoder<'a>
        where
            Self: 'a;

        fn coding_path(&self) -> &CodingPath<'_, CodingKey>;

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
        fn decode_option<T: Decode>(&mut self) -> Result<T, Self::Error>;
        fn decode<T: Decode>(&mut self) -> Result<T, Self::Error>;

        fn nested_container<'a>(
            &'a mut self,
        ) -> Result<<Self::Decoder<'a> as Decoder>::KeyedContainer, Self::Error>;

        fn nested_seq_container<'a>(
            &'a mut self,
        ) -> Result<<Self::Decoder<'a> as Decoder>::SeqContainer, Self::Error>;
    }

    pub type DecodeResult<'d, T, D> = Result<T, <D as Decoder<'d>>::Error>;

    pub trait Decode {
        fn decode<'d, D>(decoder: D) -> DecodeResult<'d, Self, D>
        where
            Self: Sized,
            D: Decoder<'d> + 'd;
    }
}

#[derive(Clone)]
pub struct CodingPath<'a, T>(*const CodingPath<'a, T>, T, PhantomData<&'a ()>);

impl<T: Clone + Debug> Debug for CodingPath<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_vec().fmt(f)
    }
}

impl<'a, T: Clone> CodingPath<'a, T> {
    pub fn root(value: T) -> CodingPath<'static, T> {
        CodingPath(std::ptr::null(), value, PhantomData)
    }

    pub fn join(&self, item: T) -> CodingPath<'a, T> {
        CodingPath(self, item, PhantomData)
    }

    pub fn iter(&self) -> CodingPathIter<'_, T> {
        CodingPathIter { current: self }
    }

    pub fn to_vec(&self) -> Vec<T> {
        let mut vec = self.iter().cloned().collect::<Vec<_>>();
        vec.reverse();
        vec
    }
}

pub struct CodingPathIter<'a, T> {
    current: *const CodingPath<'a, T>,
}

impl<'a, T> Iterator for CodingPathIter<'a, T>
where
    T: 'a,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.is_null() {
            return None;
        }

        let data = unsafe { &*self.current };
        self.current = data.0;
        Some(&data.1)
    }
}
