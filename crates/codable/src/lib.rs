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

    #[derive(Debug, Encode)]
    #[codable(rename = "kebab-case")]
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
    }

    #[test]
    fn blep() {
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
            k: Default::default(),
            l: Default::default(),
            m: Default::default(),
            // n: Default::default(),
            o: Default::default(),
            p: Default::default(),
            q_r_s: Default::default(),
            le_enum: Enum::AnotherOne,
        };
        let x = codable_json::to_value(&x).unwrap();
        println!("{:?}", x);
    }
}
