use crate::game_state::GameState;
use crate::wad::WadFile;
use crate::HEIGHT;
use crate::WIDTH;
use image::{DynamicImage, GenericImageView};
use minifb::{Key, Window};
const LINE_HEIGHT: usize = 20 * crate::SCALE as usize;
const MENU_X: usize = 105 * crate::SCALE as usize;
const PADDING: usize = 10;

const SKULL_LUMP_NAME: &str = "M_SKULL1";
const BACKGROUND_LUMP_NAME: &str = "TITLEPIC";
const TITLE_LUMP_NAME: &str = "M_DOOM";
const START_LUMP_NAME: &str = "M_NEWG";
const QUIT_LUMP_NAME: &str = "M_QUITG";
const LOAD_LUMP_NAME: &str = "M_LOADG";
const SAVE_LUMP_NAME: &str = "M_SAVEG";
const OPT_LUMP_NAME: &str = "M_OPTION";

struct MenuItem {
    image: DynamicImage,
    action: fn(&mut GameState),
}

pub struct Menu {
    options: Vec<MenuItem>,
    selected: usize,
    background: DynamicImage,
    logo: DynamicImage,
    skull: DynamicImage,
}

fn scale_image(img: DynamicImage) -> DynamicImage {
    img.resize(
        (img.width() as f32 * crate::SCALE) as u32,
        (img.height() as f32 * crate::SCALE) as u32,
        image::imageops::FilterType::Nearest,
    )
}

fn is_black(pixel: &image::Rgba<u8>) -> bool {
    pixel[0] == 0 && pixel[1] == 0 && pixel[2] == 0
}

fn render_image(img: &DynamicImage, x_pos: usize, y_pos: usize, buffer: &mut [u32]) {
    for (x, y, pixel) in img.pixels() {
        if is_black(&pixel) {
            continue;
        }
        let rgba = pixel.0;
        // Convert the pixel's color channels from u8 to u32, and arrange them into an ARGB format
        let color = ((rgba[3] as u32) << 24)
            | ((rgba[0] as u32) << 16)
            | ((rgba[1] as u32) << 8)
            | rgba[2] as u32;
        buffer[((y as usize + y_pos) * WIDTH) + x as usize + x_pos] = color;
    }
}

impl Menu {
    pub fn new() -> Self {
        let wad = WadFile::load(crate::WAD_FILE);

        let img = wad
            .get_image(BACKGROUND_LUMP_NAME)
            .expect("Background image not found")
            .resize_exact(
                WIDTH as u32,
                HEIGHT as u32,
                image::imageops::FilterType::Nearest,
            );

        Self {
            options: vec![
                MenuItem {
                    image: wad.get_image(START_LUMP_NAME).map(scale_image).unwrap(),
                    action: |game_state| *game_state = GameState::Playing,
                },
                MenuItem {
                    image: wad.get_image(OPT_LUMP_NAME).map(scale_image).unwrap(),
                    action: |game_state| *game_state = GameState::Playing,
                },
                MenuItem {
                    image: wad.get_image(LOAD_LUMP_NAME).map(scale_image).unwrap(),
                    action: |game_state| *game_state = GameState::Playing,
                },
                MenuItem {
                    image: wad.get_image(SAVE_LUMP_NAME).map(scale_image).unwrap(),
                    action: |game_state| *game_state = GameState::Playing,
                },
                MenuItem {
                    image: wad.get_image(QUIT_LUMP_NAME).map(scale_image).unwrap(),
                    action: |game_state| *game_state = GameState::Quit,
                },
            ],
            selected: 0,
            background: img,
            skull: wad.get_image(SKULL_LUMP_NAME).map(scale_image).unwrap(),
            logo: wad.get_image(TITLE_LUMP_NAME).map(scale_image).unwrap(),
        }
    }

    pub fn handle_input(&mut self, window: &mut Window, game_state: &mut GameState) {
        if window.is_key_down(Key::Down) {
            self.selected = (self.selected + 1) % self.options.len();
        }
        if window.is_key_down(Key::Up) {
            self.selected = match self.selected > 0 {
                true => self.selected - 1,
                false => self.options.len() - 1,
            };
        }
        if window.is_key_down(Key::Enter) {
            (self.options[self.selected].action)(game_state);
        }
    }

    pub fn render(&self, buffer: &mut [u32]) {
        // Copy the image's pixels into the buffer
        render_image(&self.background, 0, 0, buffer);

        // Render the logo
        let start_y = (HEIGHT - self.logo.height() as usize) / 4;
        render_image(&self.logo, MENU_X, start_y, buffer);

        let mut start_y = start_y + self.logo.height() as usize + PADDING;
        for (i, option) in self.options.iter().enumerate() {
            let y = start_y;
            let skull_x = MENU_X - self.skull.width() as usize - PADDING;
            // render the skull if this is the selected option
            if i == self.selected {
                render_image(&self.skull, skull_x, y, buffer);
            }
            render_image(&option.image, MENU_X, y, buffer);
            start_y += LINE_HEIGHT;
        }
    }
}
