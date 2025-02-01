use std::fs::File;
use std::io::prelude::*;
use std::io::Write;
use std::process::Command;

use config::playlist_url;

mod config;

fn get_playlist_videos(url: &str) -> Result<String, std::io::Error> {
    println!("yt-dlp --flat-playlist -J {}", url);

    let output = Command::new("yt-dlp")
        .args(["--flat-playlist", "-J", url]) // JSON output
        .output()?;

    let json_output = String::from_utf8_lossy(&output.stdout).to_string();
    Ok(json_output)
}

fn format_json(json_output: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Prettifying the result...");

    let json_value: serde_json::Value = serde_json::from_str(json_output)?;
    let pretty_json = serde_json::to_string_pretty(&json_value)?;

    let mut file = File::create("playlist.json")?;
    file.write(pretty_json.as_bytes())?;

    Ok(())
}

fn read_json() -> Result<serde_json::Value, std::io::Error> {
    let mut file = File::open("playlist.json")?;
    let v: serde_json::Value = serde_json::from_reader(file)?;
    Ok(v)
}

fn main() {
    let url = config::playlist_url;
    /*
    let json = match get_playlist_videos(url) {
        Ok(json) => json,
        Err(e) => {
            eprintln!("Error fetching playlist: {}", e);
            return;
        }
    };
    format_json(json.as_str());
    */

    let v = read_json().unwrap();
    println!("{}", v);
}
