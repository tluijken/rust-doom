use menu::Menu;
use minifb::{Key, Window, WindowOptions};

pub const WIDTH: usize = 320;
pub const HEIGHT: usize = 200;
const WAD_FILE: &str = "doom1.wad";

mod game;
use game::{Game, GameState};
mod audio;
mod image_tools;
mod menu;
mod wad;

/// The main function
fn main() {
    let mut window = Window::new(
        "Doom",
        WIDTH,
        HEIGHT,
        WindowOptions {
            borderless: false,
            title: true,
            resize: false,
            scale: minifb::Scale::X4,
            scale_mode: minifb::ScaleMode::AspectRatioStretch,
            topmost: false,
            transparency: false,
            none: false,
        },
    )
    .unwrap_or_else(|e| {
        panic!("Unable to start new window: {}", e);
    });

    let wad = wad::WadFile::load(WAD_FILE);
    let mut game = game::Game::new(wad);
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    while window.is_open() && game.state != GameState::Quit {
        update_game_state(&mut game, &mut window);
        render_game_state(&mut buffer, &game);
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}

/// Updates the game state based on user input
fn update_game_state(game: &mut Game, window: &mut Window) {
    match game.state {
        GameState::Menu => {
            Menu::handle_input(window, game);
        }
        GameState::Playing => {
            if window.is_key_down(Key::Escape) {
                game.set_state(GameState::Menu);
            }
        }
        GameState::GameOver => {
            if window.is_key_down(Key::Space) {
                game.set_state(GameState::GameOver);
            }
        }
        GameState::Quit => {}
    }
}

/// Renders the game state to the screen buffer
fn render_game_state(buffer: &mut [u32], game: &Game) {
    // Clear the buffer
    buffer.iter_mut().for_each(|pixel| *pixel = 0);
    match game.state {
        GameState::Menu => {
            game.menu.render(buffer, game);
        }
        GameState::Playing => {
            // Render gameplay graphics
        }
        GameState::GameOver => {
            // Render game over graphics
        }
        GameState::Quit => {}
    }
}
