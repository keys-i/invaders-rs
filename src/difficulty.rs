use std::time::Duration;

pub enum DifficultyLevel {
    Easy,
    Normal,
    Hard,
    Hardcore,
}

#[derive(Debug)]
pub struct Difficulty {
    pub invader_speed: Duration,
    pub player_fire_rate: Duration,
    pub max_shots: Option<usize>,
}

impl Difficulty {
    pub fn new(level: DifficultyLevel) -> Self {
        match level {
            DifficultyLevel::Easy => Self {
                invader_speed: Duration::from_millis(800), // Slower invaders
                player_fire_rate: Duration::from_millis(400), // Faster fire rate
                max_shots: Some(3), // More shots allowed
            },
            DifficultyLevel::Normal => Self {
                invader_speed: Duration::from_millis(600), // Moderate invader speed
                player_fire_rate: Duration::from_millis(500), // Balanced fire rate
                max_shots: Some(2), // Default shot limit
            },
            DifficultyLevel::Hard => Self {
                invader_speed: Duration::from_millis(400), // Faster invaders
                player_fire_rate: Duration::from_millis(600), // Slower fire rate
                max_shots: Some(2), // Default shot limit
            },
            DifficultyLevel::Hardcore => Self {
                invader_speed: Duration::from_millis(100), // Very fast invaders
                player_fire_rate: Duration::from_millis(200000), // Slow fire rate
                max_shots: Some(1), // Only one shot allowed at a time
            },
        }
    }
}

impl Default for Difficulty {
    fn default() -> Self {
        Self::new(DifficultyLevel::Normal)
    }
}