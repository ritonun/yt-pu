use serde_json::Value;
use std::path::PathBuf;
use std::process::Command as ProcessCommand;

struct Config {
    output_path: PathBuf,
    url: String,
    delete_local: bool,
}

fn get_playlist_videos(config: &Config) -> Result<Value, std::io::Error> {
    println!("Requesting playlist {}", config.url);
    let output = ProcessCommand::new("yt-dlp")
        .args(["--flat-playlist", "-J", &config.url]) // JSON output
        .output()?;

    let output: String = String::from_utf8_lossy(&output.stdout).to_string();
    let json_value: Value = serde_json::from_str(&output)?;

    Ok(json_value)
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
        output_path: matches.get_one::<PathBuf>("output_path").unwrap().clone(),
        delete_local: matches.get_flag("delete_local"),
    };

    // make the request and get a json result of all the videos in the playlist
    let json = match get_playlist_videos(&config) {
        Ok(json) => json,
        Err(e) => {
            eprintln!("Error fetching playlist: {}", e);
            return;
        }
    };
}
