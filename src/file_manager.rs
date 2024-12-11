use crate::anki::Card;
use std::fs;

#[derive(Debug, Clone)]
pub struct File {
    file_path: String,
    content: Option<String>,
    tags: Option<Vec<String>>,
}

impl File {
    pub fn new(file_path: String) -> File {
        assert!(
            file_path.ends_with(".md"),
            "File passed to the file struct is not a .md file"
        );
        File {
            file_path,
            content: None,
            tags: None,
        }
    }

    pub fn read(&mut self) -> bool {
        let read_result = fs::read_to_string(&self.file_path);
        if let Ok(content) = read_result {
            self.content = Some(content);
            true
        } else {
            self.content = None;
            log::warn!("Error while reading the file {:?}", &self.file_path);
            false
        }
    }

    pub fn write(&self) -> bool {
        if let Some(content) = &self.content {
            fs::write(&self.file_path, content).is_ok()
        } else {
            false
        }
    }

    pub fn load_tags(&mut self) -> bool {
        if let Some(content) = &self.content {
            let tags: Vec<String> = crate::parser::get_tags(content);
            self.tags = Some(tags);
            true
        } else {
            log::warn!("Trying to load tags, but content is null");
            false
        }
    }

    pub fn replace(&mut self, replacement: &str, start: i32, end: i32) -> i32 {
        assert!(
            self.content.is_some(),
            "The content is null and therefore cannot be replaced"
        );

        let content = self.content.clone().unwrap();

        assert!(start >= 0 && start < content.len() as i32);
        assert!(end >= 0 && end <= content.len() as i32);

        let ustart: usize = start as usize;
        let uend: usize = end as usize;

        self.content = Some(content[0..ustart].to_owned() + replacement + &content[uend..]);
        let length_before: i32 = end - start;
        replacement.len() as i32 - length_before
    }

    pub fn contains_tag(&self, tag: &str) -> bool {
        assert!(self.is_tags_loaded(), "Tags are not loaded yet");

        let tags: &Vec<String> = self.get_tags().unwrap();

        tags.contains(&tag.to_owned())
    }

    // Getter
    pub fn get_file_path(&self) -> &str {
        &self.file_path
    }

    pub fn get_content(&self) -> Option<&str> {
        if let Some(content) = &self.content {
            Some(&content)
        } else {
            log::warn!("Trying to get content from file while content is null");
            None
        }
    }

    pub fn get_tags(&self) -> Option<&Vec<String>> {
        if let Some(tags) = &self.tags {
            Some(tags)
        } else {
            log::warn!("Trying to get tags from file while tags are null");
            None
        }
    }

    pub fn is_content_loaded(&self) -> bool {
        self.content.is_some()
    }

    pub fn is_tags_loaded(&self) -> bool {
        self.tags.is_some()
    }
}

pub fn get_md_files_in_directory(directory: &str) -> Vec<File> {
    let read_dir_res = fs::read_dir(directory).expect("Couldn't read directory specified");

    let mut res: Vec<File> = Vec::new();
    for i in read_dir_res {
        let path = i.unwrap().path();
        // Check if mark down file
        if path.is_file() && path.extension().is_some() && path.extension().unwrap() == "md" {
            res.push(File::new(path.display().to_string()));
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let file: File = File::new(String::from("data/some_file.md"));
        assert_eq!(file.get_file_path(), "data/some_file.md");
        assert_eq!(file.get_content(), None);
        assert_eq!(file.get_tags(), None);
    }

    #[test]
    fn test_read_sucess() {
        let mut file: File = File::new(String::from("test_data/line.md"));
        assert!(file.read(), "Successfully read file");
        assert!(file.get_content().is_some());
        assert_eq!(file.get_content().unwrap(), "asdf\n")
    }

    #[test]
    fn test_read_fail() {
        let mut file: File = File::new(String::from("data/failed.md"));
        assert!(!file.read(), "Successfully failed to read fail file");
        assert!(file.get_content().is_none());
    }

    #[test]
    fn test_tags_vocabulary() {
        let mut file: File = File::new(String::from("test_data/vocabulary.md"));
        assert!(file.read(), "Couldn't load test file");
        file.load_tags();
        assert_eq!(
            file.get_tags(),
            Some(&vec!["Japanese".to_owned(), "Test".to_owned()])
        );
    }

    #[test]
    fn test_replace() {
        let mut file: File = File::new(String::from("test_data/easy_vocab.md"));
        assert!(file.read(), "Couldn't load test file");
        let mut size_change = file.replace("Some:::Some3", 0, 12);
        assert_eq!(
            file.get_content().unwrap(),
            "Some:::Some3\nasdf:::asdf2\nfdsa:::fdsa2\n"
        );
        assert_eq!(size_change, 0);
        size_change = file.replace("Some:::Some32", 0, 12);
        assert_eq!(
            file.get_content().unwrap(),
            "Some:::Some32\nasdf:::asdf2\nfdsa:::fdsa2\n"
        );
        assert_eq!(size_change, 1);
        size_change = file.replace("Test:::Test", 14, 26);
        assert_eq!(
            file.get_content().unwrap(),
            "Some:::Some32\nTest:::Test\nfdsa:::fdsa2\n"
        );
        assert_eq!(size_change, -1);
    }

    #[test]
    fn test_contains_tag_success() {
        let mut file: File = File::new(String::from("test_data/vocabulary.md"));
        assert!(file.read(), "Couldn't load test file");
        assert!(file.load_tags(), "Couldn't load tags");

        let tags = file.get_tags();
        assert!(tags.is_some());
        assert_eq!(
            tags.unwrap(),
            &vec!["Japanese".to_owned(), "Test".to_owned()]
        );

        // Test contains
        assert!(file.contains_tag("Japanese"));
        assert!(file.contains_tag("Test"));
        assert!(!file.contains_tag("Test2"));
    }
}
