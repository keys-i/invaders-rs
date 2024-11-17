use crate::frame::{Drawable, Frame};
use rusty_time::Timer;
use std::time::Duration;

pub struct Shot {
    pub x: usize,
    pub y: usize,
    pub exploding: bool,
    timer: Timer,
}

impl Shot {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            exploding: false,
            timer: Timer::new(Duration::from_millis(50)),
        }
    }
    pub fn update(&mut self, delta: Duration) {
        self.timer.tick(delta);
        if self.timer.finished() && !self.exploding {
            if self.y > 0 {
                self.y -= 1; // Move upward
            }
            self.timer.reset();
        }
    }
    pub fn explode(&mut self) {
        self.exploding = true;
        self.timer = Timer::new(Duration::from_millis(250));
    }
    pub fn dead(&self) -> bool {
        (self.exploding && self.timer.finished()) || (self.y == 0)
    }
}

impl Drawable for Shot {
    fn draw(&self, frame: &mut Frame) {
        // Ensure shots are drawn within bounds
        if self.y < frame[0].len() && self.x < frame.len() {
            frame[self.x][self.y] = if self.exploding { '*' } else { '|' };
        }
    }
}

pub struct ShotManager {
    shots: Vec<Shot>,
    fire_rate_timer: Timer,
}

impl ShotManager {
    pub fn new() -> Self {
        Self {
            shots: Vec::new(),
            fire_rate_timer: Timer::new(Duration::from_millis(333)), // Limit to 3 shots per second
        }
    }

    pub fn try_fire_shot(&mut self, x: usize, y: usize, max_shots: usize) {
        // Check if we can fire a new shot based on the fire rate timer
        if self.shots.len() < max_shots && self.fire_rate_timer.finished() {
            self.shots.push(Shot::new(x, y));
            self.fire_rate_timer.reset();
        }
    }

    pub fn update(&mut self, delta: Duration) {
        // Update the fire rate timer
        self.fire_rate_timer.tick(delta);

        // Update each shot and remove any that are dead
        for shot in &mut self.shots {
            shot.update(delta);
        }
        self.shots.retain(|shot| !shot.dead());
    }
}

impl Drawable for ShotManager {
    fn draw(&self, frame: &mut Frame) {
        // Draw each active shot onto the frame
        for shot in &self.shots {
            shot.draw(frame);
        }
    }
}
