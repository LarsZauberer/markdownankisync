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

pub fn get_cards_from_content(content: &str, cli: &crate::tui::CLI) -> Vec<Card> {
    let mut res: Vec<Vec<Card>> = Vec::with_capacity(4); // Update number if more types are
                                                         // implemented
                                                         // Get all the types
                                                         // It doesn't care if there are multiple decks in this note file

    let tags = get_tags(content);
    for tag in cli.get_tags() {
        if tags.contains(&tag.to_string()) {
            continue;
        };

        // res.push(get_basics_from_content(content, tag, &cli.wiki_absolute));
    }

    res.concat()
}

pub fn get_tags(content: &str) -> Vec<String> {
    use regex::Regex;

    let re = Regex::new(r"(tags:\n)  - (\w+)").unwrap();
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
        // FIX: there is a problem that the suffix \n of the extracted tag is not deleted. Example
        // "tags:\n  - Analysis\n..." becomes "tags:\n\n..."
        text = re.replace(&text, "$1").to_string();
    }

    res
}

/* fn get_basics_from_content(content: &str, deck: &str, wiki_absolute: &str) -> Vec<Card> {
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

        let front = ma.get(2).map_or("", |m| m.as_str()); // FIX: Better problem/error handling
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
} */
