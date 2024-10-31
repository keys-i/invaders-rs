use crate::frame::Frame;
use crossterm::{
    cursor::MoveTo,
    style::{Color, SetBackgroundColor, SetForegroundColor},
    terminal::{size, Clear, ClearType},
    QueueableCommand,
};
use std::io::{Stdout, Write};

pub fn render(
    stdout: &mut Stdout,
    last_frame: &Frame,
    curr_frame: &Frame,
    force: bool,
    last_size: &mut (u16, u16), // Track the last terminal size
) {
    // Get current terminal size
    let (term_width, term_height) = size().unwrap();

    // Check if the terminal has been resized
    let resized = (term_width, term_height) != *last_size;
    if resized || force {
        *last_size = (term_width, term_height); // Update the last known terminal size
        stdout.queue(SetBackgroundColor(Color::Blue)).unwrap(); // Set background to blue
        stdout.queue(Clear(ClearType::All)).unwrap(); // Clear screen with blue background
    }

    let frame_width = curr_frame.len() as u16;
    let frame_height = curr_frame[0].len() as u16;

    // Calculate offsets to center the entire box, including the border
    let x_offset = (term_width.saturating_sub(frame_width + 2)) / 2; // +2 to account for the border
    let y_offset = (term_height.saturating_sub(frame_height + 2)) / 2; // +2 to account for the border

    // Draw white border around the game box
    stdout.queue(SetForegroundColor(Color::White)).unwrap();

    // Draw top border
    stdout.queue(MoveTo(x_offset, y_offset)).unwrap();
    print!("{}", "─".repeat((frame_width + 2) as usize));

    // Draw bottom border
    stdout
        .queue(MoveTo(x_offset, y_offset + frame_height + 1))
        .unwrap();
    print!("{}", "─".repeat((frame_width + 2) as usize));

    // Draw left and right borders
    for y in 0..frame_height {
        // Left border
        stdout.queue(MoveTo(x_offset, y_offset + y + 1)).unwrap(); // Shifted down by 1 for the border
        print!("│");
        // Right border
        stdout
            .queue(MoveTo(x_offset + frame_width + 1, y_offset + y + 1))
            .unwrap(); // Shifted down by 1 for the border
        print!("│");
    }

    // Draw the corners
    stdout.queue(MoveTo(x_offset, y_offset)).unwrap();
    print!("┏");
    stdout
        .queue(MoveTo(x_offset + frame_width + 1, y_offset))
        .unwrap();
    print!("┓");
    stdout
        .queue(MoveTo(x_offset, y_offset + frame_height + 1))
        .unwrap();
    print!("└");
    stdout
        .queue(MoveTo(
            x_offset + frame_width + 1,
            y_offset + frame_height + 1,
        ))
        .unwrap();
    print!("┘");

    // Reset color back to default for game content (white text, black background)
    stdout.queue(SetBackgroundColor(Color::Black)).unwrap();
    stdout.queue(SetForegroundColor(Color::White)).unwrap();

    // Ensure the frame sizes match to avoid out-of-bounds access
    let min_width = curr_frame.len().min(last_frame.len());
    let min_height = curr_frame[0].len().min(last_frame[0].len());

    // Ensure we're not rendering outside the terminal bounds
    let renderable_width = (term_width.saturating_sub(2)).min(min_width as u16);
    let renderable_height = (term_height.saturating_sub(2)).min(min_height as u16);

    // Iterate over each cell and render the game frame with offset
    for x in 0..renderable_width as usize {
        for y in 0..renderable_height as usize {
            if curr_frame[x][y] != last_frame[x][y] || force || resized {
                stdout
                    .queue(MoveTo(x as u16 + x_offset + 1, y as u16 + y_offset + 1)) // Offset by 1 for the border
                    .unwrap();
                print!("{}", curr_frame[x][y]);
            }
        }
    }

    stdout.flush().unwrap();
}
