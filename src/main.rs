use anyhow::Result;
use reqwest::blocking as reqwest;
use std::{fs, path::PathBuf};

mod cli;
mod mount;
mod upload;

use cli::*;
use mount::*;

fn main() -> Result<()> {
    let args: Args = argh::from_env();

    match args.command {
        Command::UploadCommand(args) => match args.command {
            UploadCommand::Gallery(args) => {
                let fetched_file = reqwest::get(format!(
                    "https://raw.githubusercontent.com/hackclub/sprig/main/games/{}.js",
                    args.name
                ))?
                .text()?;
                upload::upload(&fetched_file);
            }
            UploadCommand::Local(args) => {
                let js = fs::read_to_string(args.file_name)?;
                upload::upload(&js);
            }
        },
        Command::FlashCommand(args) => {
            if args.latest {
                let mount_dir = MountDir::new(&args.dev)?;
                let fetched_file = reqwest::get(
                    "https://raw.githubusercontent.com/hackclub/sprig/main/pico-os.uf2",
                )?
                .bytes()?;

                let mut write_path = PathBuf::from(mount_dir.get_target_dir());
                write_path.push("pico-os.uf2");

                let mut temp_path = dirs::cache_dir().unwrap();
                temp_path.push("sprig-upload-os.uf2");

                fs::write(&temp_path, fetched_file)?;
                fs::copy(&temp_path, write_path)?;

                let _ = fs::remove_file(temp_path);
            } else if let Some(local) = args.local {
                let mount_dir = MountDir::new(&args.dev)?;
                let mut write_path = PathBuf::from(mount_dir.get_target_dir());
                write_path.push("pico-os.uf2");
                fs::copy(local, write_path)?;
            }
        }
    }

    Ok(())
}
