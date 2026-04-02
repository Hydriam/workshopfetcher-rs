use std::{
    path::Path,
    io
};
use std::process::Command;
use crate::args::{DownloadMod, ModsSubcommand};
pub fn handle_download(cmd: DownloadMod) -> io::Result<()> {
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

    let status = command.status()?;
    return Ok(())
}
pub fn run_mods(subcommand: ModsSubcommand) {
    match subcommand {
        ModsSubcommand::Download(cmd) => {
            match handle_download(cmd) {
                Ok(_) => println!("Steamcmd is succesfully reseted."),
                // TODO: Test if the error handling works good
                Err(error) => println!("Error: {:?}", error)
            }
        }
    }
}