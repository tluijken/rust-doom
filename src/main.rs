use minifb::{Key, Window, WindowOptions};

pub const WIDTH: usize = 1024;
pub const HEIGHT: usize = 768;

mod game_state;
use game_state::GameState;
mod menu;
use menu::Menu;
const WAD_FILE: &str = "wads/doom2.wad";
mod directory;

mod wad_parser;

fn main() {
    let mut window =
        Window::new("Doom", WIDTH, HEIGHT, WindowOptions::default()).unwrap_or_else(|e| {
            panic!("{}", e);
        });

    let mut menu = Menu::new();
    let mut game_state = GameState::Menu;
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    while window.is_open() && game_state != GameState::Quit {
        window.update();

        update_game_state(&mut game_state, &mut window, &mut menu);

        render_game_state(&mut buffer, &game_state, &menu);
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}

fn update_game_state(game_state: &mut GameState, window: &mut Window, menu: &mut Menu) {
    match game_state {
        GameState::Menu => {
            menu.handle_input(window, game_state);
        }
        GameState::Playing => {
            if window.is_key_down(Key::Escape) {
                *game_state = GameState::Menu;
            }
        }
        GameState::GameOver => {
            if window.is_key_down(Key::Space) {
                *game_state = GameState::Playing;
            }
        }
        GameState::Quit => {}
    }
}

fn render_game_state(buffer: &mut [u32], game_state: &GameState, menu: &Menu) {
    buffer.iter_mut().for_each(|pixel| *pixel = 0);
    match game_state {
        GameState::Menu => {
            menu.render(buffer);
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
