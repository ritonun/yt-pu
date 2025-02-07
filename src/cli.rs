use std::path::PathBuf;

use crate::models::Config;

pub fn parse_args() -> Config {
    let matches = clap::Command::new("yt-pu")
        .version("1.3.0")
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

    // get playlist url
    let url = if let Some(url) = matches.get_one::<String>("url") {
        url.to_string()
    } else {
        panic!("Could not read playlist url.")
    };

    // get output_path folder
    let output_path = if let Some(output_path) = matches.get_one::<String>("output_path") {
        PathBuf::from(output_path)
    } else {
        panic!("Could not get output_path from user input.")
    };

    // get delete_local option
    let delete_local = matches.get_flag("delete_local");

    Config::new(output_path, url, delete_local)
}
