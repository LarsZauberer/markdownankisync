use clap::Parser;
use markdownankisync::file_manager;
use markdownankisync::tui::CLI;

fn main() {
    env_logger::init();
    let cli: CLI = CLI::parse();

    let file_paths = file_manager::get_md_files_in_directory(&cli.wiki_absolute);

    println!("Tags allowed: {:?}", cli.get_tags());

    /* for file in file_paths {
        println!("Loading file {}", file);
        let mut content: String = file_manager::read_file(&file);
        content = file_manager::create_cards_from_content(&content, &cli);
        // println!("New file content:\n{}", content);
        if !file_manager::write_file(&file, &content) {
            panic!("Error writing back the file");
        }
    } */
}
