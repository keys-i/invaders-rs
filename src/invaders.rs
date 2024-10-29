use crate::frame::{Drawable, Frame};
use rusty_time::Timer;
use std::{cmp::max, time::Duration};

pub struct Invader {
    pub x: usize,
    pub y: usize,
    points: u16,
    pub is_visible: bool, // Control visibility for the pop animation
}

pub struct Invaders {
    pub army: Vec<Invader>,
    pub total_count: usize,
    move_timer: Timer,
    pop_timer: Timer, // Timer to control the pop animation
    direction: i32,
    level: u16,             // Add level to track which level we're on
    invaders_popped: usize, // Track how many invaders have been made visible during pop animation
}

impl Invaders {
    // Initialize invaders based on the frame size (no need to store frame dimensions)
    pub fn new() -> Self {
        Self {
            army: Vec::new(),
            total_count: 0,
            move_timer: Timer::new(Duration::from_millis(2000)),
            pop_timer: Timer::new(Duration::from_millis(200)), // Pop interval
            direction: 1,
            level: 1, // Start at level 1
            invaders_popped: 0,
        }
    }

    // Calculate the Fibonacci number for the given level
    fn series(n: u16) -> u16 {
        match n {
            1 => 3,
            2 => 5,
            _ => {
                let mut a = 3;
                let mut b = 5;
                for _ in 3..=n {
                    let tmp = a + b;
                    a = b;
                    b = tmp;
                }
                b
            }
        }
    }

    // Populate invaders dynamically based on the current level and frame size
    pub fn populate(&mut self, frame: &Frame) {
        let frame_width = frame[0].len();
        let frame_height = frame.len();

        let num_invaders = Invaders::series(self.level) as usize; // Use the Fibonacci number directly

        self.army.clear();
        self.invaders_popped = 0;

        // Determine number of rows based on the number of invaders
        let num_rows = if num_invaders >= 8 {
            4
        } else if num_invaders >= 4 {
            2
        } else {
            1
        };

        let num_cols = (num_invaders + num_rows - 1) / num_rows; // Ceiling division

        // Calculate spacing between invaders
        let x_spacing = if num_cols > 1 {
            (frame_width - 4) / (num_cols - 1)
        } else {
            (frame_width - 4) / 2
        };
        let y_spacing = 2;
        let y_positions: Vec<usize> = (1..frame_height / 2)
            .step_by(y_spacing)
            .take(num_rows)
            .collect();

        let mut invader_count = 0;
        for col in 0..num_cols {
            let x_position = 2 + col * x_spacing;
            for &y in &y_positions {
                if invader_count >= num_invaders {
                    break;
                }
                self.army.push(Invader {
                    x: x_position,
                    y,
                    points: 1,
                    is_visible: false,
                });
                invader_count += 1;
            }
            if invader_count >= num_invaders {
                break;
            }
        }

        self.total_count = self.army.len();
    }

    // Update invaders based on time elapsed and frame size
    pub fn update(&mut self, delta: Duration, frame: &Frame) -> bool {
        // Handle the pop animation by revealing invaders gradually
        self.pop_timer.tick(delta);
        if self.pop_timer.finished() {
            self.pop_timer.reset();
            if self.invaders_popped < self.army.len() {
                self.army[self.invaders_popped].is_visible = true;
                self.invaders_popped += 1;
            }
        }

        // Handle movement
        self.move_timer.tick(delta);
        if self.move_timer.finished() {
            self.move_timer.reset();
            let mut downwards = false;
            let frame_width = frame[0].len();

            if self.direction == -1 {
                let min_x = self.army.iter().map(|invader| invader.x).min().unwrap_or(0);
                if min_x == 1 {
                    self.direction = 1;
                    downwards = true;
                }
            } else {
                let max_x = self.army.iter().map(|invader| invader.x).max().unwrap_or(0);
                if max_x == frame_width - 2 {
                    self.direction = -1;
                    downwards = true;
                }
            }

            if downwards {
                let new_duration = max(self.move_timer.duration().as_millis() - 250, 250);
                self.move_timer
                    .set_duration(Duration::from_millis(new_duration as u64));
                for invader in self.army.iter_mut() {
                    invader.y += 1;
                }
            } else {
                for invader in self.army.iter_mut() {
                    invader.x = ((invader.x as i32) + self.direction) as usize;
                }
            }
            return true;
        }
        false
    }

    // Check if all invaders are killed
    pub fn all_killed(&self) -> bool {
        self.army.is_empty()
    }

    // Check if any invader has reached the bottom of the frame
    pub fn reached_bottom(&self, frame: &Frame) -> bool {
        let frame_height = frame[0].len();
        self.army
            .iter()
            .any(|invader| invader.y >= frame_height - 1)
    }

    // Kill an invader at a specific position
    pub fn kill_invader_at(&mut self, x: usize, y: usize) -> u16 {
        if let Some(idx) = self
            .army
            .iter()
            .position(|invader| (invader.x == x) && (invader.y == y))
        {
            let points = self.army[idx].points;
            self.army.remove(idx);
            points
        } else {
            0
        }
    }

    // Increment the level and repopulate invaders for the new level
    pub fn next_level(&mut self, frame: &Frame) {
        self.level += 1; // Move to the next level
        self.populate(frame); // Repopulate invaders based on the new level
    }
}

impl Default for Invaders {
    fn default() -> Self {
        Self::new()
    }
}

impl Drawable for Invaders {
    fn draw(&self, frame: &mut Frame) {
        for invader in self.army.iter() {
            if invader.is_visible {
                frame[invader.x][invader.y] = if (self.move_timer.remaining().as_secs_f32()
                    / self.move_timer.duration().as_secs_f32())
                    > 0.5
                {
                    'x'
                } else {
                    '+'
                };
            }
        }
    }
}
