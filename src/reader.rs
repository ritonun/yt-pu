use crate::models::{Config, Video};
use crate::utils;

pub fn get_local_video(config: &Config) -> Vec<Video> {
    let mut videos: Vec<Video> = Vec::new();

    // read through all files within the directory
    match std::fs::read_dir(&config.output_path) {
        Ok(entries) => {
            for entry in entries {
                let entry = match entry {
                    Ok(entry) => entry,
                    Err(e) => {
                        panic!(
                            "Failed to read file in {:?} due to error {}",
                            &config.output_path, e
                        );
                    }
                };

                videos.push(Video::new(
                    entry.path(),
                    utils::get_title_from_filename(entry.file_name().into_string().unwrap()),
                    String::new(),
                ));
            }
        }
        Err(e) => panic!(
            "Failed to read local directory {:?} due to error {}",
            &config.output_path, e
        ),
    }

    videos
}
