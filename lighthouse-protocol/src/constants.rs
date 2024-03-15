use crate::{Rect, Pos, Delta};

/// The number of rows of the lighthouse.
pub const LIGHTHOUSE_ROWS: usize = 14;
/// The number of columns of the lighthouse.
pub const LIGHTHOUSE_COLS: usize = 28;
/// The lighthouse's size.
pub const LIGHTHOUSE_SIZE: usize = LIGHTHOUSE_ROWS * LIGHTHOUSE_COLS;
/// The rectangle of valid coordinates on the lighthouse.
pub const LIGHTHOUSE_RECT: Rect = Rect::new(Pos::ZERO, Delta::new(LIGHTHOUSE_COLS as i32, LIGHTHOUSE_ROWS as i32));
