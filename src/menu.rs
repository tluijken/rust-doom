use crate::game::{Game, GameState, Skill};
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
const SELECT_EPISODE_PREFIX: &str = "M_EPI";

const SELECT_SKILL_LUMP_NAME: &str = "M_SKILL";
const SELECT_TOO_YOUNG_LUMP_NAME: &str = "M_JKILL";
const SELECT_HEY_NOT_TOO_ROUGH_LUMP_NAME: &str = "M_ROUGH";
const SELECT_HURT_ME_LUMP_NAME: &str = "M_HURT";
const SELECT_ULTRA_VIOLENT_LUMP_NAME: &str = "M_ULTRA";
const SELECT_NIGHTMARE_LUMP_NAME: &str = "M_NMARE";

const SHOTGUN_LUMP_NAME: &str = "DSPISTOL";

const SELECT_LOAD_LUMP_NAME: &str = "M_LOADG";
const SELECT_SAVE_LUMP_NAME: &str = "M_SAVEG";

// the option menu items
const OPT_MESSAGES_LUMP_NAME: &str = "M_MESSG";
const OPT_DETAIL_LUMP_NAME: &str = "M_DETAIL";
const OPT_SCREEN_SIZE_LUMP_NAME: &str = "M_SCRNSZ";
const OPT_MOUSE_SENSITIVITY_LUMP_NAME: &str = "M_MSENS";
const OPT_SOUND_VOLUME_LUMP_NAME: &str = "M_SVOL";

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

impl MenuItem {
    /// Creates a new menu item
    pub fn new(image: DynamicImage, action: fn(&mut Game)) -> Self {
        Self { image, action }
    }
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

    pub fn set_menu_type(&mut self, menu_type: MenuType) -> &mut Self {
        self.menu_type = menu_type;
        self
    }

    pub fn set_options(&mut self, options: Vec<MenuItem>) -> &mut Self {
        self.options = options;
        self
    }

    pub fn set_selected(&mut self, selected: usize) -> &mut Self {
        self.selected = selected;
        self
    }

    pub fn set_title(&mut self, title: DynamicImage) -> &mut Self {
        self.title = title;
        self
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
            let shot_gun_lump = game
                .wad
                .get_lump(SHOTGUN_LUMP_NAME)
                .expect("Shotgun lump not found");
            crate::audio::play_sound((&shot_gun_lump).to_vec());
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
    game.menu
        .set_options(get_root_menu(&game.wad))
        .set_menu_type(MenuType::Root)
        .set_selected(0)
        .set_title(game.wad.get_image(TITLE_LUMP_NAME).unwrap());
}

/// Gets the root menu items (start, load, options, quit)
fn get_root_menu(wad: &WadFile) -> Vec<MenuItem> {
    vec![
        MenuItem::new(wad.get_image(START_LUMP_NAME).unwrap(), |game| {
            set_episode_menu(game)
        }),
        MenuItem::new(wad.get_image(OPT_LUMP_NAME).unwrap(), |game| {
            set_options_menu(game)
        }),
        MenuItem::new(wad.get_image(LOAD_LUMP_NAME).unwrap(), |game| {
            game.set_state(GameState::Playing)
        }),
        MenuItem::new(wad.get_image(SAVE_LUMP_NAME).unwrap(), |game| {
            game.set_state(GameState::Playing)
        }),
        MenuItem::new(wad.get_image(QUIT_LUMP_NAME).unwrap(), |game| {
            game.set_state(GameState::Quit)
        }),
    ]
}

fn set_episode_menu(game: &mut Game) {
    game.menu
        .set_options(vec![
            MenuItem::new(
                game.wad
                    .get_image(format!("{}{}", SELECT_EPISODE_PREFIX, 1).as_str())
                    .unwrap(),
                |game| {
                    set_skill_menu(game);
                    game.set_episode(1);
                },
            ),
            MenuItem::new(
                game.wad
                    .get_image(format!("{}{}", SELECT_EPISODE_PREFIX, 2).as_str())
                    .unwrap(),
                |game| {
                    set_skill_menu(game);
                    game.set_episode(2);
                },
            ),
            MenuItem::new(
                game.wad
                    .get_image(format!("{}{}", SELECT_EPISODE_PREFIX, 3).as_str())
                    .unwrap(),
                |game| {
                    set_skill_menu(game);
                    game.set_episode(2);
                },
            ),
        ])
        .set_selected(0)
        .set_menu_type(MenuType::Episode)
        .set_title(game.wad.get_image(SELECT_EPISODE_LUMP_NAME).unwrap());
}

fn set_options_menu(game: &mut Game) {
    game.menu
        .set_options(vec![
            MenuItem::new(game.wad.get_image(OPT_DETAIL_LUMP_NAME).unwrap(), |game| {
                game.set_state(GameState::Playing);
            }),
            MenuItem::new(
                game.wad.get_image(OPT_SCREEN_SIZE_LUMP_NAME).unwrap(),
                |game| {
                    game.set_state(GameState::Playing);
                },
            ),
            MenuItem::new(
                game.wad.get_image(OPT_MOUSE_SENSITIVITY_LUMP_NAME).unwrap(),
                |game| {
                    game.set_state(GameState::Playing);
                },
            ),
            MenuItem::new(
                game.wad.get_image(OPT_SOUND_VOLUME_LUMP_NAME).unwrap(),
                |game| {
                    game.set_state(GameState::Playing);
                },
            ),
        ])
        .set_selected(0)
        .set_menu_type(MenuType::Options)
        .set_title(game.wad.get_image(OPT_LUMP_NAME).unwrap());
}

fn set_skill_menu(game: &mut Game) {
    game.menu
        .set_options(vec![
            MenuItem::new(
                game.wad.get_image(SELECT_TOO_YOUNG_LUMP_NAME).unwrap(),
                |game| {
                    game.set_skill(Skill::TooYoungToDie);
                    game.set_state(GameState::Playing);
                },
            ),
            MenuItem::new(
                game.wad
                    .get_image(SELECT_HEY_NOT_TOO_ROUGH_LUMP_NAME)
                    .unwrap(),
                |game| {
                    game.set_skill(Skill::HeyNotTooRough);
                    game.set_state(GameState::Playing);
                },
            ),
            MenuItem::new(
                game.wad.get_image(SELECT_HURT_ME_LUMP_NAME).unwrap(),
                |game| {
                    game.set_skill(Skill::HurtMePlenty);
                    game.set_state(GameState::Playing);
                },
            ),
            MenuItem::new(
                game.wad.get_image(SELECT_ULTRA_VIOLENT_LUMP_NAME).unwrap(),
                |game| {
                    game.set_skill(Skill::UltraViolence);
                    game.set_state(GameState::Playing);
                },
            ),
            MenuItem::new(
                game.wad.get_image(SELECT_NIGHTMARE_LUMP_NAME).unwrap(),
                |game| {
                    game.set_skill(Skill::Nightmare);
                    game.set_state(GameState::Playing);
                },
            ),
        ])
        .set_selected(0)
        .set_menu_type(MenuType::Skill)
        .set_title(game.wad.get_image(SELECT_SKILL_LUMP_NAME).unwrap());
}
