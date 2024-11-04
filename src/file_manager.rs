use crate::anki::Card;
use std::fs;

pub fn get_md_files_in_directory(directory: &str) -> Vec<String> {
    let read_dir_res = fs::read_dir(directory).expect("Couldn't read directory specified");

    let mut res: Vec<String> = Vec::new();
    for i in read_dir_res {
        let path = i.unwrap().path();
        // Check if mark down file
        if path.is_file() && path.extension().unwrap() == "md" {
            res.push(path.display().to_string());
        }
    }

    res
}

pub fn read_file(markdown_file: &str) -> String {
    fs::read_to_string(markdown_file).expect("Should have been able to read the file")
}

pub fn get_cards_from_content(content: &str, wiki_absolute: &str) -> Vec<Card> {
    let mut res: Vec<Vec<Card>> = Vec::with_capacity(4); // Update number if more types are
                                                         // implemented
                                                         // Get all the types

    // TODO: Pass down the CLI instead of wiki_abolute since we need the tags to make to decks
    let deck = get_deck_name(content);
    res.push(get_basics_from_content(content, &deck, wiki_absolute));

    res.concat()
}

fn get_basics_from_content(content: &str, deck: &str, wiki_absolute: &str) -> Vec<Card> {
    use regex::Regex;

    let mut text = content.to_owned();
    let mut res: Vec<Card> = Vec::new();

    // Creating regex
    // The regex captures all headings with a #card tag and goes to the next heading or to the end
    // of the file.
    // The id of the card is in an html comment tag -> <!--id:1234-->
    // There are 5 capturing groups.
    // 1: Everything before the card
    // 2: The Card Front
    // 3: The Card Back
    // 4: The id (Might be empty)
    // 5: The next heading (should stay)
    let re = Regex::new(r"((?:.*\n)*?)#+ (.+) #card\n\n((?:.*\n)*?)(?:<!--id:(\d+)-->\n\n)?(#+)")
        .unwrap();

    loop {
        let ma_option = re.captures(&text);

        if ma_option.is_none() {
            break;
        }

        let ma = ma_option.unwrap();

        let front = ma.get(2).map_or("", |m| m.as_str());
        let back = ma.get(3).map_or("", |m| m.as_str());
        let id = ma.get(4);

        // Cut out all the already solves cards
        let new_text = re.replace(&text, "$1$5").to_string();

        if id.is_none() {
            // Create new card
            // TODO: Make deck and absolute path variable
            let c = Card::new(front, back, deck, wiki_absolute);
            if c.is_none() {
                println!("Error: Couldn't create card");
            } else {
                res.push(c.unwrap());
            }
        } else {
            // TODO: Add update
        }

        text = new_text;
    }

    // This regex captures the last card at the end of the file which is not accepted by the other
    // regex.
    // It has 4 capturing groups
    // 1: Everything before the card
    // 2: The Card Front
    // 3: The Card Back
    // 4: The id (Might be empty)
    let re2 = Regex::new(r"((?:.*\n)*?)#+ (.+) #card\n\n((?:.*\n)*.*?)(?:(?:<!--id:(\d*)-->)|\z)")
        .unwrap();
    let ma = re2.captures(&text);
    if let Some(m) = ma {
        let front = m.get(2).map_or("", |m| m.as_str());
        let back = m.get(3).map_or("", |m| m.as_str());
        let id = m.get(4);

        if id.is_none() {
            // Create new card
            // TODO: Make deck and absolute path variable
            let c = Card::new(front, back, deck, wiki_absolute);
            if c.is_none() {
                println!("Error: Couldn't create card");
            } else {
                res.push(c.unwrap());
            }
        }
    }

    res
}

fn get_deck_name(content: &str) -> String {
    "test".to_string()
}
