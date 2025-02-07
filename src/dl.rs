use crate::models::{Config, Video};
use crate::utils::progress;
use serde_json::Value;
use std::error::Error;
use std::path::PathBuf;

fn reqwest_playlist_info(config: &Config) -> Result<Value, Box<dyn Error>> {
    println!("[yt-pu] Reqwesting playlist {}", &config.url);

    // call yt-dlp to request playlist info
    let output = std::process::Command::new("yt-dlp")
        .args(["--flat-playlist", "-J", &config.url]) // JSON output
        .output()?;

    // convert response into serde_json::Value
    let output: String = String::from_utf8_lossy(&output.stdout).to_string();
    let json_value: Value = serde_json::from_str(&output)?;
    Ok(json_value)
}

pub fn get_online_videos(config: &Config) -> Vec<Video> {
    let reqwest_response = match reqwest_playlist_info(&config) {
        Ok(json) => json,
        Err(e) => panic!(
            "[yt-pu] Failed to reqwest playlist {} due to error {}",
            &config.url, e
        ),
    };

    let mut videos: Vec<Video> = Vec::new();

    if let Some(entries) = reqwest_response["entries"].as_array() {
        for video in entries {
            // check there is a title
            let title = if let Some(title) = video["title"].as_str() {
                title.to_string()
            } else {
                continue;
            };

            // check there is an url
            let url = if let Some(url) = video["url"].as_str() {
                url.to_string()
            } else {
                continue;
            };

            // check title and url are not empty
            if !url.is_empty() && !title.is_empty() {
                videos.push(Video::new(PathBuf::new(), title, url));
            }
        }
    }

    videos
}

fn dl_video(video: &Video, path: &String) -> Result<(), std::io::Error> {
    println!(
        "[yt-pu] yt-dlp --embed-thumbnail -f bestaudio[ext=m4a] --output {} {}",
        path, &video.url
    );

    let output = std::process::Command::new("yt-dlp")
        .args([
            "--embed-thumbnail",
            "-f",
            "bestaudio[ext=m4a]",
            "--output",
            path.as_str(),
            &video.url,
        ])
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .status()?;

    if output.success() {
        println!("[yt-pu] Video downloaded succesfully");
    } else {
        eprintln!(
            "[yt-pu] Failed to download video with exit status {}",
            output
        );
    }
    Ok(())
}

pub fn dl_playlist(videos: &Vec<Video>, config: &Config) -> Result<(), Box<dyn Error>> {
    // create output filepath
    let mut output_path: String = config.output_path.to_str().unwrap().to_string();
    output_path += "/%(title)s.%(ext)s";

    let mut progress_count: i32 = 1;
    let total_progress: i32 = videos.len() as i32;
    for video in videos {
        progress(&mut progress_count, &total_progress);

        match dl_video(&video, &output_path) {
            Ok(_) => {}
            Err(e) => eprintln!(
                "[yt-pu] Failed to download video {} due to error {}",
                &config.url, e
            ),
        }
    }
    Ok(())
}
