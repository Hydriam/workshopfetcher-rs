use clap:: {
    Args,
    Parser,
    Subcommand
};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct WorkshopFetcherArgs{
    #[clap(subcommand)]
    pub entity_type: EntityType,
}
#[derive(Debug, Subcommand)]
pub enum EntityType {
    // Download mods
    Download(DownloadCommand),
    Reset,
}
#[derive(Debug, Args)]
pub struct DownloadCommand {
    #[clap(subcommand)]
    pub command: DownloadSubcommand
}
#[derive(Debug, Subcommand)]
pub enum DownloadSubcommand {
    Mods(DownloadMods)
    // TODO: Collection downloading
}
#[derive(Debug, Args)]
pub struct DownloadMods {
    #[arg(short,long, required = true)]
    pub game_id: String,
    #[arg(short,long, required = true, num_args = 1..)]
    pub mod_ids: Vec<String>
}