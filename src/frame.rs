use crate::{DEFAULT_COLS, DEFAULT_ROWS};

pub type Frame = Vec<Vec<char>>;

// Create a new frame with the given width and height
pub fn new_frame() -> Frame {
    vec![vec![' '; DEFAULT_ROWS]; DEFAULT_COLS]
}

// Trait for drawable objects. They should implement a draw function that modifies the frame.
pub trait Drawable {
    fn draw(&self, frame: &mut Frame);
}
