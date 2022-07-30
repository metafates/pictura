use std::collections::HashSet;
use std::fs;
use std::fs::read_to_string;
use std::path::PathBuf;

use handlebars::{Handlebars, handlebars_helper, RenderError};
use pathdiff::diff_paths;
use serde_json::{json, Value};

use crate::common::{capitalize, paths};
use crate::config::Config;
use crate::gallery;
use crate::gallery::get_pictura_root_dir;

handlebars_helper!(join_path: |path: Value, {with:str="."}| {
    let path = PathBuf::from(path.as_str().unwrap_or(""));
    let with_path = PathBuf::from(with);

    path.join(&with_path).to_str().unwrap().to_string()
});

handlebars_helper!(relative_path: |path: Value| {
    let path = PathBuf::from(path.as_str().unwrap());
    let page = fs::canonicalize(paths::html_file()).unwrap();

    diff_paths(
        fs::canonicalize(path).unwrap(),
        page.clone().parent().unwrap(),
    ).unwrap().to_str().unwrap().to_string()
});

handlebars_helper!(title_case: |string: Value| {
    let re = regex::Regex::new(r"[-_ ]").unwrap();

    re.split(string.as_str().unwrap())
        .map(capitalize)
        .collect::<Vec<String>>()
        .join(" ")
});

handlebars_helper!(length: |value: Value| {
    if value.is_array() {
        value.as_array().unwrap().len()
    } else if value.is_object() {
        value.as_object().unwrap().len()
    } else if value.is_string() {
        value.as_str().unwrap().len()
    } else {
        0
    }
});

handlebars_helper!(is_dark_helper: |hex: Value| {
    is_dark(hex.as_str().unwrap())
});

handlebars_helper!(contrast_color: |hex: Value| {
    if is_dark(hex.as_str().unwrap()) {
        "#ffffff".to_string()
    } else {
        "#000000".to_string()
    }
});

pub fn gen_html(config: &Config, pictures: gallery::Pictures) -> Result<String, RenderError> {
    let mut reg = Handlebars::new();
    let theme = {
        let default_theme = include_str!("gallery.hbs");
        if let Ok(dir) = get_pictura_root_dir() {
            let custom_theme = dir.join(paths::pictura()).join("index.hbs");
            if custom_theme.exists() {
                read_to_string(custom_theme).unwrap_or(default_theme.to_string())
            } else {
                default_theme.to_string()
            }
        } else {
            default_theme.to_string()
        }
    };

    reg.register_template_string("gallery", theme).unwrap();
    reg.register_helper("relative-path", Box::new(relative_path));
    reg.register_helper("title-case", Box::new(title_case));
    reg.register_helper("length", Box::new(length));
    reg.register_helper("contrast-color", Box::new(contrast_color));
    reg.register_helper("is-dark", Box::new(is_dark_helper));
    reg.register_helper("join-path", Box::new(join_path));

    let mut pictures = pictures.pictures.unwrap_or(Vec::new());
    let mut categories: HashSet<String> = HashSet::new();
    let mut extensions: HashSet<String> = HashSet::new();

    pictures
        .iter()
        .for_each(|mapping| {
            if let Some(category) = &mapping.category {
                categories.insert(category.clone());
            }
            extensions.insert(mapping.extension.clone());
        });

    pictures.sort_unstable_by_key(|mapping| mapping.name.clone());

    reg.render(
        "gallery",
        &json!({
            "config": config,
            "pictures": pictures,
            "categories": categories,
            "extensions": extensions,
        }))
}

fn is_dark(hex: &str) -> bool {
    let hex = {
        if hex.chars().next().unwrap() == '#' {
            hex.chars().skip(1).collect::<String>()
        } else {
            hex.to_string()
        }
    };

    if hex.len() != 6 {
        panic!("Hex color must be 6 characters long");
    }

    let (r, g, b) = (
        u8::from_str_radix(&hex[0..2], 16).unwrap(),
        u8::from_str_radix(&hex[2..4], 16).unwrap(),
        u8::from_str_radix(&hex[4..6], 16).unwrap(),
    );

    r as f32 * 0.299 + g as f32 * 0.587 + b as f32 * 0.114 <= 186 as f32
}