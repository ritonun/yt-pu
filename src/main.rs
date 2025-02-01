use serde_json::Value;
use std::fs::File;
use std::io::prelude::*;
use std::io::Write;
use std::process::Command;
use std::process::Stdio;
use std::str::FromStr;

mod config;

fn get_playlist_videos(url: &str) -> Result<String, std::io::Error> {
    println!("yt-dlp --flat-playlist -J {}", url);

    let output = Command::new("yt-dlp")
        .args(["--flat-playlist", "-J", url]) // JSON output
        .output()?;

    let json_output = String::from_utf8_lossy(&output.stdout).to_string();
    Ok(json_output)
}

fn dl_playlist(urls: Vec<String>) -> Result<(), std::io::Error> {
    let mut output: String = String::from_str("").unwrap();
    output += config::output_path;
    output += "%(title)s.%(ext)s";

    for url in urls {
        println!("$ yt-dlp -x --audio-format mp3 --output {} {}", output, url);

        let mut output = Command::new("yt-dlp")
            .args([
                "-x",
                "--audio-format",
                "mp3",
                "--output",
                output.as_str(),
                &url,
            ])
            .stdout(Stdio::piped())
            .spawn()?;
        // Get the child process's stdout
        let stdout = output.stdout.take().expect("Failed to capture stdout");

        // Create a buffered reader for the output
        let reader = std::io::BufReader::new(stdout);

        // Iterate over the lines of stdout and print each line
        for line in reader.lines() {
            match line {
                Ok(line) => println!("{}", line),
                Err(e) => eprintln!("Error reading line: {}", e),
            }
        }

        // Wait for the command to finish
        let status = output.wait()?;
        println!("Command finished with status: {}", status);
    }
    Ok(())
}

fn format_json(json_output: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Prettifying the result...");

    let json_value: serde_json::Value = serde_json::from_str(json_output)?;
    let pretty_json = serde_json::to_string_pretty(&json_value)?;

    let mut file = File::create("playlist.json")?;
    file.write(pretty_json.as_bytes())?;

    Ok(())
}

fn extract_links(v: serde_json::Value) -> Vec<String> {
    /*
    entries []
    -> url
    */
    let mut urls: Vec<String> = Vec::new();

    if let Some(entries) = v["entries"].as_array() {
        for video in entries {
            if let Some(title) = video["title"].as_str() {
                println!("{}", title);
            }

            if let Some(url) = video["url"].as_str() {
                urls.push(url.to_string());
            }
        }
    } else {
        println!("No 'entries' filed find in the JSON!");
    }

    urls
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

    let v: serde_json::Value = read_json().unwrap();
    println!("{}", v);
    let videos_urls = extract_links(v);
    match dl_playlist(videos_urls) {
        Ok(_) => {}
        Err(e) => println!("Error: {}", e),
    }
}
