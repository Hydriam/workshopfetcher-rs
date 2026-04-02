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
    Mods(ModsCommand),
    Reset,
    // Download collections
    /*
    TODO
    Collection(CollectionCommand)
    */
}
#[derive(Debug, Args)]
pub struct ModsCommand {
    #[clap(subcommand)]
    pub command: ModsSubcommand
}
#[derive(Debug, Subcommand)]
pub enum ModsSubcommand {
    // download the mod using steamcmd
    Download(DownloadMod)
}
#[derive(Debug, Args)]
pub struct DownloadMod {
    #[arg(short,long, required = true)]
    pub game_id: String,
    #[arg(short,long, required = true, num_args = 1..)]
    pub mod_ids: Vec<String>
}