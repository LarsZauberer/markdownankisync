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

pub fn write_file(markdown_file: &str, content: &str) -> bool {
    fs::write(markdown_file, content).is_ok()
}

pub fn create_cards_from_content(content: &str, cli: &crate::tui::CLI) -> String {
    let mut text: String = content.to_string();
    let tags = get_tags(content);
    println!("Tags found in file: {:?}", tags);
    // BUG: There is a conceptional bug that if multiple cards are in the tags are in the same
    // file, and multiple cards for the different decks should be created, then the problem is that
    // we only safe one id information about one card deck.
    for tag in cli.get_tags() {
        if !tags.contains(&tag.to_string()) {
            println!("Tag {} is not in file!", tag);
            continue;
        };
        println!("Generate cards");

        text = create_inline_reverse_from_content(&text, tag, &cli.wiki_absolute);
    }

    text
}

fn get_tags(content: &str) -> Vec<String> {
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

/// Extracts the inline reverse cards (declared with `:::`)
/// It automatically uploads the card so it gets the id information for the card and adds it to the
/// content.
fn create_inline_reverse_from_content(content: &str, deck: &str, wiki_absolute: &str) -> String {
    use regex::Regex;

    let mut text = content.to_owned();

    // BUG: The regex probably doesn't recongnize the finale line
    let re = Regex::new(r"(.*):::(.*?)(?:\n| (?:<!--id1:(\d+)--> <!--id2:(\d+)-->\n))").unwrap();

    for ma in re.captures_iter(&content) {
        let front = ma.get(1).map_or("", |m| m.as_str()).to_string();
        let back = ma.get(2).map_or("", |m| m.as_str()).to_string();
        let id_str_1: Option<usize> = ma.get(3).map_or(None, |m| utility::parse_id(m.as_str()));
        let id_str_2: Option<usize> = ma.get(4).map_or(None, |m| utility::parse_id(m.as_str()));

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

        // Assemble the new file information
        text = utility::double_inject_comment_attribute(
            text,
            &format!("{}:::{}", front, back),
            "id1",
            "id2",
            &format!("{}", left_card.id),
            &format!("{}", right_card.id),
            total_match.start(),
            total_match.end(),
        )
    }

    text
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

    let front = ma.get(2).map_or("", |m| m.as_str());
    let back = ma.get(3).map_or("", |m| m.as_str());
    let id = ma.get(4).map_or(None, |m| utility::parse_id(m.as_str()));

    // Cut out all the already solves cards
    let new_text = re.replace(&text, "$1$5").to_string();

    if id.is_none() {
        // Create new card
        res.push(Card::new(
            None,
            front.to_string(),
            back.to_string(),
            deck.to_string(),
            wiki_absolute,
        ));
    } else {
        res.push(Card::new(
            id,
            front.to_string(),
            back.to_string(),
            deck.to_string(),
            wiki_absolute,
        ))
    }

    text = new_text;
} */

// This regex captures the last card at the end of the file which is not accepted by the other
// regex.
// It has 4 capturing groups
// 1: Everything before the card
// 2: The Card Front
// 3: The Card Back
// 4: The id (Might be empty)
/* let re2 = Regex::new(r"((?:.*\n)*?)#+ (.+) #card\n\n((?:.*\n)*.*?)(?:(?:<!--id:(\d*)-->)|\z)")
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
} */
/*
    res
} */

mod utility {
    pub fn parse_id(id: &str) -> Option<usize> {
        let id_usize = id.parse::<usize>();
        if id_usize.is_err() {
            println!("Warning: id is not a valid uzise. New card will be created");
            None
        } else {
            Some(id_usize.unwrap())
        }
    }

    pub fn inject_comment_attribute(
        content: String,
        normal_content: &str,
        attribute_name: &str,
        value: &str,
        start: usize,
        end: usize,
    ) -> String {
        let mut suffix = "";
        // Check if last character is a \n Character. If yes we need to keep it
        if content.chars().nth(end - 1).unwrap() == '\n' {
            suffix = "\n";
        }
        let before = content[0..start].to_string();
        let after = content[end..].to_string();

        // Assemble the new string
        return before
            + &format!(
                "{} <!--{}:{}-->{}",
                normal_content, attribute_name, value, suffix
            )
            + &after;
    }

    pub fn double_inject_comment_attribute(
        content: String,
        normal_content: &str,
        attribute_name1: &str,
        attribute_name2: &str,
        value1: &str,
        value2: &str,
        start: usize,
        end: usize,
    ) -> String {
        let mut suffix = "";
        // Check if last character is a \n Character. If yes we need to keep it
        if content.chars().nth(end - 1).unwrap() == '\n' {
            suffix = "\n";
        }
        let before = content[0..start].to_string();
        let after = content[end..].to_string();

        // Assemble the new string
        return before
            + &format!(
                "{} <!--{}:{}--> <!--{}:{}-->{}",
                normal_content, attribute_name1, value1, attribute_name2, value2, suffix
            )
            + &after;
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn comment_injection_test_no_new_line() {
            let mut text = "asdf:::asdf".to_string();
            text = inject_comment_attribute(
                text,
                "asdf:::asdf",
                "id1",
                "1234567890101111314151617181920",
                0,
                11,
            );
            assert_eq!(
                text,
                "asdf:::asdf <!--id1:1234567890101111314151617181920-->",
            );
        }

        #[test]
        fn comment_injection_test_with_new_line() {
            let mut text = "asdf:::asdf\n".to_string();
            text = inject_comment_attribute(
                text,
                "asdf:::asdf",
                "id1",
                "1234567890101111314151617181920",
                0,
                12,
            );
            assert_eq!(
                text,
                "asdf:::asdf <!--id1:1234567890101111314151617181920-->\n",
            );
        }

        #[test]
        fn comment_double_injection_test_regex() {
            let text = "asdf:::asdf\n".to_string();
            let re =
                regex::Regex::new(r"(.*):::(.*?)(?:\n| (?:<!--id1:(\d+)--> <!--id2:(\d+)-->\n))")
                    .unwrap();
            let mut new_text = text.clone();
            for ma in re.captures_iter(&text) {
                let front = ma.get(1).map_or("", |m| m.as_str()).to_string();
                let back = ma.get(2).map_or("", |m| m.as_str()).to_string();

                let total_match = ma.get(0).expect("Invalid match");

                assert_eq!(total_match.start(), 0);
                assert_eq!(total_match.end(), text.len());

                new_text = double_inject_comment_attribute(
                    new_text,
                    &format!("{}:::{}", front, back),
                    "id1",
                    "id2",
                    "1234567890101111314151617181920",
                    "1234567890101111314151617181920",
                    total_match.start(),
                    total_match.end(),
                );
                assert_eq!(
                    new_text,
                    "asdf:::asdf <!--id1:1234567890101111314151617181920--> <!--id2:1234567890101111314151617181920-->\n",
                );
            }
        }

        #[test]
        fn comment_double_injection_test_regex_before_and_after() {
            let text = "before\nasdf:::asdf\nafter".to_string();
            let re =
                regex::Regex::new(r"(.*):::(.*?)(?:\n| (?:<!--id1:(\d+)--> <!--id2:(\d+)-->\n))")
                    .unwrap();
            let mut new_text = text.clone();
            for ma in re.captures_iter(&text) {
                let front = ma.get(1).map_or("", |m| m.as_str()).to_string();
                let back = ma.get(2).map_or("", |m| m.as_str()).to_string();

                let total_match = ma.get(0).expect("Invalid match");

                assert_eq!(total_match.start(), 7);
                assert_eq!(total_match.end(), 19);

                new_text = double_inject_comment_attribute(
                    new_text,
                    &format!("{}:::{}", front, back),
                    "id1",
                    "id2",
                    "1234567890101111314151617181920",
                    "1234567890101111314151617181920",
                    total_match.start(),
                    total_match.end(),
                );
                assert_eq!(
                    new_text,
                    "before\nasdf:::asdf <!--id1:1234567890101111314151617181920--> <!--id2:1234567890101111314151617181920-->\nafter",
                );
            }
        }
    }
}
