use byteorder::{LittleEndian, ReadBytesExt};
use image::{DynamicImage, ImageBuffer, Rgb};
use std::collections::HashMap;
use std::env;
use std::io::{Cursor, Read};
use std::path::PathBuf;

const PALETTE_LUMP_NAME: &str = "PLAYPAL";

pub struct WadFile {
    // create a tuple array with an id, name and byte array
    pub lumps: HashMap<String, Vec<u8>>,
}

fn get_wad_dir() -> PathBuf {
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        // If the CARGO_MANIFEST_DIR environment variable is set, we're probably running with `cargo run`
        // CARGO_MANIFEST_DIR points to the directory where your Cargo.toml exists.
        PathBuf::from(manifest_dir).join("wad")
    } else {
        // Otherwise, we're probably running the program directly
        // env::current_exe() gives us the path of the current executable
        PathBuf::from(env::current_exe().unwrap().parent().unwrap()).join("wad")
    }
}

impl WadFile {
    pub fn load(path: &str) -> Self {
        let wad = wad::load_wad_file(get_wad_dir().join(path)).expect("Failed to load WAD file");
        let lumps = wad.entry_iter().fold(HashMap::new(), |mut result, item| {
            result.insert(item.display_name().to_string(), item.lump.to_vec());
            result
        });

        Self {
            lumps: lumps.clone(),
        }
    }

    pub fn get_lump(&self, name: &str) -> Option<&Vec<u8>> {
        self.lumps.get(name)
    }

    pub fn get_image(&self, name: &str) -> Result<DynamicImage, std::io::Error> {
        let lump = self
            .get_lump(name)
            .expect(format!("Image with name {} not found", name).as_str());
        let palette = self.get_lump(PALETTE_LUMP_NAME).expect("Palette not found");
        let palette = decode_palette(palette);
        decode_lump_image(lump, palette)
    }
}

#[allow(dead_code)]
struct PictureHeader {
    width: i16,
    height: i16,
    left_offset: i16,
    top_offset: i16,
}

struct Post {
    top_delta: u8,
    length: u8,
    data: Vec<u8>,
}

struct Column {
    posts: Vec<Post>,
}

fn decode_picture_header(cursor: &mut Cursor<Vec<u8>>) -> PictureHeader {
    let width = cursor.read_i16::<LittleEndian>().unwrap();
    let height = cursor.read_i16::<LittleEndian>().unwrap();
    let left_offset = cursor.read_i16::<LittleEndian>().unwrap();
    let top_offset = cursor.read_i16::<LittleEndian>().unwrap();

    PictureHeader {
        width,
        height,
        left_offset,
        top_offset,
    }
}

fn decode_post(cursor: &mut Cursor<Vec<u8>>) -> Result<Post, std::io::Error> {
    let top_delta = cursor.read_u8()?;

    // End of column's data
    if top_delta == 0xFF {
        return Err(std::io::Error::new(
            std::io::ErrorKind::UnexpectedEof,
            "End of column data",
        ));
    }

    let length = cursor.read_u8()?;
    cursor.read_u8()?; // Padding byte

    let mut data = vec![0; length as usize];
    cursor.read_exact(&mut data)?;

    cursor.read_u8()?; // Padding byte

    Ok(Post {
        top_delta,
        length,
        data,
    })
}

fn decode_column(cursor: &mut Cursor<Vec<u8>>) -> Result<Column, std::io::Error> {
    let mut posts = Vec::new();

    while let Ok(post) = decode_post(cursor) {
        posts.push(post);
    }

    Ok(Column { posts })
}

fn decode_lump_image(
    data: &Vec<u8>,
    palette: Vec<[u8; 3]>,
) -> Result<DynamicImage, std::io::Error> {
    let mut cursor = Cursor::new(data.clone());
    let picture_header = decode_picture_header(&mut cursor);

    let column_offsets: Vec<u32> = (0..picture_header.width)
        .map(|_| cursor.read_u32::<LittleEndian>().unwrap())
        .collect();

    let mut columns = Vec::new();
    for &offset in &column_offsets {
        cursor.set_position(offset as u64);
        columns.push(decode_column(&mut cursor)?);
    }

    // Create an image buffer with the same dimensions as the Doom image
    let mut img = ImageBuffer::new(picture_header.width as u32, picture_header.height as u32);

    // Iterate over the columns and the posts within each column
    for (x, column) in columns.iter().enumerate() {
        for post in &column.posts {
            for y in 0..post.length {
                // Get the color index from the post data
                let color_index = post.data[y as usize];

                // Look up the RGB values from the palette
                let rgb = palette[color_index as usize];

                // Set the pixel in the image buffer
                img.put_pixel(x as u32, (post.top_delta + y) as u32, Rgb(rgb));
            }
        }
    }

    // Convert the image buffer to a DynamicImage and return it
    Ok(DynamicImage::ImageRgb8(img))
}

fn decode_palette(data: &Vec<u8>) -> Vec<[u8; 3]> {
    let mut palette = Vec::new();
    for i in 0..256 {
        let r = data[i * 3];
        let g = data[i * 3 + 1];
        let b = data[i * 3 + 2];
        palette.push([r, g, b]);
    }
    palette
}
