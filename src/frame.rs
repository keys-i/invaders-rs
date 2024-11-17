use crossterm::terminal::size;

pub type Frame = Vec<Vec<char>>;

// Create a new frame with the given width and height
pub fn new_frame() -> Frame {
    // Get terminal size
    let (term_width, term_height) = size().unwrap();

    // Subtract some space for the border (if you're drawing a border around the game)
    let frame_width = term_width.saturating_sub(50); // Account for border or padding if needed
    let frame_height = term_height.saturating_sub(10);

    // Create a frame with dynamic rows and columns
    let frame = vec![vec![' '; frame_height as usize]; frame_width as usize];

    frame
}

// Trait for drawable objects. They should implement a draw function that modifies the frame.
pub trait Drawable {
    fn draw(&self, frame: &mut Frame);
}
