pub mod dec;
pub mod enc;
pub mod value;

pub use dec::JsonDecoder;
pub use enc::JsonEncoder;
pub use value::{from_value, to_value, Value};

// #[cfg(test)]
// mod test {
//     use std::collections::HashMap;

//     use codable::{
//         dec::{Decode, DecodeResult, Decoder, KeyedContainer as _},
//         enc::KeyedContainer as _,
//     };

//     use super::*;

//     #[test]
//     fn basic_int() {
//         let value = to_value(&32_u8).unwrap();
//         println!("{:?}", value);
//     }

//     #[test]
//     fn basic_struct() {
//         struct Hmm {
//             test: u32,
//             a_bool: bool,
//             a_str: String,
//         }

//         impl Encode for Hmm {
//             fn encode<'e, E>(&self, encoder: E) -> enc::EncodeResult<'e, E>
//             where
//                 E: Encoder<'e>,
//             {
//                 let mut con = encoder.into_container();
//                 con.encode(&self.test, &"test")?;
//                 con.encode(&self.a_bool, &"a_bool")?;
//                 con.encode(&self.a_str, &"a_str")?;
//                 Ok(con.finish())
//             }
//         }

//         let hmm = Hmm {
//             test: 1238123,
//             a_bool: true,
//             a_str: "a test string".into(),
//         };

//         let value = to_value(&hmm).unwrap();
//         println!("{:?}", value);
//     }

//     #[test]
//     fn enums() {
//         #[derive(Debug)]
//         struct ThingA {
//             tag: String,
//             pew: u32,
//         }

//         #[derive(Debug)]
//         struct ThingB {
//             tag: String,
//             another: String,
//         }

//         #[derive(Debug)]
//         struct Base {
//             tagged: Tagged,
//         }

//         #[derive(Debug)]
//         enum Tagged {
//             ThingA(ThingA),
//             ThingB(ThingB),
//         }

//         impl Encode for ThingA {
//             fn encode<'e, E>(&self, encoder: E) -> enc::EncodeResult<'e, E>
//             where
//                 E: Encoder<'e>,
//             {
//                 let mut c = encoder.into_container();
//                 c.encode_str(&self.tag, &"tag")?;
//                 c.encode_u32(self.pew, &"pew")?;
//                 Ok(c.finish())
//             }
//         }

//         impl Encode for ThingB {
//             fn encode<'e, E>(&self, encoder: E) -> enc::EncodeResult<'e, E>
//             where
//                 E: Encoder<'e>,
//             {
//                 let mut c = encoder.into_container();
//                 c.encode_str(&self.tag, &"tag")?;
//                 c.encode_str(&self.another, &"another")?;
//                 Ok(c.finish())
//             }
//         }

//         impl Encode for Tagged {
//             fn encode<'e, E>(&self, encoder: E) -> enc::EncodeResult<'e, E>
//             where
//                 E: Encoder<'e>,
//             {
//                 match self {
//                     Tagged::ThingA(a) => a.encode(encoder),
//                     Tagged::ThingB(b) => b.encode(encoder),
//                 }
//             }
//         }

//         impl Encode for Base {
//             fn encode<'e, E>(&self, encoder: E) -> enc::EncodeResult<'e, E>
//             where
//                 E: Encoder<'e>,
//             {
//                 let mut c = encoder.into_container();
//                 c.encode(&self.tagged, &"tagged")?;
//                 Ok(c.finish())
//             }
//         }

//         impl Decode for ThingA {
//             fn decode<'d, D>(decoder: &mut D) -> DecodeResult<'d, Self, D>
//             where
//                 D: Decoder + 'd,
//             {
//                 let mut d = decoder.as_container()?;
//                 Ok(ThingA {
//                     tag: "a".into(),
//                     pew: d.decode_u32(&"pew")?,
//                 })
//             }
//         }

//         impl Decode for ThingB {
//             fn decode<'d, D>(decoder: &mut D) -> DecodeResult<'d, Self, D>
//             where
//                 D: Decoder + 'd,
//             {
//                 let mut d = decoder.as_container()?;
//                 Ok(ThingB {
//                     tag: "b".into(),
//                     another: d.decode_string(&"another")?,
//                 })
//             }
//         }

//         impl Decode for Tagged {
//             fn decode<'d, D>(decoder: &mut D) -> DecodeResult<'d, Self, D>
//             where
//                 D: Decoder + 'd,
//             {
//                 let mut d = decoder.as_container()?;
//                 let tag = d.decode_string(&"tag")?;
//                 drop(d);
//                 match &*tag {
//                     "a" => Ok(Tagged::ThingA(ThingA::decode(decoder)?)),
//                     "b" => Ok(Tagged::ThingB(ThingB::decode(decoder)?)),
//                     _ => panic!(),
//                 }
//             }
//         }

//         impl Decode for Base {
//             fn decode<'d, D>(decoder: &mut D) -> DecodeResult<'d, Self, D>
//             where
//                 D: Decoder + 'd,
//             {
//                 let mut d = decoder.as_container()?;
//                 Ok(Base {
//                     tagged: d.decode(&"tagged")?,
//                 })
//             }
//         }

//         let value = to_value(&Base {
//             tagged: Tagged::ThingA(ThingA {
//                 tag: "a".into(),
//                 pew: 32,
//             }),
//         })
//         .unwrap();

//         println!("{:?}", &value);

//         let base = from_value::<Base>(&value).unwrap();

//         println!("{:?}", &base);
//     }

//     #[test]
//     fn encode_prim() {
//         let encoder = JsonEncoder::new();
//         assert_eq!(123u8.encode(encoder).unwrap(), Value::Number("123".into()))
//     }

//     #[test]
//     fn basic_obj() {
//         enum Hmm {
//             String(&'static str),
//             Dict(HashMap<&'static str, Hmm>),
//             Arr(Vec<Hmm>),
//             Number(u32),
//         }
//         impl Encode for Hmm {
//             fn encode<'e, E>(&self, encoder: E) -> enc::EncodeResult<'e, E>
//             where
//                 E: Encoder<'e>,
//             {
//                 match self {
//                     Hmm::String(x) => x.encode(encoder),
//                     Hmm::Dict(x) => x.encode(encoder),
//                     Hmm::Arr(x) => x.encode(encoder),
//                     Hmm::Number(x) => x.encode(encoder),
//                 }
//             }
//         }
//         let mut input = HashMap::new();
//         let mut map = HashMap::new();
//         map.insert("test", Hmm::String("another"));
//         input.insert("hello", Hmm::String("hi"));
//         input.insert("interesting", Hmm::String("yes"));
//         input.insert("lolwut", Hmm::Dict(map));
//         input.insert(
//             "hmm",
//             Hmm::Arr(vec![
//                 Hmm::String("no"),
//                 Hmm::Number(2),
//                 Hmm::Number(3),
//                 Hmm::Number(42),
//                 Hmm::Number(9),
//             ]),
//         );
//         let value = to_value(&input).unwrap();
//         println!("{:?}", value);
//     }
// }
