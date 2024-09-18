use crate::anki::{Card, Image};
use serde_json::from_str;

pub fn add_note(card: &Card, deck: &str) -> Result<Card, String> {
    let resp = requests::AddNote::build(deck, &card.front, &card.back).send();

    if let Ok(res) = resp {
        // Deserialize response
        let text: &str = &res.text().unwrap();
        let add_note_resp: responses::AddNote =
            from_str(text).expect("Failed to read response json to AddNote struct");

        // Check if there is an error in response
        if add_note_resp.error.is_some() {
            Err(add_note_resp.error.unwrap().to_string())
        } else {
            // Update card object
            let id: usize = add_note_resp.result.unwrap();
            let mut new_card: Card = card.clone();
            new_card.id = id;

            Ok(new_card)
        }
    } else {
        Err(resp.unwrap_err().to_string())
    }
}

pub fn get_decks() -> Result<Vec<String>, String> {
    let resp = requests::GetDecks::build().send();

    if let Ok(res) = resp {
        // Deserialize response
        let text: &str = &res.text().unwrap();
        let get_decks_resp: responses::GetDecks =
            from_str(text).expect("Failed to read response json to GetDecks struct");

        // Check if there is an error in response
        if get_decks_resp.error.is_some() {
            Err(get_decks_resp.error.unwrap().to_string())
        } else {
            Ok(get_decks_resp.result.unwrap())
        }
    } else {
        // Erorr while connecting to the server
        Err(resp.unwrap_err().to_string())
    }
}

pub fn get_note_ids(query: &str) -> Result<Vec<usize>, String> {
    let resp = requests::GetNoteIds::build(query.to_string()).send();

    if let Ok(res) = resp {
        // Deserialze response
        let text: &str = &res.text().unwrap();
        let get_notes_resp: responses::GetNoteIds =
            from_str(text).expect("Failed to read response json to GetNotes struct");

        // Check if there is an error in response
        if get_notes_resp.error.is_some() {
            Err(get_notes_resp.error.unwrap().to_string())
        } else {
            Ok(get_notes_resp.result.unwrap())
        }
    } else {
        // Error while connecting to the server
        Err(resp.unwrap_err().to_string())
    }
}

pub fn get_note_id(query: &str) -> Result<usize, String> {
    let res: Result<Vec<usize>, String> = get_note_ids(query);
    if res.is_ok() {
        let result: Vec<usize> = res.unwrap();
        if result.len() == 1 {
            Ok(result[0]) // Is it possible without the clone?
        } else if result.len() == 0 {
            Err(String::from("404"))
        } else {
            Err(String::from("Overflow"))
        }
    } else {
        Err(res.unwrap_err())
    }
}

pub fn get_notes_data(ids: &[usize]) -> Result<Vec<responses::NoteData>, String> {
    // TODO: Create struct `responses::NoteData`
    // TODO: Create struct `responses::GetNotesData`
    // TODO: Create struct `requests::GetNotesData`
    let query = requests::GetNotesData::build(ids).send();

    if let Ok(res) = query {
        // Deserialize
        let text: &str = &res.text().unwrap();
        let get_notes_resp: responses::GetNotesData =
            from_str(text).expect("Failed to read response json to GetNotesData struct");
        if get_notes_resp.error.is_some() {
            Err(get_notes_resp.error.unwrap().to_string())
        } else {
            Ok(get_notes_resp.result.unwrap())
        }
    } else {
        // Error while connecting to the server
        Err(query.unwrap_err().to_string())
    }
}

pub mod responses {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct GetDecks {
        pub result: Option<Vec<String>>,
        pub error: Option<String>,
    }

    #[derive(Serialize, Deserialize)]
    pub struct AddNote {
        pub result: Option<usize>,
        pub error: Option<String>,
    }

    #[derive(Serialize, Deserialize)]
    pub struct GetNoteIds {
        pub result: Option<Vec<usize>>,
        pub error: Option<String>,
    }
}

pub mod requests {
    use reqwest::blocking::RequestBuilder;
    use serde::{Deserialize, Serialize};
    use serde_json::to_string;

    #[derive(Serialize, Deserialize)]
    pub struct GetDecks {
        action: String,
        version: usize,
    }

    impl GetDecks {
        pub fn build() -> RequestBuilder {
            let get_decks: GetDecks = GetDecks {
                action: "deckNames".to_string(),
                version: 6,
            };
            let client: RequestBuilder = get_sender();
            client.body(to_string(&get_decks).unwrap())
        }
    }

    #[derive(Serialize, Deserialize)]
    pub struct AddNote {
        action: String,
        version: usize,
        params: AddNoteParams,
    }

    impl AddNote {
        pub fn build(deck: &str, front: &str, back: &str) -> RequestBuilder {
            let add_note: AddNote = AddNote {
                action: "addNote".to_string(),
                version: 6,
                params: AddNoteParams {
                    note: AddNoteParamsNote {
                        deckName: deck.to_string(),
                        modelName: "Basic".to_string(),
                        fields: Fields {
                            Front: front.to_string(),
                            Back: back.to_string(),
                        },
                    },
                },
            };

            get_sender().body(to_string(&add_note).unwrap())
        }
    }

    #[derive(Serialize, Deserialize)]
    struct AddNoteParams {
        note: AddNoteParamsNote,
    }

    #[allow(non_snake_case)]
    #[derive(Serialize, Deserialize)]
    struct AddNoteParamsNote {
        deckName: String,
        modelName: String,
        fields: Fields,
    }

    #[allow(non_snake_case)]
    #[derive(Serialize, Deserialize)]
    pub struct Fields {
        Front: String,
        Back: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct GetNoteIds {
        action: String,
        version: usize,
        params: GetNotesParams,
    }

    impl GetNoteIds {
        pub fn build(query: String) -> RequestBuilder {
            let get_notes: GetNoteIds = GetNoteIds {
                action: "findNotes".to_string(),
                version: 6,
                params: GetNotesParams { query },
            };
            get_sender().body(to_string(&get_notes).unwrap())
        }
    }

    #[derive(Serialize, Deserialize)]
    struct GetNotesParams {
        query: String,
    }

    fn get_sender() -> RequestBuilder {
        reqwest::blocking::Client::new().post("http://localhost:8765")
    }
}
