mod args;
mod commands;

use args::WorkshopFetcherArgs;
use clap::Parser;

use crate::{args::{EntityType}, commands::download_subcommand::run_download, commands::reset_subcommand::run_reset};

fn main() {
    let args = WorkshopFetcherArgs::parse();

    match args.entity_type {
        EntityType::Download(download) => {
            run_download(download.command);
        }
        EntityType::Reset => {
            match run_reset() {
                Ok(_) => println!("Steamcmd is succesfully reseted."),
                // TODO: Test if the error handling works good
                Err(error) => println!("Error: {}", error)
            };
        }
    }
}