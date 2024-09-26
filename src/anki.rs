use crate::api::*;
use crate::renderer::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Card {
    pub id: usize,
    pub front: String,
    pub back: String,
    pub media: Vec<Image>, // Media is only set for updates and never retrieved
}

impl Card {
    pub fn new(front: &str, back: &str, deck: &str) -> Option<Card> {
        // Extract all the media links
        let mut front_media: Vec<Image> = extract_images(front);
        let mut back_media: Vec<Image> = extract_images(back);
        front_media.append(&mut back_media);

        // Render the card
        let mut new_card: Card = Card {
            id: 0,
            front: render(front),
            back: render(back),
            media: front_media,
        };

        // Save all the media files
        for i in &new_card.media {
            let _ = store_media_file(i);
        }

        let card_result = add_note(&new_card, deck);
        if let Ok(id) = card_result {
            new_card.id = id;
            Some(new_card)
        } else {
            None
        }
    }

    /// Updates the card in anki with the new front and back.
    /// It returns false if there was an error and true if it was updated successfully
    pub fn update_card(&self) -> bool {
        let resp = update_note(&self);
        if resp.is_some() {
            false
        } else {
            true
        }
    }

    pub fn get_card(id: usize) -> Option<Card> {
        // Get the card information
        let information = get_notes_data(&[id; 0]);

        if let Ok(datas) = information {
            // Check if one card was found
            if datas.len() == 1 {
                // Create the card object
                let data: &responses::NoteData = &datas[0];
                Some(Card {
                    id: data.noteId,
                    front: data.fields.Front.value.clone(),
                    back: data.fields.Back.value.clone(),
                    media: Vec::new(),
                })
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn get_card_from_query(query: &str) -> Option<Card> {
        // Try to get the id
        let id_resp = get_note_id(query);

        if id_resp.is_err() {
            // Couldn't find the id
            return None;
        }

        // Get card with id
        Card::get_card(id_resp.unwrap())
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
