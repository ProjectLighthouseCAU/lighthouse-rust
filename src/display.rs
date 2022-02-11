use serde::{Serialize, Deserialize, de, Serializer, Deserializer};
use std::fmt;

use crate::Color;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Display {
    pixels: Vec<Color>,
}

impl Serialize for Display {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer {
        let bytes: Vec<u8> = self.pixels.iter()
            .flat_map(|c| [c.red, c.green, c.blue].into_iter())
            .collect();
        serializer.serialize_bytes(&bytes)
    }
}

impl<'de> Deserialize<'de> for Display {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de> {
        deserializer.deserialize_bytes(DisplayBytesVisitor)
    }
}

struct DisplayBytesVisitor;

impl<'de> de::Visitor<'de> for DisplayBytesVisitor {
    type Value = Display;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "a byte array whose length is a multiple of 3")
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
        where E: de::Error {
        if v.len() % 3 == 0 {
            Ok(Display {
                pixels: v
                    .chunks(3)
                    .map(|c| match c {
                        &[r, g, b] => Color::new(r, g, b),
                        _ => unreachable!(),
                    })
                    .collect()
            })
        } else {
            Err(E::custom(format!("{} (length of byte array) is not a multiple of 3", v.len())))
        }
    }
}
