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

pub fn create_inline_reverse_from_content(
    file: &mut crate::file_manager::File,
    deck: &str,
    wiki_absolute: &str,
) {
    use regex::Regex;

    // Assert that everything from the file is already loaded
    assert!(file.is_content_loaded());
    assert!(file.is_tags_loaded());

    // The change tracker keeps track how much the indices of the regex matches have shifted due to
    // the id adds/changes
    let mut change_tracker: i32 = 0;

    // BUG: The regex probably doesn't recongnize the finale line (Maybe it works, needs some
    // testing)
    let re = Regex::new(r"(.*):::(.*?)(?:\n| (?:<!--id1:(\d+)--> <!--id2:(\d+)-->\n))").unwrap();

    let content = file.get_content().unwrap().to_owned();

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
        let l_fail = left_card.upload();
        let r_fail = right_card.upload();

        if !l_fail || !r_fail {
            panic!("Error while uploading inline reverse cards to anki");
        }

        // Add id information
        let total_match = ma.get(0).expect("Invalid match");
        let new_string: String = format!(
            "{}:::{} <!--id1:{}--> <!--id2:{}-->\n",
            front, back, left_card.id, right_card.id
        );

        change_tracker = change_tracker
            + file.replace(
                &new_string,
                total_match.start() as i32 + change_tracker,
                total_match.end() as i32 + change_tracker,
            );

        file.write();
    }
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
