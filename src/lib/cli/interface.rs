use clap::{self, Parser};

pub fn input() -> (String, bool, Option<String>) {
    #[derive(Parser, Debug)]
    #[clap(author, version, about = "weh using feh, what a deal.")]
    struct Args {
        /// full path to the file. If -d or --download is passed in, path must be a valid URL to a image
        #[clap(short = 'p', long)]
        path: String,

        // Download from the web
        #[clap(short = 'd', long)]
        download: bool,

        // Name of the file to download.
        #[clap(short = 'n')]
        name: Option<String>,
    }

    let args = Args::parse();

    (args.path, args.download, args.name)
}
