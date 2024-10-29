use crate::{
    frame::{Drawable, Frame},
    invaders::Invaders,
    shot::Shot,
};
use std::time::Duration;

pub struct Player {
    x: usize,
    y: usize,
    shots: Vec<Shot>,
}

impl Player {
    // Initialize player position to zero; will be centered later based on frame size
    pub fn new() -> Self {
        Self {
            x: 0, // Will be centered dynamically based on frame size
            y: 0, // Will be adjusted to bottom of the frame dynamically
            shots: Vec::new(),
        }
    }

    // Center the player based on the current frame size
    pub fn center(&mut self, frame: &Frame) {
        self.x = frame.len() / 2; // Center horizontally
        self.y = frame[0].len() / 3 * 2; // Position near the bottom of the frame
    }

    // Move player left based on the current frame width
    pub fn move_left(&mut self, frame: &Frame) {
        if self.x <= 1 {
            self.x = frame.len() - 2; // Wrap around to the right
        } else {
            self.x -= 1;
        }
    }

    // Move player right based on the current frame width
    pub fn move_right(&mut self, frame: &Frame) {
        if self.x >= frame.len() - 2 {
            self.x = 0; // Wrap around to the left
        } else {
            self.x += 1;
        }
    }

    // Player shoots a shot (only 2 shots allowed at a time)
    pub fn shoot(&mut self) -> bool {
        if self.shots.len() < 2 {
            self.shots.push(Shot::new(self.x, self.y - 2)); // Shot appears above player
            true
        } else {
            false
        }
    }

    // Update the player's shots
    pub fn update(&mut self, delta: Duration) {
        for shot in self.shots.iter_mut() {
            shot.update(delta);
        }
        self.shots.retain(|shot| !shot.dead());
    }

    // Detect collisions with invaders and handle explosions
    pub fn detect_hits(&mut self, invaders: &mut Invaders) -> u16 {
        let mut hit_something = 0u16;
        for shot in self.shots.iter_mut() {
            if !shot.exploding {
                let hit_count = invaders.kill_invader_at(shot.x, shot.y);
                if hit_count > 0 {
                    hit_something += hit_count;
                    shot.explode();
                }
            }
        }
        hit_something
    }
}

impl Default for Player {
    fn default() -> Self {
        Self::new()
    }
}

impl Drawable for Player {
    fn draw(&self, frame: &mut Frame) {
        // Ensure the player stays within the frame and doesn't overlap with the bottom border
        if self.x < frame.len() && self.y < frame[0].len() - 2 {
            frame[self.x][self.y] = 'A'; // Draw the player
        }
        // Draw all shots
        for shot in self.shots.iter() {
            shot.draw(frame); // Draw the shots
        }
    }
}
