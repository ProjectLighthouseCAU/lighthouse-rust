use crate::{Rect, Vec2, Zero};

/// The number of rows of the lighthouse.
pub const LIGHTHOUSE_ROWS: usize = 14;
/// The number of columns of the lighthouse.
pub const LIGHTHOUSE_COLS: usize = 28;
/// The number of pixels in a lighthouse frame.
pub const LIGHTHOUSE_SIZE: usize = LIGHTHOUSE_ROWS * LIGHTHOUSE_COLS;
/// The total number of bytes in a lighthouse frame.
pub const LIGHTHOUSE_BYTES: usize = LIGHTHOUSE_SIZE * 3;
/// The rectangle of valid coordinates on the lighthouse.
pub const LIGHTHOUSE_RECT: Rect<i32> = Rect::new(Vec2::ZERO, Vec2::new(LIGHTHOUSE_COLS as i32, LIGHTHOUSE_ROWS as i32));
