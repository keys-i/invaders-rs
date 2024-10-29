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

    render::render(
        &mut stdout,
        &mut last_frame,
        &mut last_frame,
        true,
        last_size,
    );
    while let Ok(curr_frame) = render_rx.recv() {
        render::render(
            &mut stdout,
            &mut last_frame,
            &mut curr_frame,
            false,
            last_size,
        );
        last_frame = curr_frame;
    }
}

fn reset_game(in_menu: &mut bool, player: &mut Player, invaders: &mut Invaders, frame: &Frame) {
    *in_menu = true;
    *player = Player::new(); // Player is reset
    player.center(frame); // Center player dynamically
    invaders.populate(frame); // Populate invaders based on the current frame
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

    // Game loop
    let mut instant = Instant::now();
    let mut player = Player::new();
    let mut invaders = Invaders::new(); // Use `new()` without frame size; use populate later
    let mut score = Score::new();
    let mut menu = Menu::new();
    let mut in_menu = true;
    let mut level = Level::new();
    let mut curr_frame = new_frame(); // Initialize the frame
    player.center(&curr_frame); // Center the player initially
    invaders.populate(&curr_frame); // Populate invaders based on the frame

    'gameloop: loop {
        // Get terminal size and check if it has changed
        let (new_term_width, new_term_height) = crossterm::terminal::size()?;
        if new_term_width != last_size.0 || new_term_height != last_size.1 {
            // Adjust frame dimensions if the terminal was resized
            last_size = (new_term_width, new_term_height);
            curr_frame = new_frame();
            player.center(&curr_frame); // Center player dynamically after resize
            invaders.populate(&curr_frame); // Re-populate invaders dynamically based on the new frame
        }

        // Per-frame init
        let delta = instant.elapsed();
        instant = Instant::now();
        let mut curr_frame = new_frame();

        if in_menu {
            // Input handlers for the menu
            while event::poll(Duration::default())? {
                if let Event::Key(key_event) = event::read()? {
                    match key_event.code {
                        KeyCode::Up => menu.change_option(true),
                        KeyCode::Down => menu.change_option(false),
                        KeyCode::Char(' ') | KeyCode::Enter => {
                            if menu.selection == 0 {
                                in_menu = false;
                            } else {
                                break 'gameloop;
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

        // Input handlers for the game
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Left => player.move_left(&curr_frame), // Adjust player position dynamically
                    KeyCode::Right => player.move_right(&curr_frame), // Adjust player position dynamically
                    KeyCode::Char(' ') | KeyCode::Enter => {
                        if player.shoot() {
                            audio.play("pew");
                        }
                    }
                    KeyCode::Esc | KeyCode::Char('q') => {
                        audio.play("lose");
                        reset_game(&mut in_menu, &mut player, &mut invaders, &curr_frame);
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
        let hits: u16 = player.detect_hits(&mut invaders);
        if hits > 0 {
            audio.play("explode");
            score.add_points(hits);
        }

        // Draw & render
        let drawables: Vec<&dyn Drawable> = vec![&player, &invaders, &score, &level];
        for drawable in drawables {
            drawable.draw(&mut curr_frame);
        }
        let _ = render_tx.send(curr_frame.clone());
        thread::sleep(Duration::from_millis(1));

        // Win or lose?
        if invaders.all_killed() {
            if level.increment_level() {
                audio.play("win");
                break 'gameloop;
            }
            invaders.next_level(&curr_frame); // Re-populate invaders after winning
        } else if invaders.reached_bottom(&curr_frame) {
            audio.play("lose");
            reset_game(&mut in_menu, &mut player, &mut invaders, &curr_frame);
        }
    }

    // Cleanup
    drop(render_tx);
    render_handle.join().unwrap();
    audio.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
