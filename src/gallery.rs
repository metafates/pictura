use std::error::Error;
use std::io;
use std::fs;
use std::path::PathBuf;

use crate::common::{PICTURA, COMPRESSED_DIR, WALLPAPERS_DIR, MEDIUM_DIR, CONFIG_FILE, MAPPINGS_FILE};

/// Initialize a new gallery.
pub fn init(name: &str) -> io::Result<()> {
    let dot_dir = PathBuf::from(format!(".{}", PICTURA));

    // create all these directories if they don't exist
    vec![
        PathBuf::from(WALLPAPERS_DIR),
        dot_dir.clone(),
        dot_dir.clone().join(COMPRESSED_DIR),
        dot_dir.clone().join(MEDIUM_DIR),
    ]
        .into_iter()
        .try_for_each(|dir| -> io::Result<()> {
            if !dir.exists() {
                fs::create_dir(dir)?;
            }

            Ok(())
        })?;

    fs::write(
        dot_dir.join(CONFIG_FILE),
        format!("name = \"{}\"", name),
    )?;

    fs::write(
        dot_dir.join(MAPPINGS_FILE),
        "[[mappings]]",
    )?;

    Ok(())
}

/// Sync the gallery with the filesystem.
pub fn sync() -> Result<(), Box<dyn Error>> {
    Ok(())
}
