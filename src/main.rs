use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use rusty_audio::Audio;
use std::{
    error::Error,
    sync::mpsc::{self, Receiver},
    time::{Duration, Instant},
    {io, thread},
};

use invaders::{
    difficulty::{Difficulty, DifficultyLevel},
    frame::{self, new_frame, Drawable, Frame},
    invaders::Invaders,
    level::Level,
    menu::Menu,
    player::Player,
    render,
    score::Score,
};

fn render_screen(render_rx: Receiver<Frame>, last_size: &mut (u16, u16)) {
    let mut last_frame = frame::new_frame();
    let mut stdout = io::stdout();

    render::render(&mut stdout, &last_frame, &last_frame, true, last_size);
    while let Ok(curr_frame) = render_rx.recv() {
        render::render(&mut stdout, &last_frame, &curr_frame, false, last_size);
        last_frame = curr_frame;
    }
}

fn reset_game(
    in_menu: &mut bool,
    player: &mut Player,
    invaders: &mut Invaders,
    frame: &Frame,
    difficulty: &Difficulty,
) {
    *in_menu = true;
    *player = Player::new(difficulty); // Reapply difficulty to player
    player.center(frame);
    *invaders = Invaders::new(difficulty); // Reapply difficulty to invaders
    invaders.populate(frame);
    println!(
        "\n\n\nGame reset: Difficulty = {:?}, Invader speed = {:?}, Player fire rate = {:?}",
        difficulty,
        difficulty.invader_speed,
        difficulty.player_fire_rate
    );
}



fn run_game(
    audio: &mut Audio,
    render_tx: &mpsc::Sender<Frame>,
    last_size: &mut (u16, u16),
) -> Result<(), Box<dyn Error>> {
    let mut instant = Instant::now();
    let mut menu = Menu::new();
    let mut difficulty = Difficulty::new(DifficultyLevel::Normal); // Default difficulty
    let mut in_menu = true;

    // Initialize game entities
    let mut curr_frame = new_frame(); // Initial frame
    let mut player = Player::new(&difficulty); // Player with difficulty settings
    let mut invaders = Invaders::new(&difficulty); // Invaders with difficulty settings
    let mut score = Score::new();
    let mut level = Level::new();

    player.center(&curr_frame); // Center the player
    invaders.populate(&curr_frame); // Populate invaders

    'gameloop: loop {
        // Adjust frame dimensions if terminal size changes
        let (new_term_width, new_term_height) = crossterm::terminal::size()?;
        if new_term_width != last_size.0 || new_term_height != last_size.1 {
            curr_frame = new_frame();
            player.center(&curr_frame);
            invaders.populate(&curr_frame);
            *last_size = (new_term_width, new_term_height); // Update last known size
        }

        // Per-frame initialization
        let delta = instant.elapsed();
        instant = Instant::now();
        let mut curr_frame = new_frame();

        if in_menu {
            // Menu logic
            while event::poll(Duration::default())? {
                if let Event::Key(key_event) = event::read()? {
                    match key_event.code {
                        KeyCode::Up => menu.change_option(true),
                        KeyCode::Down => menu.change_option(false),
                        KeyCode::Left => menu.toggle_difficulty(true), // Toggle difficulty up
                        KeyCode::Right => menu.toggle_difficulty(false), // Toggle difficulty down
                        KeyCode::Char(' ') | KeyCode::Enter => {
                            if menu.selection == 0 {
                                difficulty = Difficulty::new(match menu.get_selected_difficulty() {
                                    "Easy" => DifficultyLevel::Easy,
                                    "Normal" => DifficultyLevel::Normal,
                                    "Hard" => DifficultyLevel::Hard,
                                    "Hardcore" => DifficultyLevel::Hardcore,
                                    _ => DifficultyLevel::Normal,
                                });
                                player = Player::new(&difficulty);
                                player.center(&curr_frame);
                                invaders = Invaders::new(&difficulty);
                                invaders.populate(&curr_frame);
                                in_menu = false; // Exit menu and start the game
                            } else {
                                break 'gameloop; // Exit game
                            }
                        }
                        _ => {}
                    }
                }
            }
            menu.draw(&mut curr_frame);
            let _ = render_tx.send(curr_frame);
            thread::sleep(Duration::from_millis(1));
            continue;
        }

        // Game input handling
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Left => player.move_left(&curr_frame),
                    KeyCode::Right => player.move_right(&curr_frame),
                    KeyCode::Char(' ') | KeyCode::Enter => {
                        if player.shoot() {
                            audio.play("pew");
                        }
                    }
                    KeyCode::Esc | KeyCode::Char('q') => {
                        audio.play("lose");
                        reset_game(&mut in_menu, &mut player, &mut invaders, &curr_frame, &difficulty);
                    }
                    _ => {}
                }
            }
        }

        // Updates
        player.update(delta);
        if invaders.update(delta, &curr_frame) {
            audio.play("move");
        }
        let hits = player.detect_hits(&mut invaders);
        if hits > 0 {
            audio.play("explode");
            score.add_points(hits);
        }

        // Draw and render
        let drawables: Vec<&dyn Drawable> = vec![&player, &invaders, &score, &level];
        for drawable in drawables {
            drawable.draw(&mut curr_frame);
        }
        let _ = render_tx.send(curr_frame.clone());
        thread::sleep(Duration::from_millis(1));

        // Win or lose conditions
        if invaders.all_killed() {
            if level.increment_level() {
                audio.play("win");
                break 'gameloop;
            }
            invaders.next_level(&curr_frame); // Reset invaders
        } else if invaders.reached_bottom(&curr_frame) {
            audio.play("lose");
            reset_game(&mut in_menu, &mut player, &mut invaders, &curr_frame, &difficulty);
        }
    }
    Ok(())
}


fn main() -> Result<(), Box<dyn Error>> {
    let mut audio = Audio::new();
    for item in &["explode", "lose", "move", "pew", "startup", "win"] {
        audio.add(item, format!("sounds/{}.wav", item));
    }
    audio.play("startup");

    // Terminal setup
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    // Render loop in a separate thread
    let (render_tx, render_rx) = mpsc::channel();
    let mut last_size = crossterm::terminal::size()?; // Track the initial terminal size
    let render_handle = thread::spawn(move || {
        render_screen(render_rx, &mut last_size);
    });

    let res = run_game(&mut audio, &render_tx, &mut last_size);

    // Cleanup
    drop(render_tx);
    render_handle.join().unwrap();
    audio.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    if let Err(e) = res {
        stdout.execute(crossterm::style::SetForegroundColor(
            crossterm::style::Color::Red,
        ))?;
        eprintln!("Error: {}", e);
        stdout.execute(crossterm::style::ResetColor)?;
        Err(e)
    } else {
        Ok(())
    }
}
