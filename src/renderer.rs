use crate::anki::Image;
use regex::Regex;
use std::path::Path;

/// Converts a string to the anki text format
/// It converts all the image links to html images and converts the latex $ and $$ to correct KaTeX
/// math equations.
pub fn render(text: &str, wiki_absolute: &str) -> String {
    let mut text: String = text.to_owned();

    // Handle images
    text = convert_image_links(&text);

    // Handle links
    text = convert_links(&text, wiki_absolute);

    // Math handle
    text = convert_math(&text);

    text
}

/// Computes all the images stored in a certain text passage and returns all the `Image` Objects
pub fn extract_images(text: &str, wiki_absolute: &str) -> Vec<Image> {
    let mut images: Vec<Image> = Vec::with_capacity(2);

    let re = get_image_link_regex();

    for i in re.captures_iter(text) {
        // Get the path and the filename from the image link
        let (_, [path, filename]) = i.extract();

        // Check if path exists
        if !Path::new(path).exists() {
            log::warn!(
                "Image reference found but couldn't find image file. Image: {}",
                path
            );
            continue;
        }

        // Create the image
        images.push(Image::new(filename, path));
    }

    images
}

/// Converts all the image links to html images in a certain text
fn convert_image_links(text: &str) -> String {
    let re = get_image_link_regex();
    re.replace_all(text, "<img src='$2'>").to_string()
}

/// Converts all the $ and $$ to correct KaTeX math environments in a certain text
fn convert_math(text: &str) -> String {
    use regex::Regex;

    // Double quotes have to be handled directly since the single quotes would interfere
    let re_double = Regex::new(r"\$\$(.*)\$\$").unwrap();
    let new_text: String = re_double.replace(text, "\\[$1\\]").to_string();

    let re_single = Regex::new(r"\$(.*)\$").unwrap();
    re_single.replace_all(&new_text, "\\($1\\)").to_string()
}

/// Returns the regex to capture a image link in a markdown text
/// Checks for a markdown image reference. It has 2 capture groups. One for the image filename
/// and one for the image path
fn get_image_link_regex() -> Regex {
    Regex::new(r"\[.*\]\(((?:.*\/)*(.*(?:(?:\.png)|(?:\.jpeg))))\)").unwrap()
}

/// Converts all the links into absolute html links for anki to handle
fn convert_links(text: &str, absolute_path: &str) -> String {
    // This regex has 3 captureing groups one everything before the relative path, the relative
    // path itself and everything after
    let re: Regex = Regex::new(r"\[(.*)\]\((.*.md)\)").unwrap();
    re.replace_all(
        &text,
        format!("<a href=\"file:/{}/${{2}}\">${{1}}</a>", absolute_path),
    )
    .to_string()
}
