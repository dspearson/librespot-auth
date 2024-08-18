# librespot-auth

A simple program for populating a `credentials.json` via Spotify's zeroconf authentication.

## Optional Arguments

- `--name`: Name of the virtual speaker (default: "Speaker").
- `--path`: Target path for credentials (default: `credentials.json` relative to execution).

## How It Works

While running on the same network as a Spotify client, it should appear as a Spotify Connect device with the given name. Select it once as an output device, and the Spotify client will transmit the required authentication blob. This blob is received by this application, written to the specified credentials path, and the application immediately exits.

This process is useful in cases where you run librespot on remote hosts on a different network (thus complicating zeroconf). As of version 3.203.235 of the Spotify eSDK, the `SpConnectionLoginPassword` API has been removed.

## How To Use It With SpotifyD

1. Clone the repo
2. Make sure to stop the spotifyd service `sudo systemctl stop spotifyd.service`
3. Run `cargo build --release` (requires rust compiler/toolkit, see https://rustup.rs/)
4. Run `./target/release/librespot-auth --name "Example Speaker"`
5. In Spotify app select the `Example Speaker` as output device
6. Copy the `credentials.json` to yours spotifyd `cache_directory`, eg. `~/spotifyd/target/release/cache_directory/`
7. Restart spotifyd service `sudo systemctl restart spotifyd.service`

```bash
# Example
./target/release/librespot-auth --name "Example Speaker"
Open Spotify and select output device: Example Speaker
Credentials saved: credentials.json
```
