use serde::{Serialize, Deserialize, de, Serializer, Deserializer};
use std::{fmt, ops::{Index, IndexMut}};

use crate::{Color, BLACK, LIGHTHOUSE_SIZE, LIGHTHOUSE_ROWS, LIGHTHOUSE_COLS, Pos};

/// An 'image' to be displayed on the lighthouse.
/// The pixels are stored in row-major order.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Display {
    pixels: [Color; LIGHTHOUSE_SIZE],
}

impl Display {
    /// Creates a new empty `Display`.
    pub fn empty() -> Self {
        Self::fill(BLACK)
    }

    /// Creates a new `Display` from the given pixels in
    /// row-major order.
    pub fn new(pixels: [Color; LIGHTHOUSE_SIZE]) -> Self {
        Self { pixels }
    }

    /// Creates a new uniformly colored `Display`.
    pub fn fill(color: Color) -> Self {
        Self { pixels: [color; LIGHTHOUSE_SIZE] }
    }

    /// Creates a new `Display` from the given generator function
    /// that associates each position of the form `(x, y)` with a
    /// color.
    pub fn generate(f: impl Fn(usize, usize) -> Color) -> Self {
        let mut display = Self::empty();
        for y in 0..LIGHTHOUSE_ROWS {
            for x in 0..LIGHTHOUSE_COLS {
                display.set(x, y, f(x, y));
            }
        }
        display
    }

    /// Fetches the pixel at the given position.
    pub fn get(&self, x: usize, y: usize) -> Color {
        self[Pos::new(x, y)]
    }

    /// Sets the given pixel to the given color.
    pub fn set(&mut self, x: usize, y: usize, color: Color) {
        self[Pos::new(x, y)] = color;
    }
}

impl Index<Pos> for Display {
    type Output = Color;

    fn index(&self, pos: Pos) -> &Color {
        &self.pixels[pos.y * LIGHTHOUSE_COLS + pos.x]
    }
}

impl IndexMut<Pos> for Display {
    fn index_mut(&mut self, pos: Pos) -> &mut Color {
        &mut self.pixels[pos.y * LIGHTHOUSE_COLS + pos.x]
    }
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
                    .collect::<Vec<_>>()
                    .try_into()
                    .map(Ok)
                    .unwrap_or_else(|_| Err(E::custom("Could not decode into pixels array".to_owned())))?
            })
        } else {
            Err(E::custom(format!("{} (length of byte array) is not a multiple of 3", v.len())))
        }
    }
}
