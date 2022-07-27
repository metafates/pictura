use std::{fmt, fs};
use std::error::Error;
use std::io;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

use colored::Colorize;
use image::GenericImageView;
use rand::Rng;
use serde::{Deserialize, Serialize};
use walkdir::{DirEntry, WalkDir};

use crate::common::{get_compressed_dir, get_config_file, get_mappings_file, get_medium_dir, get_pictura_dir, get_wallpapers_dir, IMAGE_EXTENSIONS};

/// Initialize a new gallery.
pub fn init(name: &str) -> io::Result<()> {
    // create all these directories if they don't exist
    vec![
        get_wallpapers_dir(),
        get_pictura_dir(),
        get_compressed_dir(),
        get_medium_dir(),
    ]
        .into_iter()
        .try_for_each(|dir| -> io::Result<()> {
            if !dir.exists() {
                fs::create_dir_all(dir)?;
            }

            Ok(())
        })?;

    fs::write(
        get_config_file(),
        format!("name = \"{}\"", name),
    )?;

    fs::write(
        get_mappings_file(),
        "",
    )?;

    Ok(())
}

/// Represents a single image mapping in the gallery.
/// This is used to map a compressed image with metadata in it's name to a gallery image.
#[derive(Serialize, Deserialize, Debug)]
struct Mapping {
    /// Image name
    pub name: String,
    // Image extension
    pub extension: String,
    /// Image width in px
    pub width: u32,
    /// Image height in px
    pub height: u32,
    /// Dominant color of an image in HEX format
    pub color: String,
    /// Unique identifier of an image
    pub id: u32,
}

impl Mapping {
    pub fn new(path: &Path) -> Result<Self, Box<dyn Error>> {
        // image name
        let name = match path.file_stem() {
            Some(name) => name.to_str().unwrap().to_string(),
            None => return Err(io::Error::new(io::ErrorKind::Other, "Invalid path").into()),
        };

        let extension = match path.extension() {
            Some(extension) => {
                let extension = extension.to_str().unwrap().to_string();
                if !IMAGE_EXTENSIONS.contains(&extension.as_str()) {
                    return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid extension").into());
                }

                extension
            },
            None => return Err(io::Error::new(io::ErrorKind::Other, "Invalid path").into()),
        };

        let image = image::open(path)?;
        let (width, height) = image.dimensions();

        // dominant color of an image in HEX format
        let color = color_thief::get_palette(
            image.as_bytes(),
            color_thief::ColorFormat::Rgb,
            10,
            2,
        )?[0];
        let color = rgb_to_hex(color.r, color.g, color.b);

        // image id
        let id = rand::thread_rng().gen_range(0..1000000);

        let mapping = Self {
            name,
            extension,
            width,
            height,
            color,
            id,
        };

        Ok(mapping)
    }
}

impl fmt::Display for Mapping {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "w-{}_h-{}_c-{}_i-{}.{}",
            self.width,
            self.height,
            self.color,
            self.id,
            self.extension
        )
    }
}

/// Sync the gallery with the filesystem.
pub fn sync() -> Result<(), Box<dyn Error>> {
    // let _mappings: Vec<Mapping> = toml::from_str(
    //     fs::read_to_string(get_mappings_file())?.as_str()
    // )?;

    let gallery_root = get_pictura_root_dir()?;

    for entry in WalkDir::new(gallery_root.join(get_wallpapers_dir()))
        .into_iter()
        .filter_entry(|e| !is_hidden(e))
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file() && is_image(e.path()))
    {
        let mapping = Mapping::new(entry.path())?;
        println!("{}", mapping.to_string());
    }

    Ok(())
}

/// Check if a file is hidden.
fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

fn get_pictura_root_dir() -> io::Result<PathBuf> {
    let pwd = PathBuf::from(std::env::current_dir()?);

    for ancestor in pwd.ancestors() {
        match is_pictura_root(ancestor.to_path_buf()) {
            Ok(true) => return Ok(ancestor.to_path_buf()),
            Ok(false) => continue,
            Err(e) => {
                match e.kind() {
                    io::ErrorKind::PermissionDenied => return Err(e),
                    _ => continue,
                }
            }
        }
    }


    Err(io::Error::new(io::ErrorKind::NotFound, "Pictura root directory not found"))
}

/// Check if dir at path contains dir named .pictura
fn is_pictura_root(p: PathBuf) -> io::Result<bool> {
    Ok(
        p.is_dir() &&
            fs::read_dir(p)?
                .filter_map(|e| e.ok())
                .find(|e| e.file_name().to_str().unwrap_or("") == get_pictura_dir().to_str().unwrap())
                .is_some()
    )
}

fn rgb_to_hex(r: u8, g: u8, b: u8) -> String {
    format!("#{:02x}{:02x}{:02x}", r, g, b)
}

fn hex_to_rgb(hex: &str) -> (u8, u8, u8) {
    let hex = hex.trim_start_matches("#");
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap();
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap();
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap();
    (r, g, b)
}

fn is_image(path: &Path) -> bool {
    let ext = path.extension().unwrap_or("".as_ref()).to_str().unwrap();
    IMAGE_EXTENSIONS.contains(&ext)
}