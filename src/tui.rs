use clap::Parser;

#[derive(Parser, Debug)]
#[command(version)]
pub struct CLI {
    /// The **absolute** path to the wiki directory
    #[arg(short='p', long="path", default_value_t = String::from(""))]
    pub wiki_absolute: String,

    #[arg(short='t', long="tags", default_value_t = String::from(""))]
    tags: String,

    #[arg(short = 'd', long = "dry-run", default_value_t = false)]
    dry_run: bool,
}

impl CLI {
    pub fn get_tags(&self) -> Vec<&str> {
        self.tags.split(',').collect()
    }
}
