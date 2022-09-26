use serde::{Serialize, Deserialize, de, Serializer, Deserializer};
use std::{fmt, ops::{Index, IndexMut}};

use crate::{Color, LIGHTHOUSE_SIZE, LIGHTHOUSE_ROWS, LIGHTHOUSE_COLS, Pos};

/// An 'image' to be displayed on the lighthouse.
/// The pixels are stored in row-major order.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Frame {
    pixels: [Color; LIGHTHOUSE_SIZE],
}

impl Frame {
    /// Creates a new empty `Frame`.
    pub fn empty() -> Self {
        Self::fill(Color::BLACK)
    }

    /// Creates a new `Frame` from the given pixels in
    /// row-major order.
    pub fn new(pixels: [Color; LIGHTHOUSE_SIZE]) -> Self {
        Self { pixels }
    }

    /// Creates a new uniformly colored `Frame`.
    pub fn fill(color: Color) -> Self {
        Self { pixels: [color; LIGHTHOUSE_SIZE] }
    }

    /// Creates a new `Frame` from the given generator function
    /// that associates each position of the form `(x, y)` with a
    /// color.
    pub fn generate(f: impl Fn(usize, usize) -> Color) -> Self {
        let mut frame = Self::empty();
        for y in 0..LIGHTHOUSE_ROWS {
            for x in 0..LIGHTHOUSE_COLS {
                frame.set(x, y, f(x, y));
            }
        }
        frame
    }

    /// Fetches the pixel at the given position.
    pub fn get(&self, x: usize, y: usize) -> Color {
        self.pixels[y * LIGHTHOUSE_COLS + x]
    }

    /// Sets the given pixel to the given color.
    pub fn set(&mut self, x: usize, y: usize, color: Color) {
        self.pixels[y * LIGHTHOUSE_COLS + x] = color;
    }
}

impl Index<Pos> for Frame {
    type Output = Color;

    fn index(&self, pos: Pos) -> &Color {
        debug_assert!(pos.in_range());
        &self.pixels[pos.y as usize * LIGHTHOUSE_COLS + pos.x as usize]
    }
}

impl IndexMut<Pos> for Frame {
    fn index_mut(&mut self, pos: Pos) -> &mut Color {
        debug_assert!(pos.in_range());
        &mut self.pixels[pos.y as usize * LIGHTHOUSE_COLS + pos.x as usize]
    }
}

impl Serialize for Frame {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer {
        let bytes: Vec<u8> = self.pixels.iter()
            .flat_map(|c| [c.red, c.green, c.blue].into_iter())
            .collect();
        serializer.serialize_bytes(&bytes)
    }
}

impl<'de> Deserialize<'de> for Frame {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de> {
        deserializer.deserialize_bytes(FrameBytesVisitor)
    }
}

struct FrameBytesVisitor;

impl<'de> de::Visitor<'de> for FrameBytesVisitor {
    type Value = Frame;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "a byte array whose length is a multiple of 3")
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
        where E: de::Error {
        if v.len() % 3 == 0 {
            Ok(Frame {
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
