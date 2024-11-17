use crate::{difficulty::Difficulty, frame::{Drawable, Frame}};
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
    pub shots_fired: u32,
}

impl Invaders {
    // Initialize invaders based on the frame size (no need to store frame dimensions)
    pub fn new(difficulty: &Difficulty) -> Self {
        Self {
            army: Vec::new(),
            total_count: 0,
            move_timer: Timer::new(difficulty.invader_speed),
            pop_timer: Timer::new(Duration::from_millis(200)), // Pop interval
            direction: 1,
            level: 1, // Start at level 1
            invaders_popped: 0,
            shots_fired: 0,
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
        let frame_width = frame.len();
        let frame_height = frame[0].len();

        let num_invaders = Invaders::series(self.level) as usize; // Use the Fibonacci number directly

        self.army.clear();
        self.invaders_popped = 0;

        let x_spacing = 3; // Space between invaders horizontally
        let y_spacing = 2; // Space between invaders vertically

        // Calculate how many invaders can fit in one row based on frame width
        let invaders_per_row = (frame_width - 4) / x_spacing;

        let mut invader_count = 0;
        for row in 0.. {
            let y_position = 2 + row * y_spacing;

            // Stop if we've reached beyond the available height in the frame
            if y_position >= frame_height / 2 {
                break;
            }

            for col in 0..invaders_per_row {
                if invader_count >= num_invaders {
                    break; // Stop if we have placed all invaders
                }

                let x_position = 2 + col * x_spacing;

                self.army.push(Invader {
                    x: x_position,
                    y: y_position,
                    points: 1,
                    is_visible: false,
                });

                invader_count += 1;
            }

            // Break out of the loop once all invaders are placed
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
            let frame_width = frame.len();

            if self.direction == -1 {
                let min_x = self.army.iter().map(|invader| invader.x).min().unwrap_or(0);
                if min_x == 1 {
                    self.direction = 1;
                    downwards = true;
                }
            } else {
                let max_x = self.army.iter().map(|invader| invader.x).max().unwrap_or(0);
                if max_x >= frame_width - 2 {
                    self.direction = -1;
                    downwards = true;
                }
            }

            if downwards {
                let new_duration = self.calculate_speed();
                self.move_timer
                    .set_duration(Duration::from_millis(new_duration));
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

    // Calculate new speed based on level and shots fired (for levels > 10)
    fn calculate_speed(&self) -> u64 {
        if self.level > 10 {
            let base_speed = max(1000 - (self.level as u64 * 50), 100); // Base speed decreases with level
            let shots_speed_increase = self.shots_fired as u64 * 10; // Speed up with more shots fired
            max(base_speed - shots_speed_increase, 100)
        } else {
            max(2000 - (self.level as u64 * 200), 500)
        }
    }

    // Track shots fired by the player
    pub fn record_shot(&mut self) {
        self.shots_fired += 1;
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
        self.shots_fired = 0;
        self.populate(frame); // Repopulate invaders based on the new level
    }
}

impl Default for Invaders {
    fn default() -> Self {
        Self::new(&Difficulty::default())
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
