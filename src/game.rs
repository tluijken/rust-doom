use crate::image_tools::scale_image;
use crate::menu::Menu;
use crate::wad::WadFile;
use image::DynamicImage;

use crate::HEIGHT;
use crate::WIDTH;
const SKULL_LUMP_NAME: &str = "M_SKULL1";
const BACKGROUND_LUMP_NAME: &str = "TITLEPIC";
#[derive(Debug, PartialEq, Copy, Clone)]
#[allow(dead_code)]
pub enum GameState {
    Menu,
    Playing,
    GameOver,
    Quit,
}

pub struct Game {
    pub state: GameState,
    pub menu: Menu,
    pub wad: WadFile,
    pub episode: Episode,
    pub skill: Skill,
    pub skull: DynamicImage,
    pub background: DynamicImage,
}

pub enum Episode {
    KneeDeep,
    Shores,
    Inferno,
}

pub enum Skill {
    TooYoungToDie,
    HeyNotTooRough,
    HurtMePlenty,
    UltraViolence,
    Nightmare,
}
impl Game {
    pub fn new(wad: WadFile) -> Self {
        let background = wad
            .get_image(BACKGROUND_LUMP_NAME)
            .expect("Background image not found")
            .resize_exact(
                WIDTH as u32,
                HEIGHT as u32,
                image::imageops::FilterType::Nearest,
            );

        let skull = wad.get_image(SKULL_LUMP_NAME).map(scale_image).unwrap();

        Self {
            state: GameState::Menu,
            menu: Menu::root(&wad),
            wad,
            episode: Episode::KneeDeep,
            background,
            skull,
            skill: Skill::TooYoungToDie,
        }
    }

    pub fn set_state(&mut self, game_state: GameState) {
        self.state = game_state;
    }
}
