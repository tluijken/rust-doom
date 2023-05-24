use crate::menu::Menu;
use crate::wad::WadFile;
use image::DynamicImage;

use crate::HEIGHT;
use crate::WIDTH;
const SKULL_LUMP_NAME: &str = "M_SKULL1";
const BACKGROUND_LUMP_NAME: &str = "TITLEPIC";

/// The game state
/// # Remarks
/// This enum is used to determine what the game should be doing at any given time. For example, if
/// the game is in the `Playing` state, the game should be rendering the game world and handling
/// player input. If the game is in the `Menu` state, the game should be rendering the menu and
/// handling menu input.
#[derive(Debug, PartialEq, Copy, Clone)]
#[allow(dead_code)]
pub enum GameState {
    Menu,
    Playing,
    GameOver,
    Quit,
}

/// The game struct
/// # Remarks
/// This struct is used to store the game state, the menu, the WAD file, the episode, the skill
/// level, and the background and skull images.
/// This also contains the menu struct, which is used to store the current menu state.
pub struct Game {
    pub state: GameState,
    pub menu: Menu,
    pub wad: WadFile,
    pub episode: usize,
    pub skill: Skill,
    pub skull: DynamicImage,
    pub background: DynamicImage,
}

/// The possible skill levels
/// # Remarks
/// This enum is used to determine the skill level of the player. This is used to determine the
/// amount of damage the player takes, the amount of damage the player deals, and the amount of
/// ammo the player starts with.
pub enum Skill {
    TooYoungToDie,
    HeyNotTooRough,
    HurtMePlenty,
    UltraViolence,
    Nightmare,
}

/// The game implementation
impl Game {
    /// Creates a new game
    /// # Examples
    /// ```
    /// use game::Game;
    /// use wad::WadFile;
    /// let wad = WadFile::new("doom1.wad");
    /// let game = Game::new(wad);
    /// ```
    pub fn new(wad: WadFile) -> Self {
        let background = wad
            .get_image(BACKGROUND_LUMP_NAME)
            .expect("Background image not found")
            .resize_exact(
                WIDTH as u32,
                HEIGHT as u32,
                image::imageops::FilterType::Nearest,
            );

        let skull = wad.get_image(SKULL_LUMP_NAME).unwrap();

        Self {
            state: GameState::Menu,
            menu: Menu::root(&wad),
            wad,
            episode: 1,
            background,
            skull,
            skill: Skill::TooYoungToDie,
        }
    }

    /// Sets the game state
    /// # Arguments
    /// * `game_state` - The game state to set
    /// # Examples
    /// ```
    /// use game::Game;
    /// use game::GameState;
    /// use wad::WadFile;
    /// let wad = WadFile::new("doom1.wad");
    /// let mut game = Game::new(wad);
    /// game.set_state(GameState::Playing);
    /// ```
    pub fn set_state(&mut self, game_state: GameState) {
        self.state = game_state;
    }

    /// Sets the episode
    /// # Arguments
    /// * `episode` - The episode to set
    /// # Examples
    /// ```
    /// use game::Game;
    /// use game::Episode;
    /// use wad::WadFile;
    /// let wad = WadFile::new("doom1.wad");
    /// let mut game = Game::new(wad);
    /// game.set_episode(2);
    /// ```
    pub fn set_episode(&mut self, episode: usize) {
        self.episode = episode;
    }

    /// Sets the skill level
    /// # Arguments
    /// * `skill` - The skill level to set
    /// # Examples
    /// ```
    /// use game::Game;
    /// use game::Skill;
    /// use wad::WadFile;
    /// let wad = WadFile::new("doom1.wad");
    /// let mut game = Game::new(wad);
    /// game.set_skill(Skill::HurtMePlenty);
    /// ```
    pub fn set_skill(&mut self, skill: Skill) {
        self.skill = skill;
    }
}
