use menu::Menu;
use minifb::{Key, Window, WindowOptions};

pub const WIDTH: usize = 1024;
pub const HEIGHT: usize = 768;
// the original Doom resolution is 320x200, so we need to scale the graphics up
const SCALE: f32 = WIDTH as f32 / 320 as f32;
// TEMPORARY: hard-coded the WAD file name
const WAD_FILE: &str = "doom1.wad";

mod game;
use game::{Game, GameState};
mod image_tools;
mod menu;
mod wad;

fn main() {
    let mut window =
        Window::new("Doom", WIDTH, HEIGHT, WindowOptions::default()).unwrap_or_else(|e| {
            panic!("Unable to start new window: {}", e);
        });

    let wad = wad::WadFile::load(WAD_FILE);
    let mut game = game::Game::new(wad);
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    while window.is_open() && game.state != GameState::Quit {
        // reset all pixels to black
        buffer.iter_mut().for_each(|pixel| *pixel = 0);
        update_game_state(&mut game, &mut window);
        render_game_state(&mut buffer, &game);
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
        std::thread::sleep(std::time::Duration::from_micros(16_666)); // Approx 60 FPS
    }
}

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

fn render_game_state(buffer: &mut [u32], game: &Game) {
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
