use crate::game::{Episode, Game, GameState, Skill};
use crate::image_tools::render_image;
use crate::wad::WadFile;
use crate::HEIGHT;
use image::DynamicImage;
use minifb::{Key, Window};
const LINE_HEIGHT: usize = 15;
const PADDING: usize = 3;

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
const SELECT_TOO_YOUNG_LUMP_NAME: &str = "M_JKILL";
const SELECT_HEY_NOT_TOO_ROUGH_LUMP_NAME: &str = "M_ROUGH";
const SELECT_HURT_ME_LUMP_NAME: &str = "M_HURT";
const SELECT_ULTRA_VIOLENT_LUMP_NAME: &str = "M_ULTRA";
const SELECT_NIGHTMARE_LUMP_NAME: &str = "M_NMARE";

const INPUT_COOLDOWN: std::time::Duration = std::time::Duration::from_millis(200); // Change to your needs

/// Defines the menu options for a menu
struct MenuItem {
    /// The image resource from the WAD file to display for this menu item
    image: DynamicImage,
    /// The action to take when this menu item is selected
    action: fn(&mut Game),
}

enum MenuType {
    Root,
    Episode,
    Skill,
    Load,
    Save,
    Options,
}

/// Defined a menu
pub struct Menu {
    menu_type: MenuType,
    options: Vec<MenuItem>,
    selected: usize,
    title: DynamicImage,
    last_input: std::time::Instant,
}

impl Menu {
    /// Creates a new menu root menu containing the start, load, options, and quit options
    pub fn root(wad: &WadFile) -> Self {
        Self {
            menu_type: MenuType::Root,
            options: get_root_menu(&wad),
            title: wad.get_image(TITLE_LUMP_NAME).unwrap(),
            selected: 0,
            last_input: std::time::Instant::now(),
        }
    }

    /// Handles input for the menu
    /// # Arguments
    /// * `window` - The window to handle input for
    /// * `game` - The game to modify based on input
    pub fn handle_input(window: &Window, game: &mut Game) {
        if window.get_keys().is_empty()
            || std::time::Instant::now().duration_since(game.menu.last_input) < INPUT_COOLDOWN
        {
            return;
        }
        game.menu.last_input = std::time::Instant::now();
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
        if window.is_key_down(Key::Escape) {
            match game.menu.menu_type {
                MenuType::Root => game.set_state(GameState::Playing),
                MenuType::Episode => set_root_menu(game),
                MenuType::Skill => set_episode_menu(game),
                MenuType::Load => set_root_menu(game),
                MenuType::Save => set_root_menu(game),
                MenuType::Options => set_root_menu(game),
            }
        }
    }

    /// Renders the menu to the buffer
    /// # Arguments
    /// * `buffer` - The buffer to render to
    /// * `game` - The game to render
    pub fn render(&self, buffer: &mut [u32], game: &Game) {
        // Copy the image's pixels into the buffer
        render_image(&game.background, 0, 0, buffer);

        // Render the logo
        let start_y = (HEIGHT - self.title.height() as usize) / 4;
        let x = crate::WIDTH / 2 - self.title.width() as usize / 2;
        render_image(&self.title, x, start_y, buffer);

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

fn set_root_menu(game: &mut Game) {
    game.menu.options = get_root_menu(&game.wad);
    game.menu.title = game.wad.get_image(TITLE_LUMP_NAME).unwrap();
    game.menu.menu_type = MenuType::Root;
    game.menu.selected = 0;
}

/// Gets the root menu items (start, load, options, quit)
fn get_root_menu(wad: &WadFile) -> Vec<MenuItem> {
    vec![
        MenuItem {
            image: wad.get_image(START_LUMP_NAME).unwrap(),
            action: |game| set_episode_menu(game),
        },
        MenuItem {
            image: wad.get_image(OPT_LUMP_NAME).unwrap(),
            action: |game| game.set_state(GameState::Playing),
        },
        MenuItem {
            image: wad.get_image(LOAD_LUMP_NAME).unwrap(),
            action: |game| game.set_state(GameState::Playing),
        },
        MenuItem {
            image: wad.get_image(SAVE_LUMP_NAME).unwrap(),
            action: |game| game.set_state(GameState::Playing),
        },
        MenuItem {
            image: wad.get_image(QUIT_LUMP_NAME).unwrap(),
            action: |game| game.set_state(GameState::Quit),
        },
    ]
}

fn set_episode_menu(game: &mut Game) {
    game.menu.options = vec![
        MenuItem {
            image: game.wad.get_image(SELECT_KNEE_DEEP_LUMP_NAME).unwrap(),
            action: |game| {
                set_skill_menu(game);
                game.set_episode(Episode::KneeDeep);
            },
        },
        MenuItem {
            image: game.wad.get_image(SELECT_SHORE_LUMP_NAME).unwrap(),
            action: |game| {
                set_skill_menu(game);
                game.set_episode(Episode::Shores);
            },
        },
        MenuItem {
            image: game.wad.get_image(SELECT_INFERNO_LUMP_NAME).unwrap(),
            action: |game| {
                set_skill_menu(game);
                game.set_episode(Episode::Inferno);
            },
        },
    ];
    game.menu.selected = 0;
    game.menu.title = game.wad.get_image(SELECT_EPISODE_LUMP_NAME).unwrap();
    game.menu.menu_type = MenuType::Episode;
}

fn set_skill_menu(game: &mut Game) {
    game.menu.options = vec![
        MenuItem {
            image: game.wad.get_image(SELECT_TOO_YOUNG_LUMP_NAME).unwrap(),
            action: |game| {
                game.set_skill(Skill::TooYoungToDie);
                game.set_state(GameState::Playing);
            },
        },
        MenuItem {
            image: game
                .wad
                .get_image(SELECT_HEY_NOT_TOO_ROUGH_LUMP_NAME)
                .unwrap(),
            action: |game| {
                game.set_skill(Skill::HeyNotTooRough);
                game.set_state(GameState::Playing);
            },
        },
        MenuItem {
            image: game.wad.get_image(SELECT_HURT_ME_LUMP_NAME).unwrap(),
            action: |game| {
                game.set_skill(Skill::HurtMePlenty);
                game.set_state(GameState::Playing);
            },
        },
        MenuItem {
            image: game.wad.get_image(SELECT_ULTRA_VIOLENT_LUMP_NAME).unwrap(),
            action: |game| {
                game.set_skill(Skill::UltraViolence);
                game.set_state(GameState::Playing);
            },
        },
        MenuItem {
            image: game.wad.get_image(SELECT_NIGHTMARE_LUMP_NAME).unwrap(),
            action: |game| {
                game.set_skill(Skill::Nightmare);
                game.set_state(GameState::Playing);
            },
        },
    ];
    game.menu.selected = 0;
    game.menu.menu_type = MenuType::Skill;
    game.menu.title = game.wad.get_image(SELECT_SKILL_LUMP_NAME).unwrap();
}
