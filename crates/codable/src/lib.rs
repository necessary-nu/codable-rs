#![deny(rust_2018_idioms)]

mod coding_path;
pub mod dec;
pub mod enc;

#[cfg(feature = "derive")]
pub use codable_derive::{Decode, Encode};

pub use coding_path::{CodingKey, CodingPath, CodingPathIter, ToCodingKey};

#[cfg(all(test, feature = "derive"))]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    #[derive(Debug, Encode)]
    #[codable(rename = "kebab-case", tag("type", "LeEnum"), tag("second", 42))]
    enum Enum {
        A,
        B,
        Potato,
        C,
        #[codable(rename = "test")]
        AnotherOne,
    }

    #[derive(Debug, Encode, Decode)]
    #[codable(rename = "kebab-case", tag("type", "blep"), tag("type2", "blep2"))]
    struct Something {
        a: u8,
        b: u16,
        c: u32,
        d: u64,
        e: u128,
        f: i8,
        g: i16,
        h: i32,
        i: i64,
        j: i128,
        k: f32,
        l: f64,
        m: bool,
        // n: char,
        o: String,
        p: Vec<u8>,
        q_r_s: Option<u8>,
        le_enum: Enum,
        le_enums: Vec<Enum>,
        u: uuid::Uuid,
        um: BTreeMap<uuid::Uuid, u8>,
    }

    #[test]
    fn blep() {
        let mut um = BTreeMap::new();
        um.insert(uuid::Uuid::default(), 42);
        let x = Something {
            a: Default::default(),
            b: Default::default(),
            c: Default::default(),
            d: Default::default(),
            e: Default::default(),
            f: Default::default(),
            g: Default::default(),
            h: Default::default(),
            i: Default::default(),
            j: Default::default(),
            k: 1234.56,
            l: 1234.56,
            m: Default::default(),
            // n: Default::default(),
            o: Default::default(),
            p: Default::default(),
            q_r_s: Default::default(),
            le_enum: Enum::AnotherOne,
            le_enums: vec![Enum::AnotherOne, Enum::A],
            u: uuid::Uuid::default(),
            um,
        };
        let x = codable_json::to_value(&x).unwrap();
        println!("{:?}", x);
    }
}
