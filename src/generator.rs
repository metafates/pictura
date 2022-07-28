use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

use handlebars::{Handlebars, handlebars_helper};
use pathdiff::diff_paths;
use serde_json::{json, Value};

use crate::common::get_web_dir;
use crate::gallery;

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
        .map(|word| {

            let mut c = word.chars();
            match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
            }
        })
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

pub fn gen_html(name: &str, mappings: gallery::Mappings) -> String {
    let mut reg = Handlebars::new();
    let hbs = include_str!("gallery.hbs");
    reg.register_template_string("gallery", hbs).unwrap();
    reg.register_helper("relative-path", Box::new(relative_path));
    reg.register_helper("title-case", Box::new(title_case));
    reg.register_helper("length", Box::new(length));

    let mappings = mappings.mappings.unwrap_or(Vec::new());
    let mut categories: HashSet<String> = HashSet::new();

    mappings
        .iter()
        .for_each(|mapping| {
            if let Some(category) = &mapping.category {
                categories.insert(category.clone());
            }
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
