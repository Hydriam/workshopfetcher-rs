use std::{
    io, path::Path, vec
};
use regex::Regex;
use std::process::Command;
use crate::args::{DownloadCollection, DownloadMods, DownloadSubcommand};
pub fn handle_download_mods(game_id: String, mod_ids: Vec<String>) -> io::Result<()> {
    /*println!("Game ID: {}", cmd.game_id);
    println!("Mod IDs: {:?}", cmd.mod_ids); */
    if !Path::new("./steamcmd/steamcmd.sh").exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Steamcmd not detected, please run reset subcommand."
        ));
    }
    let mut command = Command::new("./steamcmd/steamcmd.sh");
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

    command.status()?;
    println!("If steamcmd reported success the mods should be under ~/.steam/steam/steamapps/workshop/content/{}", &game_id);
    return Ok(())
}
pub fn handle_download_collection(cmd: DownloadCollection) -> io::Result<()> {
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
    handle_download_mods(cmd.game_id, ids);
    return Ok(())
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