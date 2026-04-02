mod args;
mod commands;

use args::WorkshopFetcherArgs;
use clap::Parser;

use crate::{args::{EntityType}, commands::mods_subcommand::run_mods, commands::reset_subcommand::run_reset};

fn main() {
    let args = WorkshopFetcherArgs::parse();

    match args.entity_type {
        EntityType::Mods(mods) => {
            run_mods(mods.command);
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