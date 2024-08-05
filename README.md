librespot-auth
==============

A simple program for populating a `credentials.json` via Spotify's zeroconf authentication.

Optional arguments:
  --name    Name of the virtual speaker, default: "Speaker"
  --path    Target path for credentials, default: "credentials.json" relative to execution

While running on the same network as a Spotify client, it should appear as a Spotify Connect device with the given name. Select it once as an output device, and the Spotify client will transmit the required authentication blob, which is received by this application, written to the credentials path, and immediately exits. This is useful in cases where you run librespot on remote hosts on a different network (thus complicating zeroconf), as in version 3.203.235 of the Spotify eSDK they removed the SpConnectionLoginPassword API.
