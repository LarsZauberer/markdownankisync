use crate::api::*;
use crate::renderer::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Card {
    pub id: usize, // By default if the id has no value, it gets the default value 0
    pub front: String,
    pub back: String,
    pub deck: String,
    media: Vec<Image>,
}

impl Card {
    pub fn new(
        id: Option<usize>,
        front: String,
        back: String,
        deck: String,
        wiki_abosulte: &str,
    ) -> Card {
        let mut image_paths: Vec<Image> = extract_images(&front, wiki_abosulte);
        image_paths.append(&mut extract_images(&back, wiki_abosulte));

        Card {
            id: id.unwrap_or_else(|| 0),
            front,
            back,
            deck,
            media: image_paths,
        }
    }

    pub fn upload(&mut self) -> bool {
        if self.id == 0 {
            let res_id = self.create_anki_card();

            if let Ok(id) = res_id {
                self.id = id;
                true // return success
            } else {
                false // return failure
            }
        } else {
            self.update_anki_card()
        }
    }

    fn create_anki_card(&self) -> Result<usize, String> {
        // Preconditions
        assert!(self.front.len() != 0 && self.back.len() != 0);

        self.upload_media();
        add_note(self, &self.deck)
    }

    fn update_anki_card(&self) -> bool {
        // Preconditions
        assert!(self.id != 0);
        assert!(self.front.len() != 0 && self.back.len() != 0);

        self.upload_media();
        let res = update_note(self);

        res.is_none()
    }

    fn upload_media(&self) -> bool {
        for i in &self.media {
            let res = store_media_file(i);
            if res.is_err() {
                return false;
            }
        }
        true // return success
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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
}
