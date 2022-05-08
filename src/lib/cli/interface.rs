use clap::{self, Parser};
use super::super::helper::dirs;
use anyhow::Result;

pub fn input() -> Result<String> {
    #[derive(Parser, Debug)]
    #[clap(author, version, about = "weh using feh, what a deal.")]
    struct Args {
        /// name of the file if it's inside wallpapers custom folder, if -d is entered, enter the full path
        #[clap(short = 'n', long)]
        name: String,
        /// Is it in an different folder? Default: false
        #[clap(short = 'd')]
        wallpaper_dir: bool,
    }
    let dir = dirs::create_custom_folder(dirs::picture_folder()?)?;
    let args = Args::parse();

    if !args.wallpaper_dir {
      Ok(args.name)
    }  else{
      Ok(dir
        .join(args.name)
        .to_str()
        .expect("An internal error ocurred")
        .to_owned())
    }
    //(args.name, args.wallpaper_dir)
}
