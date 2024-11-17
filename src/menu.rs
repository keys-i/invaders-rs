use crate::frame::{Drawable, Frame};
pub struct Menu {
    pub options: Vec<String>,
    pub selection: usize,
    pub difficulty_levels: Vec<String>,
    pub current_difficulty: usize,
}

impl Menu {
    pub fn new() -> Self {
        Self {
            options: vec![String::from("New game"), String::from("Exit")],
            selection: 0,
            difficulty_levels: vec![
                String::from("Easy"),
                String::from("Normal"),
                String::from("Hard"),
                String::from("Hardcore"),
            ],
            current_difficulty: 1, // Default to "Normal"
        }
    }

    pub fn change_option(&mut self, upwards: bool) {
        if upwards && self.selection > 0 {
            self.selection -= 1;
        } else if !upwards && self.selection < self.options.len() - 1 {
            self.selection += 1;
        }
    }

    pub fn toggle_difficulty(&mut self, upwards: bool) {
        if upwards && self.current_difficulty > 0 {
            self.current_difficulty -= 1;
        } else if !upwards && self.current_difficulty < self.difficulty_levels.len() - 1 {
            self.current_difficulty += 1;
        }
    }

    pub fn get_selected_difficulty(&self) -> &str {
        &self.difficulty_levels[self.current_difficulty]
    }
}

impl Default for Menu {
    fn default() -> Self {
        Self::new()
    }
}

// Reuse Frame grid to print the menu options
impl Drawable for Menu {
    fn draw(&self, frame: &mut Frame) {
        let menu_start_y = 7; // Where the menu starts (adjusted to fit multiline title)
        let difficulty_start_y = menu_start_y + 1; // Place difficulty just above the menu
        let title_start_y = menu_start_y - 7; // Multiline title starts higher

        // Render multiline title
        let title = vec![
           "  ______                               __                          ",
           " /\\__  _\\                             /\\ \\                         ",
           " \\/_/\\ \\/     ___   __  __     __     \\_\\ \\     __   _ __   ____   ",
           "    \\ \\ \\   /' _ `\\/\\ \\/\\ \\  /'__`\\   /'_` \\  /'__`\\/\\`'__\\/',__\\  ",
           "     \\_\\ \\__/\\ \\/\\ \\ \\ \\_/ |/\\ \\L\\.\\_/\\ \\L\\ \\/\\  __/\\ \\ \\//\\__, `\\ ",
           "     /\\_____\\ \\_\\ \\_\\ \\___/ \\ \\__/.\\_\\ \\___,_\\ \\____\\ \\_\\/\\____/ ",
           "     \\/_____/\\/_/\\/_/\\/__/   \\/__/\\/_/\\/__,_ /\\/____/ \\/_/ \\/___/  ",
           "                                                                  ",
        ];
        let frame_width = frame[0].len();
        for (line_index, line) in title.iter().enumerate() {
            let line_start_x = frame_width.saturating_sub(line.len()); // Center each line
            for (i, c) in line.chars().enumerate() {
                frame[line_start_x + i][title_start_y + line_index] = c;
            }
        }

        // Render difficulty
        let difficulty_label = " Difficulty:";
        for (i, c) in difficulty_label.chars().enumerate() {
            frame[i][difficulty_start_y] = c;
        }
        let difficulty_level = self.get_selected_difficulty();
        for (i, c) in difficulty_level.chars().enumerate() {
            frame[difficulty_label.len() + 1 + i][difficulty_start_y] = c;
        }

        // Render menu options
        for (index, option) in self.options.iter().enumerate() {
            if index == self.selection {
                frame[0][menu_start_y + index * 2] = '>';
            }
            for (i, c) in option.chars().enumerate() {
                frame[i + 1][menu_start_y + index * 2] = c;
            }
        }
    }
}
