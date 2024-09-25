use crate::anki::Card;
use std::fs;

fn get_md_files_in_directory(directory: &str) -> Vec<String> {
    let read_dir_res = fs::read_dir(directory);
    if read_dir_res.is_err() {
        // Couldn't find any files in this directory
        println!("Error, couldn't read wiki path");
        return Vec::new();
    }

    let mut res: Vec<String> = Vec::new();
    for i in read_dir_res.unwrap() {
        let path = i.unwrap().path();
        // Check if mark down file
        if path.is_file() && path.extension().unwrap() == "md" {
            res.push(path.display().to_string());
        }
    }

    res
}

fn read_file(markdown_file: &str) -> String {}

fn get_cards_from_content(content: &str) -> Vec<Card> {}
