use markdownankisync::file_manager;

fn main() {
    let files = file_manager::get_md_files_in_directory("./test_data");
    for i in files {
        let text = file_manager::read_file(&i);
        // file_manager::get_cards_from_content(&text);
        // let tags = file_manager::get_tags(&text);
        // println!("{:?}", tags);
    }
}
