use std::path::PathBuf;

use crate::models::Video;

pub fn get_title_from_filename(filename: String) -> String {
    filename.replace(".m4a", "")
}

fn find_filename_in_vec(videos_to_search: &Vec<Video>, video: &Video) -> Option<Video> {
    // define levenshtein treshold
    let treshold = 10;

    // find closest match
    let closest_match = videos_to_search
        .iter()
        .min_by_key(|v| strsim::levenshtein(&v.name, &video.name));

    // check if closest match within treshold
    if let Some(v) = closest_match {
        if strsim::levenshtein(&v.name, &video.name) <= treshold {
            return Some(Video::new(
                v.path.clone(),
                (v.name).to_string(),
                (v.url).to_string(),
            ));
        }
    }
    None
}

pub fn remove_local_videos_from_videos_to_dl(
    local_videos: &Vec<Video>,
    online_videos: &mut Vec<Video>,
) {
    let mut videos_to_dl: Vec<Video> = Vec::new();

    for online_video in online_videos.iter() {
        match find_filename_in_vec(&local_videos, &online_video) {
            Some(_) => {}
            None => videos_to_dl.push(online_video.clone()),
        }
    }

    *online_videos = videos_to_dl;
}

pub fn progress(count: &mut i32, total: &i32) {
    println!("[yt-pu] progress {}/{}", &count, &total);
    *count += 1;
}

pub fn remove_local_not_in_playlist(local_videos: &Vec<Video>, online_videos: &Vec<Video>) {
    let mut video_to_remove: Vec<PathBuf> = Vec::new();

    // find all local videos not present in online_videos
    for local_video in local_videos {
        let mut find_match = false;
        match find_filename_in_vec(&online_videos, &local_video) {
            Some(_) => find_match = true,
            None => {}
        }
        if find_match {
            video_to_remove.push(local_video.path.clone())
        }
    }

    println!(
        "[yt-pu] Found {} video in local to remove",
        video_to_remove.len()
    );

    for path in video_to_remove.iter() {
        match trash::delete(path) {
            Ok(_) => println!("[yt-pu] Moved file {:?} to trash", path),
            Err(e) => eprintln!(
                "[yt-pu] Faile to move to trash file {:?} due to error {}",
                path, e
            ),
        }
    }
}
