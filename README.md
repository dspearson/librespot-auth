# librespot-auth

A simple program for populating a `credentials.json` via Spotify's zeroconf authentication.

## Optional Arguments

- `--name`: Name of the virtual speaker (default: "Speaker").
- `--path`: Target path for credentials (default: `credentials.json` relative to execution).

## How It Works

While running on the same network as a Spotify client, it should appear as a Spotify Connect device with the given name. Select it once as an output device, and the Spotify client will transmit the required authentication blob. This blob is received by this application, written to the specified credentials path, and the application immediately exits.

This process is useful in cases where you run librespot on remote hosts on a different network (thus complicating zeroconf). As of version 3.203.235 of the Spotify eSDK, the `SpConnectionLoginPassword` API has been removed.

## Pre-built binaries

Pre-built statically-linked binaries are available under the releases section for 64-bit Linux systems.

## Building

Install the rust toolchain (see https://rustup.rs/), and run `cargo build --release`.

## Example Usage (spotifyd)

```bash
$ ./target/release/librespot-auth --name "Example Speaker"
Open Spotify and select output device: Example Speaker
```

Open the Spotify client from a machine on the same network as you ran this, ensuring no proxy is in use that may interfere with zeroconf. Select the speaker you just defined, i.e. "Example Speaker", as an output device. The credentials are then saved to `credentials.json`. Ensure spotifyd is stopped, i.e. `sudo systemctl stop spotifyd`, copy this file to your spotifyd `cache_directory`, and then start spotifyd again (`sudo systemctl start spotifyd`).
