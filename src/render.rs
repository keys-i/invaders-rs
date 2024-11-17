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
    last_size: &mut (u16, u16),
) {
    let (term_width, term_height) = size().unwrap_or((0, 0));
    let resized = (term_width, term_height) != *last_size;

    if resized || force {
        *last_size = (term_width, term_height);
        stdout.queue(SetBackgroundColor(Color::Blue)).unwrap();
        stdout.queue(Clear(ClearType::All)).unwrap();
    }

    let frame_width = curr_frame.len() as u16;
    let frame_height = if frame_width > 0 {
        curr_frame[0].len() as u16
    } else {
        0
    };

    let renderable_width = frame_width
        .min(term_width.saturating_sub(2))
        .min(curr_frame.len() as u16);
    let renderable_height =
        frame_height
            .min(term_height.saturating_sub(2))
            .min(if curr_frame.len() > 0 {
                curr_frame[0].len() as u16
            } else {
                0
            });

    let x_offset = ((term_width.saturating_sub(frame_width + 2)) / 2).min(term_width - 1);
    let y_offset = ((term_height.saturating_sub(frame_height + 2)) / 2).min(term_height - 1);

    stdout.queue(SetForegroundColor(Color::White)).unwrap();
    if renderable_width > 0 && renderable_height > 0 {
        stdout.queue(MoveTo(x_offset, y_offset)).unwrap();
        print!("┏{}┓", "━".repeat(renderable_width as usize));

        stdout
            .queue(MoveTo(x_offset, y_offset + renderable_height + 1))
            .unwrap();
        print!("┗{}┛", "━".repeat(renderable_width as usize));

        for y in 0..renderable_height {
            stdout.queue(MoveTo(x_offset, y_offset + y + 1)).unwrap();
            print!("┃");
            stdout
                .queue(MoveTo(x_offset + renderable_width + 1, y_offset + y + 1))
                .unwrap();
            print!("┃");
        }
    }

    stdout.queue(SetBackgroundColor(Color::Black)).unwrap();
    stdout.queue(SetForegroundColor(Color::White)).unwrap();

    for x in 0..renderable_width as usize {
        for y in 0..renderable_height as usize {
            if x < curr_frame.len()
                && y < curr_frame[x].len()
                && x < last_frame.len()
                && y < last_frame[x].len()
            {
                if curr_frame[x][y] != last_frame[x][y] || force || resized {
                    stdout
                        .queue(MoveTo(x as u16 + x_offset + 1, y as u16 + y_offset + 1))
                        .unwrap();
                    print!("{}", curr_frame[x][y]);
                }
            }
        }
    }

    stdout.flush().unwrap();
}
