use crate::game::{Game, GameState};
use crate::image_tools::{render_image, scale_image};
use crate::wad::WadFile;
use crate::HEIGHT;
use image::{DynamicImage, GenericImageView};
use minifb::{Key, Window};
const LINE_HEIGHT: usize = 20 * crate::SCALE as usize;
const MENU_X: usize = 105 * crate::SCALE as usize;
const PADDING: usize = 10;

const TITLE_LUMP_NAME: &str = "M_DOOM";
const START_LUMP_NAME: &str = "M_NEWG";
const QUIT_LUMP_NAME: &str = "M_QUITG";
const LOAD_LUMP_NAME: &str = "M_LOADG";
const SAVE_LUMP_NAME: &str = "M_SAVEG";
const OPT_LUMP_NAME: &str = "M_OPTION";
const SELECT_EPISODE_LUMP_NAME: &str = "M_EPISOD";
const SELECT_KNEE_DEEP_LUMP_NAME: &str = "M_EPI1";
const SELECT_SHORE_LUMP_NAME: &str = "M_EPI2";
const SELECT_INFERNO_LUMP_NAME: &str = "M_EPI3";
const SELECT_SKILL_LUMP_NAME: &str = "M_SKILL";
const SELECT_NIGHTMARE_LUMP_NAME: &str = "M_NIGHT";
const SELECT_HURT_ME_LUMP_NAME: &str = "M_HURT";
const SELECT_BRING_EM_LUMP_NAME: &str = "M_BRING";
const SELECT_ULTRA_VIOLENT_LUMP_NAME: &str = "M_ULTRA";
const SELECT_TOO_YOUNG_LUMP_NAME: &str = "M_YOUNG";

struct MenuItem {
    image: DynamicImage,
    action: fn(&mut Game),
}

pub struct Menu {
    options: Vec<MenuItem>,
    selected: usize,
    title: DynamicImage,
}

impl Menu {
    fn new(options: Vec<MenuItem>, wad: &WadFile, title_lump: &str) -> Self {
        Self {
            options,
            selected: 0,
            title: wad.get_image(title_lump).map(scale_image).unwrap(),
        }
    }
}

impl Menu {
    pub fn root(wad: &WadFile) -> Self {
        Self {
            options: get_root_menu(&wad),
            title: wad.get_image(TITLE_LUMP_NAME).map(scale_image).unwrap(),
            selected: 0,
        }
    }

    pub fn handle_input(window: &Window, game: &mut Game) {
        if window.is_key_down(Key::Down) {
            game.menu.selected = (game.menu.selected + 1) % game.menu.options.len();
        }
        if window.is_key_down(Key::Up) {
            game.menu.selected = match game.menu.selected > 0 {
                true => game.menu.selected - 1,
                false => game.menu.options.len() - 1,
            };
        }
        if window.is_key_down(Key::Enter) {
            let action = game.menu.options[game.menu.selected].action;
            action(game);
        }
    }

    pub fn render(&self, buffer: &mut [u32], game: &Game) {
        // Copy the image's pixels into the buffer
        render_image(&game.background, 0, 0, buffer);

        // Render the logo
        let start_y = (HEIGHT - self.title.height() as usize) / 4;
        render_image(&self.title, MENU_X, start_y, buffer);

        let mut start_y = start_y + self.title.height() as usize + PADDING;
        let max_width = self
            .options
            .iter()
            .map(|option| option.image.width())
            .max()
            .unwrap_or(100);
        let x = crate::WIDTH / 2 - max_width as usize / 2;
        for (i, option) in self.options.iter().enumerate() {
            let y = start_y;
            let skull_x = x - game.skull.width() as usize - PADDING;
            // render the skull if this is the selected option
            if i == self.selected {
                render_image(&game.skull, skull_x, y, buffer);
            }
            render_image(&option.image, x, y, buffer);
            start_y += LINE_HEIGHT;
        }
    }
}

fn get_root_menu(wad: &WadFile) -> Vec<MenuItem> {
    vec![
        MenuItem {
            image: wad.get_image(START_LUMP_NAME).map(scale_image).unwrap(),
            action: |game| game.menu.options = get_episode_menu(&game.wad),
        },
        MenuItem {
            image: wad.get_image(OPT_LUMP_NAME).map(scale_image).unwrap(),
            action: |game| game.set_state(GameState::Playing),
        },
        MenuItem {
            image: wad.get_image(LOAD_LUMP_NAME).map(scale_image).unwrap(),
            action: |game| game.set_state(GameState::Playing),
        },
        MenuItem {
            image: wad.get_image(SAVE_LUMP_NAME).map(scale_image).unwrap(),
            action: |game| game.set_state(GameState::Playing),
        },
        MenuItem {
            image: wad.get_image(QUIT_LUMP_NAME).map(scale_image).unwrap(),
            action: |game| game.set_state(GameState::Playing),
        },
    ]
}

fn get_episode_menu(wad: &WadFile) -> Vec<MenuItem> {
    vec![
        MenuItem {
            image: wad
                .get_image(SELECT_KNEE_DEEP_LUMP_NAME)
                .map(scale_image)
                .unwrap(),
            action: |game| game.set_state(GameState::Playing),
        },
        MenuItem {
            image: wad
                .get_image(SELECT_SHORE_LUMP_NAME)
                .map(scale_image)
                .unwrap(),
            action: |game| game.set_state(GameState::Playing),
        },
        MenuItem {
            image: wad
                .get_image(SELECT_INFERNO_LUMP_NAME)
                .map(scale_image)
                .unwrap(),
            action: |game| game.set_state(GameState::Playing),
        },
    ]
}
