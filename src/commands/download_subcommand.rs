use std::{
    path::Path,
    io
};
use std::process::Command;
use crate::args::{DownloadMods, DownloadSubcommand};
pub fn handle_download_mods(cmd: DownloadMods) -> io::Result<()> {
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

    for mod_id in cmd.mod_ids {
        command.arg("+workshop_download_item");
        command.arg(&cmd.game_id);
        command.arg(mod_id);
    }
    command.arg("+quit");
    command.stdout(std::process::Stdio::inherit());
    command.stderr(std::process::Stdio::inherit());

    command.status()?;
    println!("If steamcmd reported success the mods should be under ~/.steam/steam/steamapps/workshop/content/{}", cmd.game_id);
    return Ok(())
}
pub fn run_download(subcommand: DownloadSubcommand) {
    match subcommand {
        DownloadSubcommand::Mods(cmd) => {
            if let Err(error) = handle_download_mods(cmd) {
                println!("Error: {:?}", error)
            }
        }
    }
}