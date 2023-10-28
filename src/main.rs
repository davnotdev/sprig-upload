use anyhow::Result;
use reqwest::blocking as reqwest;
use std::fs;

mod cli;
mod upload;

use cli::*;

fn main() -> Result<()> {
    let args: Args = argh::from_env();

    match args.command {
        Command::UploadCommand(args) => match args.command {
            UploadCommand::Gallery(args) => {
                const GITHUB_API_URL: &str =
                    "https://raw.githubusercontent.com/hackclub/sprig/main/games/";
                let fetched_file =
                    reqwest::get(format!("{}/{}.js", GITHUB_API_URL, args.name))?.text()?;
                eprintln!("{}", fetched_file);
                upload::upload(&fetched_file);
            }
            UploadCommand::Local(args) => {
                let js = fs::read_to_string(args.file_name)?;
                upload::upload(&js);
            }
        },
    }

    Ok(())
}
