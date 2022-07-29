use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

use handlebars::{Handlebars, handlebars_helper};
use pathdiff::diff_paths;
use serde_json::{json, Value};

use crate::common::{capitalize, get_web_dir};
use crate::gallery;
use crate::gallery::Config;

handlebars_helper!(relative_path: |path: Value| {
    let path = PathBuf::from(path.as_str().unwrap());
    let page_dir = fs::canonicalize(get_web_dir()).unwrap();

    diff_paths(
        fs::canonicalize(path).unwrap(),
        page_dir.clone(),
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

pub fn gen_html(config: &Config, mappings: gallery::Mappings) -> String {
    let mut reg = Handlebars::new();
    let hbs = include_str!("gallery.hbs");
    reg.register_template_string("gallery", hbs).unwrap();
    reg.register_helper("relative-path", Box::new(relative_path));
    reg.register_helper("title-case", Box::new(title_case));
    reg.register_helper("length", Box::new(length));
    reg.register_helper("contrast-color", Box::new(contrast_color));
    reg.register_helper("is-dark", Box::new(is_dark_helper));

    let mut mappings = mappings.mappings.unwrap_or(Vec::new());
    let mut categories: HashSet<String> = HashSet::new();
    let mut extensions: HashSet<String> = HashSet::new();

    mappings
        .iter()
        .for_each(|mapping| {
            if let Some(category) = &mapping.category {
                categories.insert(category.clone());
            }
            extensions.insert(mapping.extension.clone());
        });

    mappings.sort_unstable_by_key(|mapping| mapping.name.clone());

    reg.render(
        "gallery",
        &json!({
            "config": config,
            "mappings": mappings,
            "categories": categories,
            "extensions": extensions,
        }))
        .unwrap()
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