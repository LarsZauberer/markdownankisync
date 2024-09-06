use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Card {
    pub id: usize,
    pub front: String,
    pub back: String,
    pub media: Vec<Image>,
}

impl Card {
    pub fn new(front: &str, back: &str, media: Vec<Image>) -> Card {
        // TODO: Implement
        Card {
            id: 0,
            front: convert_to_renderable(front),
            back: convert_to_renderable(back),
            media,
        }
    }
    pub fn update_card(&self) -> bool {
        // TODO: Implement
        false
    }
    pub fn get(id: usize) -> Option<Card> {
        // TODO: Implement
        None
    }
    pub fn exists(id: usize) -> bool {
        Self::get(id).is_some()
    }
}

#[derive(Serialize, Deserialize)]
pub struct Image {
    pub filename: String,
    pub data: String, // Has to be base64
}

impl Image {
    pub fn new(filename: &str, path_str: &str) -> Image {
        Image {
            filename: filename.to_owned(),
            data: image_base64::to_base64(path_str), // Read the image data and convert it to base64
        }
    }

    pub fn upload_image(&self) {
        // TODO: Upload image to anki
    }
}

// TODO: Rethink the module structure

pub fn convert_to_renderable(text: &str) -> String {
    let mut text: String = text.to_owned();

    // Handle images
    image_render::extract_images(&text);
    text = image_render::convert_image_links(&text);

    // Math handle
    text = convert_math(&text);

    text
}

mod image_render {
    use crate::anki::Image;
    use regex::Regex;
    use std::path::Path;

    fn get_image_link_regex() -> Regex {
        // Checks for a markdown image reference. It has 2 capture groups. One for the image filename
        // and one for the image path
        Regex::new(r"\[.*\]\(((?:.*\/)*(.*(?:(?:\.png)|(?:\.jpeg))))\)").unwrap()
    }

    pub fn extract_images(text: &str) -> Vec<Image> {
        let mut images: Vec<Image> = Vec::with_capacity(2);

        let re = get_image_link_regex();

        for i in re.captures_iter(text) {
            // Get the path and the filename from the image link
            let (_, [path, filename]) = i.extract();

            // Check if path exists
            if !Path::new(path).exists() {
                log::warn!(
                    "Image reference found but couldn't find image file. Image: {}",
                    path
                );
                continue;
            }

            // Create the image
            images.push(Image::new(filename, path));
        }

        images
    }

    pub fn convert_image_links(text: &str) -> String {
        let re = get_image_link_regex();
        re.replace_all(text, "<img src='$2'>").to_string()
    }
}

fn convert_math(text: &str) -> String {
    use regex::Regex;

    // Double quotes have to be handled directly since the single quotes would interfere
    let re_double = Regex::new(r"\$\$(.*)\$\$").unwrap();
    let new_text: String = re_double.replace(text, "\\[$1\\]").to_string();

    let re_single = Regex::new(r"\$(.*)\$").unwrap();
    re_single.replace_all(&new_text, "\\($1\\)").to_string()
}
