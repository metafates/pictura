use std::{fmt, fs};
use std::error::Error;
use std::io;
use std::path::{Path, PathBuf};

use image::GenericImageView;
use log::{info, warn};
use rand::Rng;
use serde::{Deserialize, Serialize};
use walkdir::{DirEntry, WalkDir};

use crate::common::{capitalize, get_compressed_dir, get_config_file, get_mappings_file, get_medium_dir, get_pictura_dir, get_wallpapers_dir, get_web_dir, IMAGE_EXTENSIONS};
use crate::generator;

/// Initialize a new gallery.
pub fn init(config: &Config) -> io::Result<()> {
    // create all these directories if they don't exist
    vec![
        get_wallpapers_dir(),
        get_pictura_dir(),
        get_compressed_dir(),
        get_medium_dir(),
        get_web_dir(),
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
        toml::to_string(&config).unwrap(),
    )?;

    fs::write(
        get_mappings_file(),
        "",
    )?;

    fs::write(
        get_web_dir().join("index.html"),
        generator::gen_html(&config, Mappings::default()),
    )?;

    Ok(())
}

/// Represents a single image mapping in the gallery.
/// This is used to map a compressed image with metadata in it's name to a gallery image.
#[derive(Serialize, Deserialize, Debug)]
pub struct Mapping {
    /// Image name (without extension)
    pub name: String,
    // Image extension
    pub extension: String,
    // Image category
    pub category: Option<String>,
    /// Image width in px
    pub width: u32,
    /// Image height in px
    pub height: u32,
    /// Dominant color of an image in HEX format
    pub color: String,
    /// Unique identifier of an image
    pub id: u32,
    /// Compressed path
    pub compressed: Option<PathBuf>,
    /// Medium path
    pub medium: Option<PathBuf>,
    /// Original path
    pub original: Option<PathBuf>,
}

impl Mapping {
    pub fn new(path: &Path, img: &image::DynamicImage) -> Result<Self, Box<dyn Error>> {
        // image name (without extension)
        let name = match path.file_stem() {
            Some(name) => name.to_str().unwrap().to_string(),
            None => return Err(io::Error::new(io::ErrorKind::Other, "Invalid path").into()),
        };

        // image extension
        let extension = match path.extension() {
            Some(extension) => {
                let extension = extension.to_str().unwrap().to_string();
                if !IMAGE_EXTENSIONS.contains(&extension.as_str()) {
                    return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid extension").into());
                }

                extension
            }
            None => return Err(io::Error::new(io::ErrorKind::Other, "Invalid path").into()),
        };

        let category = match path.parent() {
            Some(parent) => {
                let parent = parent.file_name().unwrap().to_str().unwrap().to_string();

                if parent == get_wallpapers_dir().to_str().unwrap() {
                    None
                } else {
                    Some(parent)
                }
            }
            None => None,
        };

        let (width, height) = img.dimensions();

        // dominant color of an image in HEX format
        let color_type = match img.color() {
            image::ColorType::Rgb8 | image::ColorType::Rgb16 => color_thief::ColorFormat::Rgb,
            image::ColorType::Rgba8 | image::ColorType::Rgba16 => color_thief::ColorFormat::Rgba,
            _ => color_thief::ColorFormat::Rgb,
        };

        let color = color_thief::get_palette(
            img.as_bytes(),
            color_type,
            10,
            5,
        )?[0];
        let color = rgb_to_hex(color.r, color.g, color.b);

        // image id
        let id = rand::thread_rng().gen_range(0..1000000);

        let mapping = Self {
            name,
            extension,
            category,
            width,
            height,
            color,
            id,
            compressed: None,
            medium: None,
            original: None,
        };

        Ok(mapping)
    }

    pub fn setup_paths(&mut self) -> io::Result<()> {
        let gallery_root = get_pictura_root_dir()?;

        if self.compressed.is_none() {
            self.compressed = Some(gallery_root.join(get_compressed_dir()).join(self.to_string()));
        }

        if self.medium.is_none() {
            self.medium = Some(gallery_root.join(get_medium_dir()).join(self.to_string()));
        }

        if self.original.is_none() {
            self.original = match &self.category {
                Some(category) => Some(gallery_root.join(get_wallpapers_dir()).join(category).join(format!("{}.{}", self.name, self.extension))),
                None => Some(gallery_root.join(get_wallpapers_dir()).join(format!("{}.{}", self.name, self.extension))),
            };
        }

        Ok(())
    }
}

impl fmt::Display for Mapping {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.category.is_none() {
            write!(
                f,
                "w-{}_h-{}_c-{}_i-{}.{}",
                self.width,
                self.height,
                self.color,
                self.id,
                self.extension
            )
        } else {
            write!(
                f,
                "w-{}_h-{}_c-{}_i-{}_k-{}.{}",
                self.width,
                self.height,
                self.color,
                self.id,
                self.category.clone().unwrap(),
                self.extension
            )
        }
    }
}

impl PartialEq for Mapping {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Mappings {
    pub(crate) mappings: Option<Vec<Mapping>>,
}

impl Default for Mappings {
    fn default() -> Self {
        Self { mappings: None }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub(crate) title: String,
    pub(crate) use_dark_theme: bool,
    pub(crate) animations: bool,
    pub(crate) github_raw_root_url: Option<String>
}

impl Default for Config {
    fn default() -> Self {
        let username = match std::env::var("USER") {
            Ok(username) => capitalize(username.as_str()),
            Err(_) => "Anon".to_string(),
        };

        Self {
            title: format!("{}'s Wallpapers", username),
            use_dark_theme: false,
            animations: false,
            github_raw_root_url: None
        }
    }
}

/// Sync the gallery with the filesystem.
pub fn sync() -> Result<(), Box<dyn Error>> {
    let gallery_root = get_pictura_root_dir()?;

    let mappings: Mappings = toml::from_str(
        fs::read_to_string(gallery_root.join(get_mappings_file()))?.as_str()
    )?;

    let config: Config = toml::from_str(
        fs::read_to_string(gallery_root.join(get_config_file()))?.as_str()
    )?;

    let mut mappings = mappings.mappings.unwrap_or(Vec::new());
    let mut to_add: Vec<PathBuf> = Vec::with_capacity(mappings.len());
    let mut added: usize = 0;
    let mut removed: usize = 0;

    mappings
        .iter_mut()
        .try_for_each(|mapping| -> io::Result<()> {
            mapping.setup_paths()?;

            Ok(())
        })?;


    let images: Vec<PathBuf> = WalkDir::new(gallery_root.join(get_wallpapers_dir()))
        .into_iter()
        .filter_entry(|e| !is_hidden(e))
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file() && is_image(e.path()))
        .map(|e| e.path().to_path_buf())
        .collect();

    images
        .iter()
        .for_each(|i| {
            if mappings.iter().find(|m| &gallery_root.join(m.original.clone().unwrap()) == i).is_none() {
                to_add.push(i.clone());
            }
        });

    for image_path in to_add.iter() {
        // TODO: print warning
        let img = image::open(image_path);
        if img.is_err() {
            warn!("Failed to open image: {}\n{}", image_path.display(), img.err().unwrap());
            continue;
        }
        let img = img.unwrap();

        let mapping = Mapping::new(image_path, &img);
        if mapping.is_err() {
            warn!("Failed to create mapping: {}\n{}", image_path.display(), mapping.err().unwrap());
            continue;
        }
        let mut mapping = mapping.unwrap();

        let (x, y) = (mapping.width, mapping.height);
        let metadata_name = mapping.to_string();

        // compressed
        img
            .thumbnail((x as f64 * 0.1).floor() as u32, (y as f64 * 0.1).floor() as u32)
            .save(gallery_root.join(get_compressed_dir()).join(&metadata_name))?;

        // medium
        img
            .thumbnail((x as f64 * 0.3).floor() as u32, (y as f64 * 0.3).floor() as u32)
            .save(gallery_root.join(get_medium_dir()).join(&metadata_name))?;

        mapping.setup_paths()?;
        mappings.push(mapping);

        added += 1;
    }

    let mappings = mappings
        .into_iter()
        .map(|m| {
            if images.iter().find(|p| p == &&gallery_root.join(m.original.clone().unwrap())).is_none() {
                let metadata_name = m.to_string();

                vec![
                    gallery_root.join(get_compressed_dir()).join(&metadata_name),
                    gallery_root.join(get_medium_dir()).join(&metadata_name),
                ]
                    .into_iter()
                    .for_each(|path| {
                        if let Err(e) = fs::remove_file(&path) {
                            warn!("Failed to remove file: {}\n{}", path.display(), e);
                        }
                    });

                removed += 1;
                None
            } else {
                Some(m)
            }
        })
        .filter_map(|m| m)
        .collect();

    let mappings = Mappings { mappings: Some(mappings) };

    fs::write(
        gallery_root.join(get_mappings_file()),
        toml::to_string(&mappings)?,
    )?;


    fs::write(
        gallery_root.join(get_web_dir()).join("index.html"),
        generator::gen_html(&config, mappings),
    )?;

    info!("{added} images added, {removed} images removed");

    Ok(())
}

/// Check if a file is hidden.
fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

pub fn get_pictura_root_dir() -> io::Result<PathBuf> {
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
    format!("{:02x}{:02x}{:02x}", r, g, b)
}

fn is_image(path: &Path) -> bool {
    let ext = path.extension().unwrap_or("".as_ref()).to_str().unwrap();
    IMAGE_EXTENSIONS.contains(&ext)
}