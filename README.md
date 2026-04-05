# Workshopfetcher-rs
A linux cli app for downloading mods from steam workshop

**The app is still work-in-progress**
## What this does?
This app can download mods from steam workshop using steamcmd without owning the game on steam, its uselful if you have a game on gog which modding community is on steam workshop like rimworld or project zomboid.

**You need 32 bit version of glibc installed beacuse steamcmd needs it**
## Usage
For now the app can do two things:
### Reset
It basically removes and then installs steamcmd, run it if you encounter any issues with steamcmd.
### Download Mods
This one downloads specifed mods, to find the mod and game appid refer to this image:
![alt text](workshopfetcher.png)
So for this mod the full command would be
```
cargo run download mods --game-id 108600 --mod-ids 3693258802
```
In --mod-ids you can put multiple mods
## Building
To run:
```
git clone https://github.com/Hydriam/workshopfetcher-rs
cd workshopfetcher-rs
cargo run
```