use crate::anki::{Card, Image};
use serde_json::from_str;

pub fn add_note(card: Card, deck: &str) {}

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

pub mod responses {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct GetDecks {
        pub result: Option<Vec<String>>,
        pub error: Option<String>,
    }
}

pub mod requests {
    use reqwest::blocking::RequestBuilder;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct GetDecks {
        pub action: String,
        pub version: usize,
    }

    impl GetDecks {
        pub fn build() -> RequestBuilder {
            let get_decks: GetDecks = GetDecks {
                action: "deckNames".to_string(),
                version: 6,
            };
            let client: RequestBuilder = get_sender();
            client.body(serde_json::to_string(&get_decks).unwrap())
        }
    }

    fn get_sender() -> RequestBuilder {
        reqwest::blocking::Client::new().post("http://localhost:8765")
    }
}
