use rand::{prelude::Distribution, distributions::Standard};
use serde::{Deserialize, Serialize};

/// An RGB color.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Color {
    #[serde(rename = "R")]
    pub red: u8,
    #[serde(rename = "G")]
    pub green: u8,
    #[serde(rename = "B")]
    pub blue: u8,
}

impl Color {
    pub const BLACK: Self = Self { red: 0, green: 0, blue: 0 };
    pub const WHITE: Self = Self { red: 255, green: 255, blue: 255 };
    pub const RED: Self = Self { red: 255, green: 0, blue: 0 };
    pub const GREEN: Self = Self { red: 0, green: 255, blue: 0 };
    pub const BLUE: Self = Self { red: 0, green: 0, blue: 255 };
    pub const YELLOW: Self = Self { red: 255, green: 255, blue: 0 };
    pub const CYAN: Self = Self { red: 0, green: 255, blue: 255 };
    pub const MAGENTA: Self = Self { red: 255, green: 0, blue: 255 };

    /// Creates a new color from the given RGB components.
    pub const fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }
}

impl Distribution<Color> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Color {
        Color::new(rng.gen(), rng.gen(), rng.gen())
    }
}
