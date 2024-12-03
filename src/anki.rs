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
    pub model: String,
}

impl Card {
    pub fn new(
        id: Option<usize>,
        front: String,
        back: String,
        deck: String,
        wiki_abosulte: &str,
        model: &str,
    ) -> Card {
        let mut image_paths: Vec<Image> = extract_images(&front);
        image_paths.append(&mut extract_images(&back));

        // Renderout the front and the back
        let front_text = render(&front, wiki_abosulte);
        let back_text = render(&back, wiki_abosulte);

        Card {
            id: id.unwrap_or_else(|| 0),
            front: front_text,
            back: back_text,
            deck,
            media: image_paths,
            model: model.to_string(),
        }
    }

    pub fn upload(&mut self) -> bool {
        if self.id == 0 {
            let res_id = self.create_anki_card();

            if let Ok(id) = res_id {
                self.id = id;
                true // return success
            } else {
                log::error!(
                    "Error while uploading new card {:?}\nError: {}",
                    self,
                    res_id.unwrap_err()
                );
                false // return failure
            }
        } else {
            let err_box = self.update_anki_card();
            if err_box.is_some() {
                log::error!(
                    "Error while updating card {:?}\nError: {}",
                    self,
                    err_box.unwrap()
                );
                false
            } else {
                true
            }
        }
    }

    fn create_anki_card(&self) -> Result<usize, String> {
        // Preconditions
        assert!(
            self.front.len() != 0 && self.back.len() != 0,
            "Assertion: {}:::{} (Model: {}) <- is empty",
            self.front,
            self.back,
            self.model,
        );

        self.upload_media();
        add_note(self, &self.deck, &self.model)
    }

    fn update_anki_card(&self) -> Option<String> {
        // Preconditions
        assert!(self.id != 0);
        assert!(
            self.front.len() != 0 && self.back.len() != 0,
            "{}:::{} - Model: {}",
            self.front,
            self.back,
            self.model,
        );

        self.upload_media();

        update_note(self)
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
