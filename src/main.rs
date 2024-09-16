use clap::Parser;
use markdownankisync::anki::Card;
use markdownankisync::tui::CLI::CLI;
use markdownankisync::{api::get_decks, renderer::render};

fn main() {
    let cli: CLI = CLI::parse();

    // Since the nothing except the quick mode is implemented yet. It will force the quick mode to
    // be used
    if cli.filter.len() > 0 {
        search_edit(&cli.filter);
    } else {
        bulk_add();
    }
}

fn bulk_add() {
    println!("Not implemented yet");
}

fn search_edit(filter: &str) {}
