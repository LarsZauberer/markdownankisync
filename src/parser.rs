use crate::anki::Card;

pub fn get_tags(content: &str) -> Vec<String> {
    use regex::Regex;

    let re = Regex::new(r"(tags:\n)  - (\w+)\n").unwrap();
    let mut res: Vec<String> = Vec::with_capacity(2);
    let mut text: String = content.to_string();

    loop {
        let ma = re.captures(&text);
        if ma.is_none() {
            break;
        }

        let m = ma.unwrap();

        res.push(m.get(2).map_or("", |m| m.as_str()).to_string());

        // Clear the first tag entry to search for the next one
        text = re.replace(&text, "$1").to_string();
    }

    res
}

fn create_inline_reverse_from_content(content: &str, deck: &str, wiki_absolute: &str) -> String {
    // WARN: Currently deprecated and shouldn't be used
    use regex::Regex;

    let mut text = content.to_owned();

    // BUG: The regex probably doesn't recongnize the finale line
    let re = Regex::new(r"(.*):::(.*?)(?:\n| (?:<!--id1:(\d+)--> <!--id2:(\d+)-->\n))").unwrap();

    for ma in re.captures_iter(&content) {
        let front = ma.get(1).map_or("", |m| m.as_str()).to_string();
        let back = ma.get(2).map_or("", |m| m.as_str()).to_string();
        let id_str_1: Option<usize> = ma.get(3).map_or(None, |m| parse_id(m.as_str()));
        let id_str_2: Option<usize> = ma.get(4).map_or(None, |m| parse_id(m.as_str()));

        let mut left_card = Card::new(
            id_str_1,
            front.clone(),
            back.clone(),
            deck.to_string(),
            wiki_absolute,
            "Basic",
        );

        let mut right_card = Card::new(
            id_str_2,
            back.clone(),
            front.clone(),
            deck.to_string(),
            wiki_absolute,
            "Basic (type in the answer)",
        );

        // We upload the card here since it is necessary to get the id information for the card. So
        // we can add the id information to the content
        left_card.upload();
        right_card.upload();

        // Add id information
        let total_match = ma.get(0).expect("Invalid match");
    }

    text
}

fn parse_id(id: &str) -> Option<usize> {
    let id_usize = id.parse::<usize>();
    if id_usize.is_err() {
        println!("Warning: id is not a valid uzise. New card will be created");
        None
    } else {
        Some(id_usize.unwrap())
    }
}
