use crate::renderer::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Card {
    pub id: usize,
    pub front: String,
    pub back: String,
    pub media: Vec<Image>,
}

impl Card {
    pub fn new(front: &str, back: &str) -> Card {
        let mut front_media: Vec<Image> = extract_images(front);
        let mut back_media: Vec<Image> = extract_images(back);

        front_media.append(&mut back_media);

        let new_card: Card = Card {
            id: 0,
            front: render(front),
            back: render(back),
            media: front_media,
        };

        // TODO: Upload new_card

        new_card
    }
    pub fn update_card(&self, front: &str, back: &str) -> bool {
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
