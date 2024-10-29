use crate::frame::Frame;
use crossterm::{
    cursor::MoveTo,
    style::{Color, SetBackgroundColor, SetForegroundColor},
    terminal::{size, Clear, ClearType},
    QueueableCommand,
};
use std::io::{Stdout, Write};

pub fn render(stdout: &mut Stdout, last_frame: &Frame, curr_frame: &Frame, force: bool) {
    // Get terminal size
    let (term_width, term_height) = size().unwrap();
    let frame_width = curr_frame.len() as u16;
    let frame_height = curr_frame[0].len() as u16;

    // Calculate offsets to center the entire box, including the border
    let x_offset = (term_width.saturating_sub(frame_width + 2)) / 2; // +2 to account for the border
    let y_offset = (term_height.saturating_sub(frame_height + 2)) / 2; // +2 to account for the border

    // Clear and set colors if forced
    if force {
        stdout.queue(SetBackgroundColor(Color::Blue)).unwrap();
        stdout.queue(Clear(ClearType::All)).unwrap();
        stdout.queue(SetBackgroundColor(Color::Black)).unwrap();
        stdout.queue(SetForegroundColor(Color::White)).unwrap();
    }

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

    // Reset color back to default for game content
    stdout.queue(SetForegroundColor(Color::White)).unwrap();

    // Iterate over each cell and render the game frame with offset
    for (x, col) in curr_frame.iter().enumerate() {
        for (y, s) in col.iter().enumerate() {
            if *s != last_frame[x][y] || force {
                stdout
                    .queue(MoveTo(x as u16 + x_offset + 1, y as u16 + y_offset + 1)) // Offset by 1 for the border
                    .unwrap();
                print!("{}", *s);
            }
        }
    }

    stdout.flush().unwrap();
}
