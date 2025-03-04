use serde::{Serialize, Deserialize, de, Serializer, Deserializer};
use std::{array, fmt, ops::{Index, IndexMut}};

use crate::{Color, Pos, LIGHTHOUSE_BYTES, LIGHTHOUSE_COLS, LIGHTHOUSE_RECT, LIGHTHOUSE_ROWS, LIGHTHOUSE_SIZE};

/// An 'image' to be displayed on the lighthouse.
/// The pixels are stored in row-major order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
        self[Pos::new(x as i32, y as i32)]
    }

    /// Sets the given pixel to the given color.
    pub fn set(&mut self, x: usize, y: usize, color: Color) {
        self[Pos::new(x as i32, y as i32)] = color;
    }
}

impl Index<Pos<i32>> for Frame {
    type Output = Color;

    fn index(&self, pos: Pos<i32>) -> &Color {
        &self.pixels[LIGHTHOUSE_RECT.index_of(pos)]
    }
}

impl IndexMut<Pos<i32>> for Frame {
    fn index_mut(&mut self, pos: Pos<i32>) -> &mut Color {
        &mut self.pixels[LIGHTHOUSE_RECT.index_of(pos)]
    }
}

impl From<[Color; LIGHTHOUSE_SIZE]> for Frame {
    fn from(pixels: [Color; LIGHTHOUSE_SIZE]) -> Self {
        Self::new(pixels)
    }
}

impl From<[u8; LIGHTHOUSE_BYTES]> for Frame {
    fn from(bytes: [u8; LIGHTHOUSE_BYTES]) -> Self {
        Self::new(array::from_fn(|pixel_index| {
            let offset = pixel_index * 3;
            Color::from([offset, offset + 1, offset + 2].map(|i| bytes[i]))
        }))
    }
}

impl From<Frame> for [Color; LIGHTHOUSE_SIZE] {
    fn from(frame: Frame) -> Self {
        frame.pixels
    }
}

impl From<Frame> for Vec<u8> {
    fn from(frame: Frame) -> Self {
        frame.pixels.iter()
            .flat_map(|c| [c.red, c.green, c.blue].into_iter())
            .collect()
    }
}

impl From<Frame> for [u8; LIGHTHOUSE_BYTES] {
    fn from(frame: Frame) -> Self {
        // TODO: Figure out if we could do this without dynamic allocation
        // Sadly, there is no flat_map for arrays, most likely due to the
        // limitations of const generics, which do not support arithemtic (e.g.
        // multiplication). Maybe one day, once we have `generic_const_exprs`...
        Vec::from(frame).try_into().unwrap()
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
        if let Ok(bytes) = <[u8; LIGHTHOUSE_BYTES]>::try_from(v) {
            Ok(Frame::from(bytes))
        } else {
            Err(E::custom(format!("{} (length of byte array) is not {}", v.len(), LIGHTHOUSE_BYTES)))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Color, Frame, LIGHTHOUSE_BYTES};

    #[test]
    fn from_array() {
        assert_eq!(Frame::from([0u8; LIGHTHOUSE_BYTES]), Frame::empty());
        assert_eq!(Frame::from([255u8; LIGHTHOUSE_BYTES]), Frame::fill(Color::WHITE));
    }

    #[test]
    fn to_array() {
        assert_eq!(<[u8; LIGHTHOUSE_BYTES]>::from(Frame::empty()), [0u8; LIGHTHOUSE_BYTES]);
        assert_eq!(<[u8; LIGHTHOUSE_BYTES]>::from(Frame::fill(Color::WHITE)), [255u8; LIGHTHOUSE_BYTES]);
    }
}
