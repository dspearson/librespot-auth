# librespot-auth

A simple program for populating a `credentials.json` via Spotify's zeroconf authentication.

## Optional Arguments

- `--name`: Name of the virtual speaker (default: "Speaker").
- `--path`: Target path for credentials (default: `credentials.json` relative to execution).

## How It Works

While running on the same network as a Spotify client, it should appear as a Spotify Connect device with the given name. Select it once as an output device, and the Spotify client will transmit the required authentication blob. This blob is received by this application, written to the specified credentials path, and the application immediately exits.

This process is useful in cases where you run librespot on remote hosts on a different network (thus complicating zeroconf). As of version 3.203.235 of the Spotify eSDK, the `SpConnectionLoginPassword` API has been removed.
