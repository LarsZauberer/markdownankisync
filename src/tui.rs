pub mod CLI {
    use clap::Parser;

    #[derive(Parser, Debug)]
    #[command(version)]
    pub struct CLI {
        /// Start the application in the simple quick editor mode
        #[arg(short = 'q', long = "quick", default_value_t = false)]
        pub quick: bool,

        /// The filter argument searches for a certain note. The filter has to be a valid Anki
        /// search string.
        /// The filter argument is only used if the `quick` mode is active. If the filter argument
        /// is empty the quick mode will automatically be set to bulk add
        #[arg(short='f', long="filter", default_value_t = String::from(""))]
        pub filter: String,
    }
}
