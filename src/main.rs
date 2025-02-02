use serde_json::Value;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

struct Config {
    output_path: PathBuf,
    url: String,
    delete_local: bool,
}

struct Video {
    path: PathBuf,
    name: String,
    url: String,
}

fn get_playlist_videos(config: &Config) -> Result<Value, std::io::Error> {
    println!("Requesting playlist {}", config.url);
    let output = Command::new("yt-dlp")
        .args(["--flat-playlist", "-J", &config.url]) // JSON output
        .output()?;

    let output: String = String::from_utf8_lossy(&output.stdout).to_string();
    let json_value: Value = serde_json::from_str(&output)?;

    Ok(json_value)
}

fn get_online_videos(config: &Config) -> Vec<Video> {
    // make the request and get a json result of all the videos in the playlist
    let json = match get_playlist_videos(&config) {
        Ok(json) => json,
        Err(e) => {
            panic!("Error fetching playlist: {}", e);
        }
    };

    let mut online_videos: Vec<Video> = Vec::new();

    if let Some(entries) = json["entries"].as_array() {
        for video in entries {
            let mut v = Video {
                url: String::new(),
                name: String::new(),
                path: PathBuf::new(),
            };
            if let Some(title) = video["title"].as_str() {
                v.name = title.to_string();
            }
            if let Some(url) = video["url"].as_str() {
                v.url = url.to_string();
            }

            if !v.url.is_empty() && !v.name.is_empty() {
                online_videos.push(v);
            }
        }
    }

    online_videos
}

fn get_local_videos(config: &Config) -> Vec<Video> {
    let mut videos: Vec<Video> = Vec::new();

    // get all files in the directory

    match fs::read_dir(&config.output_path) {
        Ok(entries) => {
            for entry in entries {
                let entry = entry.unwrap();
                if entry.path().is_file() {
                    // check it is a file
                    let video = Video {
                        path: entry.path(),
                        name: entry.file_name().into_string().unwrap(),
                        url: String::new(),
                    };
                    videos.push(video);
                }
            }
        }
        Err(e) => panic!("Error reading directory: {}", e),
    }

    videos
}

fn main() {
    let matches = clap::Command::new("yt-pu")
        .version("0.1.0")
        .about("Keep a local save of your yt playlist")
        .arg(
            clap::Arg::new("output_path")
                .short('o')
                .long("output_path")
                .required(true)
                .help("Specify the location of the local folder"),
        )
        .arg(
            clap::Arg::new("url")
                .required(true)
                .help("URL of the youtube playlist"),
        )
        .arg(
            clap::Arg::new("delete_local")
                .long("delete_local")
                .short('d')
                .help("Delete local file if not present in the online playlist")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    // match user flags with config param
    let config = Config {
        url: matches.get_one::<String>("url").unwrap().to_string(),
        output_path: PathBuf::from(matches.get_one::<String>("output_path").unwrap()),
        delete_local: matches.get_flag("delete_local"),
    };

    // create a vector containning all local video
    let local_videos = get_local_videos(&config);

    // create vector containning all online video
    let online_videos = get_online_videos(&config);

    println!("{} {}", local_videos.len(), online_videos.len());
}
