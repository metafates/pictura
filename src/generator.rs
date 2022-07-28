use std::collections::HashSet;
use std::fs;

use handlebars::Handlebars;
use pathdiff::diff_paths;
use serde_json::json;

use crate::common::get_web_dir;
use crate::gallery;

pub fn gen_html(name: &str, mappings: gallery::Mappings) -> String {
    let mut reg = Handlebars::new();
    let hbs = include_str!("gallery.hbs");
    reg.register_template_string("gallery", hbs).unwrap();

    let mut mappings = mappings.mappings.unwrap_or(Vec::new());
    let mut categories: HashSet<String> = HashSet::new();
    let page_dir = fs::canonicalize(get_web_dir()).unwrap();

    let re = regex::Regex::new(r"[-_ ]").unwrap();

    mappings
        .iter_mut()
        .for_each(|mapping| {
            if let Some(category) = mapping.category.clone() {
                categories.insert(category);
            }

            mapping.name = re
                .split(mapping.name.as_str())
                .map(|s| capitalize(s))
                .collect::<Vec<_>>()
                .join(" ");

            mapping.original = Some(diff_paths(
                fs::canonicalize(mapping.original.clone().unwrap()).unwrap(),
                page_dir.clone(),
            ).unwrap());

            mapping.medium = Some(diff_paths(
                fs::canonicalize(mapping.medium.clone().unwrap()).unwrap(),
                page_dir.clone(),
            ).unwrap());

            mapping.compressed = Some(diff_paths(
                fs::canonicalize(mapping.compressed.clone().unwrap()).unwrap(),
                page_dir.clone(),
            ).unwrap());
        });

    reg.render(
        "gallery",
        &json!({
            "title": name,
            "mappings": mappings,
            "categories": categories,
        }))
        .unwrap()
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}