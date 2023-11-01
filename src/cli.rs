use argh::FromArgs;

#[derive(FromArgs)]
///
pub struct Args {
    #[argh(subcommand)]
    pub command: Command,
}

#[derive(FromArgs)]
#[argh(subcommand)]
pub enum Command {
    FlashCommand(FlashCommandArgs),
    UploadCommand(UploadCommandArgs),
}

#[derive(FromArgs)]
#[argh(subcommand, name = "upload")]
///
pub struct UploadCommandArgs {
    #[argh(subcommand)]
    pub command: UploadCommand,
}

#[derive(FromArgs)]
#[argh(subcommand)]
pub enum UploadCommand {
    Gallery(UploadCommandGallery),
    Local(UploadCommandLocal),
}

#[derive(FromArgs)]
#[argh(subcommand, name = "gallery")]
///
pub struct UploadCommandGallery {
    #[argh(positional)]
    pub name: String,
}

#[derive(FromArgs)]
#[argh(subcommand, name = "local")]
///
pub struct UploadCommandLocal {
    #[argh(positional)]
    pub file_name: String,
}

#[derive(FromArgs)]
#[argh(subcommand, name = "flash")]
///
pub struct FlashCommandArgs {
    #[argh(switch)]
    /// a
    pub latest: bool,
    #[argh(positional)]
    /// a
    pub dev: String,
    #[argh(positional)]
    /// a
    pub local: Option<String>,
}
