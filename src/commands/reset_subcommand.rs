use std::{
    fs::{self, File},
    io::{self, Write},
    path::Path
};
use tar::Archive;
use flate2::read::GzDecoder;
pub fn run_reset() -> io::Result<()> {
    let steamcmd_path = Path::new("./steamcmd");
        if steamcmd_path.exists() {
            fs::remove_dir_all(steamcmd_path)?;
        }
        fs::create_dir(steamcmd_path)?;
        let tar_path = "steamcmd_linux.tar.gz";
        let tar_url = "https://steamcdn-a.akamaihd.net/client/installer/steamcmd_linux.tar.gz";

        //note: https://users.rust-lang.org/t/how-to-download-files-from-the-internet/54878
        if Path::new(tar_path).exists() {
            fs::remove_file(tar_path)?;

        }
        let resp = reqwest::blocking::get(tar_url).expect("request failed");
        let bytes = resp.bytes().expect("body invalid");
        let mut out = File::create(tar_path).expect("failed to create file");
        out.write_all(&bytes).expect("failed to write file");

        let tar_gz = File::open(tar_path)?;
        let tar = GzDecoder::new(tar_gz);
        let mut archive = Archive::new(tar);
        archive.unpack("./steamcmd")?;
        fs::remove_file(tar_path)?;
        Ok(())
}