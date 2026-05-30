use std::{
    env, io, path::Path, vec
};
use regex::Regex;
use std::process::Command;
use crate::{args::{DownloadCollection, DownloadMods, DownloadSubcommand}, commands::download_subcommand::DownloadError::NoSteamcmd};
enum DownloadError {
    NoSteamcmd,
}
fn download_mods(game_id: String, mod_ids: Vec<String>) -> Result<(), DownloadError> {
    if !Path::new("./steamcmd/steamcmd.sh").exists() {
        println!("Steamcmd not found, please run reset subcommand.");
        return Err(NoSteamcmd);
    }
    let mut command = Command::new("./steamcmd/steamcmd.sh");
    command.arg("+force_install_dir");
    command.arg("./workdir");
    command.arg("+login");
    command.arg("anonymous");

    for mod_id in mod_ids {
        command.arg("+workshop_download_item");
        command.arg(&game_id);
        command.arg(mod_id);
    }
    command.arg("+quit");
    command.stdout(std::process::Stdio::inherit());
    command.stderr(std::process::Stdio::inherit());

    command.status();
    println!("If steamcmd reported success the mods should be under {}/steacmd/workdir/steamapps/workshop/content/{}", env::current_dir().expect("Error getting current dir").display() ,&game_id);
    return Ok(())
}
pub fn handle_download_mods(game_id: String, mod_ids: Vec<String>) -> Result<(), ()> {
    // it might be better to print error mesages here instead of the function (?)
    if download_mods(game_id, mod_ids).is_ok() {
        return Ok(());
    } else {
        return Err(())
    }
}
pub fn handle_download_collection(cmd: DownloadCollection) -> Result<(), ()> {
    //println!("{}", cmd.collection_url);
    //note: https://users.rust-lang.org/t/how-to-download-files-from-the-internet/54878
    let resp = reqwest::blocking::get(cmd.collection_url).expect("request failed");
    let page_source = resp.text().expect("body invalid");
    //note: https://docs.rs/regex/latest/regex/
    let re = Regex::new(
        r#"<div class="workshopItem">[\s\S]*?<a href="[^"]*?id=(\d+)""#
    ).unwrap();
    let ids: Vec<String> = re
        .captures_iter(&page_source)
        .filter_map(|caps| caps.get(1)) 
        .map(|m| m.as_str().to_string())
        .collect();
    if download_mods(cmd.game_id, ids).is_ok() {
        return Ok(());
    } else {
        return Err(())
    }
}
pub fn run_download(subcommand: DownloadSubcommand) {
    match subcommand {
        DownloadSubcommand::Mods(cmd) => {
            if let Err(error) = handle_download_mods(cmd.game_id, cmd.mod_ids) {
                println!("Error: {:?}", error)
            }
        }
        DownloadSubcommand::Collection(cmd) => {
            if let Err(error) = handle_download_collection(cmd) {
                println!("Error: {:?}", error)
            }
        }
    }
}