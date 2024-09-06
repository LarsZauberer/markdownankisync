use crate::anki::{Card, Image};
use serde_json::from_str;

pub fn add_note(card: Card, deck: &str) {}

pub fn get_decks() -> Result<Vec<String>, String> {
    let resp = requests::GetDecks::build().send();
    // TODO: Make the error handling smoother
    if let Ok(res) = resp {
        let text: &str = &res.text().unwrap();
        let get_decks_resp: responses::GetDecks =
            from_str(text).expect("Failed to read response json to GetDecks struct");
        Ok(get_decks_resp.result.unwrap())
    } else {
        let res = resp.unwrap_err();
        Err(res.to_string())
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
