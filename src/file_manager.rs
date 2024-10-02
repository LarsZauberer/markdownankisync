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

pub fn get_cards_from_content(content: &str) -> Vec<Card> {
    let mut res: Vec<Vec<Card>> = Vec::with_capacity(4); // Update number if more types are
                                                         // implemented
                                                         // Get all the types
    res.push(get_basics_from_content(content));

    res.concat()
}

fn get_basics_from_content(content: &str) -> Vec<Card> {
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
        let ma = re.captures(&text);

        if ma.is_none() {
            break;
        }

        let (_, [before, front, back, id, next]) = ma.unwrap().extract();

        text = re.replace(&text, "$1$2").to_string();
        // Needs some debugging probably
        println!("{}", text);
        println!("Id: {}", id);

        if id == "" {
            // Create new card
            // TODO: Make deck and absolute path variable
            let c = Card::new(front, back, "test", "./test_data");
            if c.is_none() {
                println!("Error: Couldn't create card");
            }
            res.push(c.unwrap());
        }
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
    if ma.is_some() {
        let (_, [before, front, back, id, next]) = ma.unwrap().extract();

        if id == "" {
            // Create new card
            // TODO: Make deck and absolute path variable
            let c = Card::new(front, back, "test", "./test_data");
            if c.is_none() {
                println!("Error: Couldn't create card");
            }
            res.push(c.unwrap());
        }
    }

    res
}
