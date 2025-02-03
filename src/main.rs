use serde_json::Value;
use std::fs;
use std::io::prelude::*;
use std::path::PathBuf;
use std::process::Command;
use std::str::FromStr;

#[derive(Debug)]
struct Config {
    output_path: PathBuf,
    url: String,
    delete_local: bool,
}

#[derive(Debug)]
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

fn find_filename_in_vec(videos: &Vec<Video>, video: &Video) -> Option<Video> {
    // create a list of all filenames
    // sanitize filename and video name
    // find match (or not)

    let treshold = 10;
    let match_video = videos
        .iter()
        .min_by_key(|v| strsim::levenshtein(&v.name, &video.name))
        .and_then(|v| {
            if strsim::levenshtein(&v.name, &video.name) <= treshold {
                Some(v)
            } else {
                None
            }
        });

    match match_video {
        Some(v) => {
            let video = Video {
                url: (v.url).to_string(),
                name: (v.name).to_string(),
                path: v.path.clone(),
            };
            Some(video)
        }
        None => None,
    }
}

fn remove_local_dl_video(online_videos: &Vec<Video>, local_videos: &Vec<Video>) -> Vec<Video> {
    let mut video_to_dl: Vec<Video> = Vec::new();

    for online_video in online_videos {
        // if video not already in local folder, add to list to dl
        match find_filename_in_vec(&local_videos, &online_video) {
            Some(_) => {}
            None => {
                let v = Video {
                    url: (online_video.url).to_string(),
                    name: (online_video.name).to_string(),
                    path: online_video.path.clone(),
                };
                video_to_dl.push(v);
            }
        }
    }

    println!("Number of videos to download: {}", video_to_dl.len());
    video_to_dl
}

fn remove_local_not_in_playlist(local_videos: &Vec<Video>, online_videos: &Vec<Video>) {
    let mut video_to_remove: Vec<Video> = Vec::new();

    // find all local video not in online playlist anymore
    for local_video in local_videos {
        match find_filename_in_vec(&online_videos, &local_video) {
            Some(_) => {}
            None => video_to_remove.push(Video {
                url: (local_video.url).to_string(),
                name: (local_video.name).to_string(),
                path: local_video.path.clone(),
            }),
        }
    }

    // remove the videos not found online
    for video in video_to_remove {
        match trash::delete(&video.path) {
            Ok(_) => println!("Moved to trash: {}", video.path.to_string_lossy()),
            Err(e) => eprintln!(
                "Failed to move to trash {} due to error: {}",
                video.path.to_string_lossy(),
                e
            ),
        }
    }
}

fn dl_playlist(videos: &Vec<Video>, config: &Config) -> Result<(), std::io::Error> {
    // structure the output_path
    let mut output: String = String::from_str("").unwrap();
    output += config.output_path.to_str().unwrap();
    output += "%(title)s.%(ext)s";

    for video in videos {
        println!(
            "$ yt-dlp -x --audio-format mp3 --output {} {}",
            output, video.url
        );

        let mut output = Command::new("yt-dlp")
            .args([
                "--embed-thumbnail",
                "-x",
                "--audio-format",
                "mp3",
                "-f",
                " bestaudio",
                "--output",
                output.as_str(),
                &video.url,
            ])
            .stdout(std::process::Stdio::piped())
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

    // if option --delete-local, remove local files that are not in the playlist anymore
    if config.delete_local {
        remove_local_not_in_playlist(&local_videos, &online_videos);
    }

    // create a vector of the videos to download (not already dl localy)
    let videos_to_dl = remove_local_dl_video(&online_videos, &local_videos);

    // download the videos using yt-dlp
    match dl_playlist(&videos_to_dl, &config) {
        Ok(_) => println!("Sucessfully downloaded all videos"),
        Err(e) => eprintln!("Error while downloading videos: {}", e),
    }
}
