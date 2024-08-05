use clap::Parser;
use futures::StreamExt;
use librespot_core::authentication::Credentials;
use librespot_core::SessionConfig;
use librespot_discovery::{DeviceType, Discovery};
use sha1::{Digest, Sha1};
use serde_json;
use std::fs::File;
use std::io::Write;
use std::process::exit;
use log::warn;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "Speaker")]
    name: String,
    #[arg(short, long, default_value = "credentials.json")]
    path: String,
}

pub fn save_credentials_and_exit(location: &str, cred: &Credentials) {
    let result = File::create(location).and_then(|mut file| {
        let data = serde_json::to_string(cred)?;
        write!(file, "{data}")
    });

    if let Err(e) = result {
        warn!("Cannot save credentials to cache: {}", e);
        exit(1);
    } else {
        println!("Credentials saved: {}", location);
        exit(0);
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let args = Args::parse();
    let name = args.name;
    let credentials_location = args.path;
    let device_id = hex::encode(Sha1::digest(name.clone().as_bytes()));

    let mut server = Discovery::builder(device_id.clone(), SessionConfig::default().client_id)
        .name(name.clone())
        .device_type(DeviceType::Speaker)
        .launch()
        .unwrap();

    println!("Open Spotify and select output device: {}", name);

    while let Some(credentials) = server.next().await {
        save_credentials_and_exit(&credentials_location, &credentials);
    }
}
