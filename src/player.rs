use crate::{
    difficulty::Difficulty,
    frame::{Drawable, Frame},
    invaders::Invaders,
    shot::Shot,
};
use rusty_time::Timer;
use std::time::Duration;

pub struct Player {
    x: usize,
    y: usize,
    shots: Vec<Shot>,
    fire_rate_timer: Timer,
    max_shots: usize,
}

impl Player {
    pub fn new(difficulty: &Difficulty) -> Self {
        Self {
            x: 0, // Will be centered dynamically
            y: 0, // Will be set based on frame size
            shots: Vec::new(),
            fire_rate_timer: Timer::new(difficulty.player_fire_rate),
            max_shots: difficulty.max_shots.unwrap_or(2),
        }
    }

    pub fn center(&mut self, frame: &Frame) {
        self.x = frame.len() / 2;      // Center horizontally
        self.y = frame[0].len() - 3;         // Position near the bottom
    }

    pub fn move_left(&mut self, frame: &Frame) {
        if self.x <= 1 {
            self.x = frame.len() - 2; // Wrap around
        } else {
            self.x -= 1;
        }
    }

    pub fn move_right(&mut self, frame: &Frame) {
        if self.x >= frame.len() - 2 {
            self.x = 0; // Wrap around
        } else {
            self.x += 1;
        }
    }

    pub fn shoot(&mut self) -> bool {
        if self.shots.len() < self.max_shots && self.fire_rate_timer.finished() {
            self.shots.push(Shot::new(self.x, self.y - 1));
            self.fire_rate_timer.reset();
            true
        } else {
            false
        }
    }

    pub fn update(&mut self, delta: Duration) {
        self.fire_rate_timer.tick(delta);
        for shot in self.shots.iter_mut() {
            shot.update(delta);
        }
        self.shots.retain(|shot| !shot.dead());
    }

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
        Self::new(&Difficulty::default())
    }
}

impl Drawable for Player {
    fn draw(&self, frame: &mut Frame) {
        if self.y < frame[0].len() && self.x < frame.len() {
            frame[self.x][self.y] = 'A'; // Draw the player
        }
        for shot in self.shots.iter() {
            shot.draw(frame);
        }
    }
}
