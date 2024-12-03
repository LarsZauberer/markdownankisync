use clap::Parser;
use markdownankisync::file_manager::{get_md_files_in_directory, File};
use markdownankisync::tui::CLI;

fn main() {
    env_logger::init();
    let cli: CLI = CLI::parse();

    let files = get_md_files_in_directory(&cli.wiki_absolute);

    println!("Tags allowed: {:?}", cli.get_tags());

    for mut file in files {
        log::info!("Loading file: {}", file.get_file_path());
        file.read();
        file.load_tags();

        let mut found_tag: Option<String> = None;
        for tag in cli.get_tags() {
            if file.contains_tag(tag) {
                found_tag = Some(tag.to_owned());
                break;
            }
        }

        if found_tag.is_none() {
            log::info!("Skipping file {}", file.get_file_path());
            continue;
        }

        let tag_found = found_tag.unwrap();

        log::info!("File '{}' has tag {}", file.get_file_path(), tag_found);

        markdownankisync::parser::create_inline_reverse_from_content(
            &mut file,
            &tag_found,
            &cli.wiki_absolute,
        );
    }
}
